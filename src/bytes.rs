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
use std::borrow::Cow;

use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    sync::GILOnceCell,
    types::{
        PyBool, PyBytes, PyCFunction, PyDict, PyFloat, PyInt, PyList, PyString, PyTuple, PyType,
    },
    PyTypeInfo,
};
use zenoh::internal::buffers::{SplitBuffer, ZBuf};

use crate::{
    macros::{downcast_or_new, import, try_import, wrapper},
    utils::{try_process, IntoPyResult, IntoPython, MapInto},
};

fn serializers(py: Python) -> &'static Py<PyDict> {
    static SERIALIZERS: GILOnceCell<Py<PyDict>> = GILOnceCell::new();
    SERIALIZERS.get_or_init(py, || PyDict::new_bound(py).unbind())
}
fn deserializers(py: Python) -> &'static Py<PyDict> {
    static DESERIALIZERS: GILOnceCell<Py<PyDict>> = GILOnceCell::new();
    DESERIALIZERS.get_or_init(py, || PyDict::new_bound(py).unbind())
}

fn get_type<'py>(func: &Bound<'py, PyAny>, name: impl ToPyObject) -> PyResult<Bound<'py, PyType>> {
    let py = func.py();
    Ok(import!(py, typing.get_type_hints)
        .call1((func,))?
        .downcast::<PyDict>()?
        .get_item(name)?
        .unwrap_or_else(|| py.None().into_bound(py))
        .downcast_into::<PyType>()?)
}

fn get_first_parameter<'py>(func: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyString>> {
    let py = func.py();
    Ok(import!(py, inspect.signature)
        .call1((func,))?
        .getattr("parameters")?
        .iter()?
        .next()
        .unwrap_or_else(|| Ok(py.None().into_bound(py)))?
        .downcast_into::<PyString>()?)
}

#[pyfunction]
#[pyo3(signature = (func = None, /, *, target = None))]
pub(crate) fn serializer(
    py: Python,
    func: Option<&Bound<PyAny>>,
    target: Option<&Bound<PyAny>>,
) -> PyResult<PyObject> {
    match (func, target) {
        (Some(func), Some(target)) => {
            serializers(py).bind(py).set_item(target, func)?;
            Ok(py.None())
        }
        (Some(func), None) => {
            match get_first_parameter(func).and_then(|param| get_type(func, param)) {
                Ok(target) => serializer(py, Some(func), Some(&target)),
                _ => Err(PyValueError::new_err(
                    "Cannot extract target from serializer signature",
                )),
            }
        }
        (None, Some(target)) => {
            let target = target.clone().unbind();
            let closure = PyCFunction::new_closure_bound(py, None, None, move |args, _| {
                let Ok(func) = args.get_item(0) else {
                    return Err(PyTypeError::new_err("expected one positional argument"));
                };
                serializer(args.py(), Some(&func), Some(target.bind(args.py())))
            })?;
            Ok(closure.into_any().unbind())
        }
        (None, None) => Err(PyTypeError::new_err("missing 'func' or 'target' parameter")),
    }
}

