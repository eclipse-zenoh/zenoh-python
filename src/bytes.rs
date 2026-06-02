//
// Copyright (c) 2024 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use std::{
    any::Any,
    borrow::Cow,
    fmt,
    io::Read,
    os::raw::{c_int, c_void},
    ptr, slice,
    sync::Arc,
};

use pyo3::{
    exceptions::{PyRuntimeError, PyTypeError, PyValueError},
    ffi,
    prelude::*,
    types::{PyByteArray, PyBytes, PyString, PyTuple},
};
use zenoh_buffers::{ZBuf, ZSliceBuffer};

use crate::{
    macros::{downcast_or_new, wrapper},
    utils::{IntoPyResult, MapInto},
};

unsafe extern "C" {
    // `PyObject_AsReadBuffer` is part of the stable ABI. Holding a Python
    // `memoryview` separately gives the acquired exporter resources a clear
    // lifetime even though this legacy API returns only a pointer and length.
    #[link_name = "PyObject_AsReadBuffer"]
    fn py_object_as_read_buffer(
        obj: *mut ffi::PyObject,
        buffer: *mut *const c_void,
        buffer_len: *mut ffi::Py_ssize_t,
    ) -> c_int;
}

struct BorrowedPyBufferSlice {
    _owner: Py<PyAny>,
    ptr: *const u8,
    len: usize,
}

impl BorrowedPyBufferSlice {
    fn new(buffer: &Bound<PyAny>) -> PyResult<Self> {
        let mut ptr = ptr::null();
        let mut len = 0;
        if unsafe { py_object_as_read_buffer(buffer.as_ptr(), &mut ptr, &mut len) } == -1 {
            return Err(PyErr::fetch(buffer.py()));
        }
        if len < 0 {
            Err(PyRuntimeError::new_err(
                "buffer exporter returned a negative length",
            ))
        } else if len > 0 && ptr.is_null() {
            Err(PyRuntimeError::new_err(
                "buffer exporter returned a null pointer for a non-empty segment",
            ))
        } else {
            Ok(Self {
                _owner: buffer.clone().unbind(),
                ptr: ptr.cast(),
                len: len as usize,
            })
        }
    }

    fn as_bytes(&self) -> &[u8] {
        if self.len == 0 {
            &[]
        } else {
            // SAFETY: `_owner` retains the validated `memoryview`, which owns
            // its exporter resources and keeps this contiguous slice valid.
            unsafe { slice::from_raw_parts(self.ptr, self.len) }
        }
    }
}

// SAFETY: `_owner` retains the validated `memoryview` while this pointer may be
// read from another thread.
unsafe impl Send for BorrowedPyBufferSlice {}
// SAFETY: The `copy=False` contract requires callers not to mutate the
// exported memory through another alias while Zenoh may reference it.
unsafe impl Sync for BorrowedPyBufferSlice {}

impl fmt::Debug for BorrowedPyBufferSlice {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BorrowedPyBufferSlice")
            .field("len", &self.len)
            .finish_non_exhaustive()
    }
}

impl ZSliceBuffer for BorrowedPyBufferSlice {
    fn as_slice(&self) -> &[u8] {
        self.as_bytes()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn py_buffer_zbytes(buffer: BorrowedPyBufferSlice) -> zenoh::bytes::ZBytes {
    ZBuf::from(Arc::new(buffer)).into()
}

wrapper!(zenoh::bytes::ZBytes: Clone, Default);
downcast_or_new!(ZBytes);

#[pymethods]
impl ZBytes {
    #[new]
    fn new(obj: Option<&Bound<PyAny>>) -> PyResult<Self> {
        let Some(obj) = obj else {
            return Ok(Self::default());
        };
        if let Ok(bytes) = obj.downcast::<PyByteArray>() {
            Ok(Self(bytes.to_vec().into()))
        } else if let Ok(bytes) = obj.downcast::<PyBytes>() {
            Ok(Self(bytes.as_bytes().into()))
        } else if let Ok(string) = obj.downcast::<PyString>() {
            Ok(Self(string.to_string().into()))
        } else {
            #[cfg(feature = "shared-memory")]
            if let Ok(buf) = obj.downcast_exact::<crate::shm::ZShmMut>() {
                return Ok(Self(buf.borrow_mut().take()?.into()));
            }
            #[cfg(feature = "shared-memory")]
            if let Ok(buf) = obj.downcast_exact::<crate::shm::ZShm>() {
                return Ok(Self(buf.borrow().0.clone().into()));
            }
            Err(PyTypeError::new_err(format!(
                "expected bytes/str type, found '{}'",
                obj.get_type().name().unwrap()
            )))
        }
    }

    fn to_bytes<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        // Not using `ZBytes::to_bytes`
        PyBytes::new_with(py, self.0.len(), |bytes| {
            self.0.reader().read_exact(bytes).into_pyres()
        })
    }

    #[staticmethod]
    #[pyo3(signature = (segments, *, copy = false, require_contiguous = true))]
    fn from_segments(
        segments: &Bound<PyAny>,
        copy: bool,
        require_contiguous: bool,
    ) -> PyResult<Self> {
        let py = segments.py();
        let memoryview = py.import("builtins")?.getattr("memoryview")?;
        let mut writer = zenoh::bytes::ZBytes::writer();
        for (index, segment) in segments.try_iter()?.enumerate() {
            let segment = segment?;
            if !copy {
                let view = memoryview.call1((&segment,)).map_err(|_| {
                    PyRuntimeError::new_err(format!(
                        "zero-copy requires a read-only, C-contiguous, byte-compatible Python \
                         buffer; segment {index} has type '{}'; use copy=True",
                        segment.get_type().name().unwrap()
                    ))
                })?;
                if !view.getattr("readonly")?.extract::<bool>()? {
                    return Err(PyRuntimeError::new_err(format!(
                        "segment {index} is writable; zero-copy requires a read-only buffer; \
                         use copy=True"
                    )));
                }
                if !view.getattr("c_contiguous")?.extract::<bool>()? {
                    return Err(PyRuntimeError::new_err(format!(
                        "segment {index} is not C-contiguous; zero-copy requires one contiguous \
                         byte slice; use copy=True"
                    )));
                }
                if view.getattr("itemsize")?.extract::<usize>()? != 1 {
                    return Err(PyRuntimeError::new_err(format!(
                        "segment {index} has unsupported item format; zero-copy requires a \
                         single-byte buffer; use copy=True"
                    )));
                }
                let buffer = BorrowedPyBufferSlice::new(&view)?;
                writer.append(py_buffer_zbytes(buffer));
                continue;
            }

            let view = memoryview.call1((&segment,)).map_err(|_| {
                PyTypeError::new_err(format!(
                    "segment {index} does not support the Python buffer protocol"
                ))
            })?;
            if view.getattr("itemsize")?.extract::<usize>()? != 1 {
                return Err(PyTypeError::new_err(format!(
                    "segment {index} has unsupported item format; \
                     expected a byte-compatible buffer"
                )));
            }
            if require_contiguous && !view.getattr("c_contiguous")?.extract::<bool>()? {
                return Err(PyTypeError::new_err(format!(
                    "segment {index} is not C-contiguous; use require_contiguous=False"
                )));
            }
            if view.getattr("c_contiguous")?.extract::<bool>()? {
                writer.append(
                    BorrowedPyBufferSlice::new(&view)?
                        .as_bytes()
                        .to_vec()
                        .into(),
                );
            } else {
                let bytes = view.call_method0("tobytes")?;
                writer.append(bytes.downcast::<PyBytes>()?.as_bytes().to_vec().into());
            }
        }
        Ok(Self(writer.finish()))
    }

    fn segments<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        let memoryview = py.import("builtins")?.getattr("memoryview")?;
        let views = self
            .0
            .slices()
            .map(|slice| memoryview.call1((PyBytes::new(py, slice),)))
            .collect::<PyResult<Vec<_>>>()?;
        PyTuple::new(py, views)
    }