#[pyfunction]
#[pyo3(signature = (func = None, /, *, target = None))]
pub(crate) fn deserializer(
    py: Python,
    func: Option<&Bound<PyAny>>,
    target: Option<&Bound<PyAny>>,
) -> PyResult<PyObject> {
    match (func, target) {
        (Some(func), Some(target)) => {
            deserializers(py).bind(py).set_item(target, func)?;
            Ok(py.None())
        }
        (Some(func), None) => match get_type(func, "return") {
            Ok(target) => deserializer(py, Some(func), Some(&target)),
            _ => Err(PyValueError::new_err(
                "Cannot extract target from deserializer signature",
            )),
        },
        (None, Some(target)) => {
            let target = target.clone().unbind();
            let closure = PyCFunction::new_closure_bound(py, None, None, move |args, _| {
                let Ok(func) = args.get_item(0) else {
                    return Err(PyTypeError::new_err("expected one positional argument"));
                };
                deserializer(args.py(), Some(&func), Some(target.bind(args.py())))
            })?;
            Ok(closure.into_any().unbind())
        }
        (None, None) => Err(PyTypeError::new_err("missing 'func' or 'target' parameter")),
    }
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
        if let Ok(obj) = Self::extract_bound(obj) {
            return Ok(obj);
        }
        let py = obj.py();
        Ok(Self(if let Ok(b) = obj.downcast::<PyBytes>() {
            zenoh::bytes::ZBytes::new(b.as_bytes().to_vec())
        } else if let Ok(s) = String::extract_bound(obj) {
            zenoh::bytes::ZBytes::serialize(s)
        } else if let Ok(i) = i128::extract_bound(obj) {
            zenoh::bytes::ZBytes::serialize(i)
        } else if let Ok(f) = f64::extract_bound(obj) {
            zenoh::bytes::ZBytes::serialize(f)
        } else if let Ok(b) = bool::extract_bound(obj) {
            zenoh::bytes::ZBytes::serialize(b)
        } else if let Ok(list) = obj.downcast::<PyList>() {
            try_process(
                list.iter()
                    .map(|elt| PyResult::Ok(Self::new(Some(&elt))?.0)),
                |iter| iter.collect(),
            )?
        } else if let Ok(dict) = obj.downcast::<PyDict>() {
            try_process(
                dict.iter()
                    .map(|(k, v)| PyResult::Ok((Self::new(Some(&k))?.0, Self::new(Some(&v))?.0))),
                |iter| iter.collect(),
            )?
        } else if let Ok(tuple) = obj.downcast::<PyTuple>() {
            if tuple.len() != 2 {
                return Err(PyValueError::new_err(
                    "only two-elements tuple are supported",
                ));
            }
            zenoh::bytes::ZBytes::serialize((
                Self::new(Some(&tuple.get_item(0)?))?,
                Self::new(Some(&tuple.get_item(1)?))?,
            ))
        } else if let Ok(Some(ser)) = serializers(py).bind(py).get_item(obj.get_type()) {
            return match ZBytes::extract_bound(&ser.call1((obj,))?) {
                Ok(b) => Ok(b),
                _ => Err(PyTypeError::new_err(format!(
                    "serializer {} didn't return ZBytes",
                    ser.repr()?
                ))),
            };
        } else {
            return Err(PyValueError::new_err(
                format!("no serializer registered for type {type}", type = obj.get_type().name()?),
            ));
        }))
    }

    fn deserialize(this: PyRef<Self>, tp: &Bound<PyAny>) -> PyResult<PyObject> {
        let py = tp.py();
        Ok(if tp.eq(PyBytes::type_object_bound(py))? {
            this.__bytes__(py).into_any().unbind()
        } else if tp.eq(PyString::type_object_bound(py))? {
            this.0.deserialize::<Cow<str>>().into_pyres()?.into_py(py)
        } else if tp.eq(PyInt::type_object_bound(py))? {
            this.0.deserialize::<i128>().into_pyres()?.into_py(py)
        } else if tp.eq(PyFloat::type_object_bound(py))? {
            this.0.deserialize::<f64>().into_pyres()?.into_py(py)
        } else if tp.eq(PyBool::type_object_bound(py))? {
            this.0.deserialize::<bool>().into_pyres()?.into_py(py)
        } else if tp.eq(PyList::type_object_bound(py))? {
            let list = PyList::empty_bound(py);
            for elt in this.0.iter::<zenoh::bytes::ZBytes>() {
                list.append(Self(elt.into_pyres()?).into_py(py))?;
            }
            list.into_py(py)
        } else if tp.eq(PyDict::type_object_bound(py))? {
            let dict = PyDict::new_bound(py);
            for kv in this
                .0
                .iter::<(zenoh::bytes::ZBytes, zenoh::bytes::ZBytes)>()
            {
                let (k, v) = kv.into_pyres()?;
                dict.set_item(k.into_pyobject(py), v.into_pyobject(py))?;
            }
            dict.into_py(py)
        } else if try_import!(py, types.GenericAlias)
            .is_ok_and(|alias| tp.is_instance(alias).unwrap_or(false))
        {
            let origin = import!(py, typing.get_origin).call1((tp,))?;
            let args = import!(py, typing.get_args)
                .call1((tp,))?
                .downcast_into::<PyTuple>()?;
            let deserialize =
                |bytes, tp| Self::deserialize(Py::new(py, Self(bytes)).unwrap().borrow(py), tp);
            if origin.eq(PyList::type_object_bound(py))? {
                let tp = args.get_item(0)?;
                let list = PyList::empty_bound(py);
                for elt in this.0.iter::<zenoh::bytes::ZBytes>() {
                    list.append(deserialize(elt.into_pyres()?, &tp)?)?;
                }
                list.into_py(py)
            } else if origin.eq(PyTuple::type_object_bound(py))?
                && args.len() == 2
                && args.get_item(1).is_ok_and(|item| !item.is(&py.Ellipsis()))
            {
                let tp_k = args.get_item(0)?;
                let tp_v = args.get_item(1)?;
                let (k, v): (zenoh::bytes::ZBytes, zenoh::bytes::ZBytes) =
                    this.0.deserialize().into_pyres()?;
                PyTuple::new_bound(py, [deserialize(k, &tp_k)?, deserialize(v, &tp_v)?]).into_py(py)
            } else if origin.eq(PyDict::type_object_bound(py))? {
                let tp_k = args.get_item(0)?;
                let tp_v = args.get_item(1)?;
                let dict = PyDict::new_bound(py);
                for kv in this
                    .0
                    .iter::<(zenoh::bytes::ZBytes, zenoh::bytes::ZBytes)>()
                {
                    let (k, v) = kv.into_pyres()?;
                    dict.set_item(deserialize(k, &tp_k)?, deserialize(v, &tp_v)?)?;
                }
                dict.into_py(py)
            } else {
                return Err(PyValueError::new_err(
                    "only list[Any], dict[Any, Any] or tuple[Any, Any] are supported as generic type",
                ));
            }
        } else if tp.eq(Self::type_object_bound(py))? {
            this.into_py(py)
        } else if let Ok(Some(de)) = deserializers(py).bind(py).get_item(tp) {
            de.call1((this,))?.unbind()
        } else if let Ok(tp) = tp.downcast::<PyType>() {
            return Err(PyValueError::new_err(
                format!("no deserializer registered for type {type}", type = tp.name()?),
            ));
        } else {
            return Err(PyTypeError::new_err(
                format!("expected a type, found {type}", type = tp.get_type().name()?),
            ));
        })
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn __bool__(&self) -> bool {
        !self.0.is_empty()
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound_with(py, self.0.len(), |mut bytes| {
            for slice in ZBuf::from(&self.0).slices() {
                let len = slice.len();
                bytes[..len].copy_from_slice(slice);
                bytes = &mut bytes[len..];
            }
            Ok(())
        })
        .unwrap()
    }

    fn __eq__(&self, other: &Bound<PyAny>) -> PyResult<bool> {
        Ok(self.0 == Self::from_py(other)?.0)
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        self.__bytes__(py).hash()
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
    fn new(s: Option<String>) -> PyResult<Self> {
        Ok(s.map_into().map(Self).unwrap_or_default())
    }

    fn with_schema(&self, schema: String) -> Self {
        Self(self.0.clone().with_schema(schema))
    }

    // Cannot use `#[pyo3(from_py_with = "...")]`, see https://github.com/PyO3/pyo3/issues/4113
    fn __eq__(&self, other: &Bound<PyAny>) -> PyResult<bool> {
        Ok(self.0 == Self::from_py(other)?.0)
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        PyString::new_bound(py, &self.__str__()).hash()
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
    const ZENOH_INT8: Self = Self(zenoh::bytes::Encoding::ZENOH_INT8);
    #[classattr]
    const ZENOH_INT16: Self = Self(zenoh::bytes::Encoding::ZENOH_INT16);
    #[classattr]
    const ZENOH_INT32: Self = Self(zenoh::bytes::Encoding::ZENOH_INT32);
    #[classattr]
    const ZENOH_INT64: Self = Self(zenoh::bytes::Encoding::ZENOH_INT64);
    #[classattr]
    const ZENOH_INT128: Self = Self(zenoh::bytes::Encoding::ZENOH_INT128);
    #[classattr]
    const ZENOH_UINT8: Self = Self(zenoh::bytes::Encoding::ZENOH_UINT8);
    #[classattr]
    const ZENOH_UINT16: Self = Self(zenoh::bytes::Encoding::ZENOH_UINT16);
    #[classattr]
    const ZENOH_UINT32: Self = Self(zenoh::bytes::Encoding::ZENOH_UINT32);
    #[classattr]
    const ZENOH_UINT64: Self = Self(zenoh::bytes::Encoding::ZENOH_UINT64);
    #[classattr]
    const ZENOH_UINT128: Self = Self(zenoh::bytes::Encoding::ZENOH_UINT128);
    #[classattr]
    const ZENOH_FLOAT32: Self = Self(zenoh::bytes::Encoding::ZENOH_FLOAT32);
    #[classattr]
    const ZENOH_FLOAT64: Self = Self(zenoh::bytes::Encoding::ZENOH_FLOAT64);
    #[classattr]
    const ZENOH_BOOL: Self = Self(zenoh::bytes::Encoding::ZENOH_BOOL);
    #[classattr]
    const ZENOH_STRING: Self = Self(zenoh::bytes::Encoding::ZENOH_STRING);
    #[classattr]
    const ZENOH_ERROR: Self = Self(zenoh::bytes::Encoding::ZENOH_ERROR);
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