    fn memoryviews<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        self.segments(py)
    }

    fn to_string(&self) -> PyResult<Cow<'_, str>> {
        self.0
            .try_to_string()
            .map_err(|_| PyValueError::new_err("not an UTF8 error"))
    }

    #[cfg(feature = "shared-memory")]
    fn as_shm(&self) -> Option<crate::shm::ZShm> {
        self.0.as_shm().map(ToOwned::to_owned).map_into()
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn __bool__(&self) -> bool {
        !self.0.is_empty()
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        self.to_bytes(py)
    }

    fn __str__(&self) -> PyResult<Cow<'_, str>> {
        self.to_string()
    }

    fn __eq__(&self, #[pyo3(from_py_with = Self::from_py)] other: Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        self.__bytes__(py)?.hash()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

wrapper!(zenoh::bytes::Encoding: Clone, Default);
downcast_or_new!(Encoding => Option<String>);

#[pymethods]
impl Encoding {
    #[new]
    fn new(s: Option<String>) -> Self {
        s.map_into().map(Self).unwrap_or_default()
    }

    fn with_schema(&self, schema: String) -> Self {
        Self(self.0.clone().with_schema(schema))
    }

    fn __eq__(&self, #[pyo3(from_py_with = Self::from_py)] other: Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        PyString::new(py, &self.__str__()).hash()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    #[classattr]
    const ZENOH_BYTES: Self = Self(zenoh::bytes::Encoding::ZENOH_BYTES);
    #[classattr]
    const ZENOH_STRING: Self = Self(zenoh::bytes::Encoding::ZENOH_STRING);
    #[classattr]
    const ZENOH_SERIALIZED: Self = Self(zenoh::bytes::Encoding::ZENOH_SERIALIZED);
    #[classattr]
    const APPLICATION_OCTET_STREAM: Self = Self(zenoh::bytes::Encoding::APPLICATION_OCTET_STREAM);
    #[classattr]
    const TEXT_PLAIN: Self = Self(zenoh::bytes::Encoding::TEXT_PLAIN);
    #[classattr]
    const APPLICATION_JSON: Self = Self(zenoh::bytes::Encoding::APPLICATION_JSON);
    #[classattr]
    const TEXT_JSON: Self = Self(zenoh::bytes::Encoding::TEXT_JSON);
    #[classattr]
    const APPLICATION_CDR: Self = Self(zenoh::bytes::Encoding::APPLICATION_CDR);
    #[classattr]
    const APPLICATION_CBOR: Self = Self(zenoh::bytes::Encoding::APPLICATION_CBOR);
    #[classattr]
    const APPLICATION_YAML: Self = Self(zenoh::bytes::Encoding::APPLICATION_YAML);
    #[classattr]
    const TEXT_YAML: Self = Self(zenoh::bytes::Encoding::TEXT_YAML);
    #[classattr]
    const TEXT_JSON5: Self = Self(zenoh::bytes::Encoding::TEXT_JSON5);
    #[classattr]
    const APPLICATION_PYTHON_SERIALIZED_OBJECT: Self =
        Self(zenoh::bytes::Encoding::APPLICATION_PYTHON_SERIALIZED_OBJECT);
    #[classattr]
    const APPLICATION_PROTOBUF: Self = Self(zenoh::bytes::Encoding::APPLICATION_PROTOBUF);
    #[classattr]
    const APPLICATION_JAVA_SERIALIZED_OBJECT: Self =
        Self(zenoh::bytes::Encoding::APPLICATION_JAVA_SERIALIZED_OBJECT);
    #[classattr]
    const APPLICATION_OPENMETRICS_TEXT: Self =
        Self(zenoh::bytes::Encoding::APPLICATION_OPENMETRICS_TEXT);
    #[classattr]
    const IMAGE_PNG: Self = Self(zenoh::bytes::Encoding::IMAGE_PNG);
    #[classattr]
    const IMAGE_JPEG: Self = Self(zenoh::bytes::Encoding::IMAGE_JPEG);
    #[classattr]
    const IMAGE_GIF: Self = Self(zenoh::bytes::Encoding::IMAGE_GIF);
    #[classattr]
    const IMAGE_BMP: Self = Self(zenoh::bytes::Encoding::IMAGE_BMP);
    #[classattr]
    const IMAGE_WEBP: Self = Self(zenoh::bytes::Encoding::IMAGE_WEBP);
    #[classattr]
    const APPLICATION_XML: Self = Self(zenoh::bytes::Encoding::APPLICATION_XML);
    #[classattr]
    const APPLICATION_X_WWW_FORM_URLENCODED: Self =
        Self(zenoh::bytes::Encoding::APPLICATION_X_WWW_FORM_URLENCODED);
    #[classattr]
    const TEXT_HTML: Self = Self(zenoh::bytes::Encoding::TEXT_HTML);
    #[classattr]
    const TEXT_XML: Self = Self(zenoh::bytes::Encoding::TEXT_XML);
    #[classattr]
    const TEXT_CSS: Self = Self(zenoh::bytes::Encoding::TEXT_CSS);
    #[classattr]
    const TEXT_JAVASCRIPT: Self = Self(zenoh::bytes::Encoding::TEXT_JAVASCRIPT);
    #[classattr]
    const TEXT_MARKDOWN: Self = Self(zenoh::bytes::Encoding::TEXT_MARKDOWN);
    #[classattr]
    const TEXT_CSV: Self = Self(zenoh::bytes::Encoding::TEXT_CSV);
    #[classattr]
    const APPLICATION_SQL: Self = Self(zenoh::bytes::Encoding::APPLICATION_SQL);
    #[classattr]
    const APPLICATION_COAP_PAYLOAD: Self = Self(zenoh::bytes::Encoding::APPLICATION_COAP_PAYLOAD);
    #[classattr]
    const APPLICATION_JSON_PATCH_JSON: Self =
        Self(zenoh::bytes::Encoding::APPLICATION_JSON_PATCH_JSON);
    #[classattr]
    const APPLICATION_JSON_SEQ: Self = Self(zenoh::bytes::Encoding::APPLICATION_JSON_SEQ);
    #[classattr]
    const APPLICATION_JSONPATH: Self = Self(zenoh::bytes::Encoding::APPLICATION_JSONPATH);
    #[classattr]
    const APPLICATION_JWT: Self = Self(zenoh::bytes::Encoding::APPLICATION_JWT);
    #[classattr]
    const APPLICATION_MP4: Self = Self(zenoh::bytes::Encoding::APPLICATION_MP4);
    #[classattr]
    const APPLICATION_SOAP_XML: Self = Self(zenoh::bytes::Encoding::APPLICATION_SOAP_XML);
    #[classattr]
    const APPLICATION_YANG: Self = Self(zenoh::bytes::Encoding::APPLICATION_YANG);
    #[classattr]
    const AUDIO_AAC: Self = Self(zenoh::bytes::Encoding::AUDIO_AAC);
    #[classattr]
    const AUDIO_FLAC: Self = Self(zenoh::bytes::Encoding::AUDIO_FLAC);
    #[classattr]
    const AUDIO_MP4: Self = Self(zenoh::bytes::Encoding::AUDIO_MP4);
    #[classattr]
    const AUDIO_OGG: Self = Self(zenoh::bytes::Encoding::AUDIO_OGG);
    #[classattr]
    const AUDIO_VORBIS: Self = Self(zenoh::bytes::Encoding::AUDIO_VORBIS);
    #[classattr]
    const VIDEO_H261: Self = Self(zenoh::bytes::Encoding::VIDEO_H261);
    #[classattr]
    const VIDEO_H263: Self = Self(zenoh::bytes::Encoding::VIDEO_H263);
    #[classattr]
    const VIDEO_H264: Self = Self(zenoh::bytes::Encoding::VIDEO_H264);
    #[classattr]
    const VIDEO_H265: Self = Self(zenoh::bytes::Encoding::VIDEO_H265);
    #[classattr]
    const VIDEO_H266: Self = Self(zenoh::bytes::Encoding::VIDEO_H266);
    #[classattr]
    const VIDEO_MP4: Self = Self(zenoh::bytes::Encoding::VIDEO_MP4);
    #[classattr]
    const VIDEO_OGG: Self = Self(zenoh::bytes::Encoding::VIDEO_OGG);
    #[classattr]
    const VIDEO_RAW: Self = Self(zenoh::bytes::Encoding::VIDEO_RAW);
    #[classattr]
    const VIDEO_VP8: Self = Self(zenoh::bytes::Encoding::VIDEO_VP8);
    #[classattr]
    const VIDEO_VP9: Self = Self(zenoh::bytes::Encoding::VIDEO_VP9);
}
