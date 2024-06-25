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
use pyo3::{prelude::*, types::PyString};

use crate::{
    macros::{downcast_or_new, wrapper},
    utils::MapInto,
};

wrapper!(zenoh::encoding::Encoding: Clone, Default);
downcast_or_new!(Encoding => Option<String>);

#[pymethods]
impl Encoding {
    #[new]
    pub(crate) fn new(s: Option<String>) -> PyResult<Self> {
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
    const ZENOH_BYTES: Self = Self(zenoh::encoding::Encoding::ZENOH_BYTES);
    #[classattr]
    const ZENOH_INT8: Self = Self(zenoh::encoding::Encoding::ZENOH_INT8);
    #[classattr]
    const ZENOH_INT16: Self = Self(zenoh::encoding::Encoding::ZENOH_INT16);
    #[classattr]
    const ZENOH_INT32: Self = Self(zenoh::encoding::Encoding::ZENOH_INT32);
    #[classattr]
    const ZENOH_INT64: Self = Self(zenoh::encoding::Encoding::ZENOH_INT64);
    #[classattr]
    const ZENOH_INT128: Self = Self(zenoh::encoding::Encoding::ZENOH_INT128);
    #[classattr]
    const ZENOH_UINT8: Self = Self(zenoh::encoding::Encoding::ZENOH_UINT8);
    #[classattr]
    const ZENOH_UINT16: Self = Self(zenoh::encoding::Encoding::ZENOH_UINT16);
    #[classattr]
    const ZENOH_UINT32: Self = Self(zenoh::encoding::Encoding::ZENOH_UINT32);
    #[classattr]
    const ZENOH_UINT64: Self = Self(zenoh::encoding::Encoding::ZENOH_UINT64);
    #[classattr]
    const ZENOH_UINT128: Self = Self(zenoh::encoding::Encoding::ZENOH_UINT128);
    #[classattr]
    const ZENOH_FLOAT32: Self = Self(zenoh::encoding::Encoding::ZENOH_FLOAT32);
    #[classattr]
    const ZENOH_FLOAT64: Self = Self(zenoh::encoding::Encoding::ZENOH_FLOAT64);
    #[classattr]
    const ZENOH_BOOL: Self = Self(zenoh::encoding::Encoding::ZENOH_BOOL);
    #[classattr]
    const ZENOH_STRING: Self = Self(zenoh::encoding::Encoding::ZENOH_STRING);
    #[classattr]
    const ZENOH_ERROR: Self = Self(zenoh::encoding::Encoding::ZENOH_ERROR);
    #[classattr]
    const APPLICATION_OCTET_STREAM: Self =
        Self(zenoh::encoding::Encoding::APPLICATION_OCTET_STREAM);
    #[classattr]
    const TEXT_PLAIN: Self = Self(zenoh::encoding::Encoding::TEXT_PLAIN);
    #[classattr]
    const APPLICATION_JSON: Self = Self(zenoh::encoding::Encoding::APPLICATION_JSON);
    #[classattr]
    const TEXT_JSON: Self = Self(zenoh::encoding::Encoding::TEXT_JSON);
    #[classattr]
    const APPLICATION_CDR: Self = Self(zenoh::encoding::Encoding::APPLICATION_CDR);
    #[classattr]
    const APPLICATION_CBOR: Self = Self(zenoh::encoding::Encoding::APPLICATION_CBOR);
    #[classattr]
    const APPLICATION_YAML: Self = Self(zenoh::encoding::Encoding::APPLICATION_YAML);
    #[classattr]
    const TEXT_YAML: Self = Self(zenoh::encoding::Encoding::TEXT_YAML);
    #[classattr]
    const TEXT_JSON5: Self = Self(zenoh::encoding::Encoding::TEXT_JSON5);
    #[classattr]
    const APPLICATION_PYTHON_SERIALIZED_OBJECT: Self =
        Self(zenoh::encoding::Encoding::APPLICATION_PYTHON_SERIALIZED_OBJECT);
    #[classattr]
    const APPLICATION_PROTOBUF: Self = Self(zenoh::encoding::Encoding::APPLICATION_PROTOBUF);
    #[classattr]
    const APPLICATION_JAVA_SERIALIZED_OBJECT: Self =
        Self(zenoh::encoding::Encoding::APPLICATION_JAVA_SERIALIZED_OBJECT);
    #[classattr]
    const APPLICATION_OPENMETRICS_TEXT: Self =
        Self(zenoh::encoding::Encoding::APPLICATION_OPENMETRICS_TEXT);
    #[classattr]
    const IMAGE_PNG: Self = Self(zenoh::encoding::Encoding::IMAGE_PNG);
    #[classattr]
    const IMAGE_JPEG: Self = Self(zenoh::encoding::Encoding::IMAGE_JPEG);
    #[classattr]
    const IMAGE_GIF: Self = Self(zenoh::encoding::Encoding::IMAGE_GIF);
    #[classattr]
    const IMAGE_BMP: Self = Self(zenoh::encoding::Encoding::IMAGE_BMP);
    #[classattr]
    const IMAGE_WEBP: Self = Self(zenoh::encoding::Encoding::IMAGE_WEBP);
    #[classattr]
    const APPLICATION_XML: Self = Self(zenoh::encoding::Encoding::APPLICATION_XML);
    #[classattr]
    const APPLICATION_X_WWW_FORM_URLENCODED: Self =
        Self(zenoh::encoding::Encoding::APPLICATION_X_WWW_FORM_URLENCODED);
    #[classattr]
    const TEXT_HTML: Self = Self(zenoh::encoding::Encoding::TEXT_HTML);
    #[classattr]
    const TEXT_XML: Self = Self(zenoh::encoding::Encoding::TEXT_XML);
    #[classattr]
    const TEXT_CSS: Self = Self(zenoh::encoding::Encoding::TEXT_CSS);
    #[classattr]
    const TEXT_JAVASCRIPT: Self = Self(zenoh::encoding::Encoding::TEXT_JAVASCRIPT);
    #[classattr]
    const TEXT_MARKDOWN: Self = Self(zenoh::encoding::Encoding::TEXT_MARKDOWN);
    #[classattr]
    const TEXT_CSV: Self = Self(zenoh::encoding::Encoding::TEXT_CSV);
    #[classattr]
    const APPLICATION_SQL: Self = Self(zenoh::encoding::Encoding::APPLICATION_SQL);
    #[classattr]
    const APPLICATION_COAP_PAYLOAD: Self =
        Self(zenoh::encoding::Encoding::APPLICATION_COAP_PAYLOAD);
    #[classattr]
    const APPLICATION_JSON_PATCH_JSON: Self =
        Self(zenoh::encoding::Encoding::APPLICATION_JSON_PATCH_JSON);
    #[classattr]
    const APPLICATION_JSON_SEQ: Self = Self(zenoh::encoding::Encoding::APPLICATION_JSON_SEQ);
    #[classattr]
    const APPLICATION_JSONPATH: Self = Self(zenoh::encoding::Encoding::APPLICATION_JSONPATH);
    #[classattr]
    const APPLICATION_JWT: Self = Self(zenoh::encoding::Encoding::APPLICATION_JWT);
    #[classattr]
    const APPLICATION_MP4: Self = Self(zenoh::encoding::Encoding::APPLICATION_MP4);
    #[classattr]
    const APPLICATION_SOAP_XML: Self = Self(zenoh::encoding::Encoding::APPLICATION_SOAP_XML);
    #[classattr]
    const APPLICATION_YANG: Self = Self(zenoh::encoding::Encoding::APPLICATION_YANG);
    #[classattr]
    const AUDIO_AAC: Self = Self(zenoh::encoding::Encoding::AUDIO_AAC);
    #[classattr]
    const AUDIO_FLAC: Self = Self(zenoh::encoding::Encoding::AUDIO_FLAC);
    #[classattr]
    const AUDIO_MP4: Self = Self(zenoh::encoding::Encoding::AUDIO_MP4);
    #[classattr]
    const AUDIO_OGG: Self = Self(zenoh::encoding::Encoding::AUDIO_OGG);
    #[classattr]
    const AUDIO_VORBIS: Self = Self(zenoh::encoding::Encoding::AUDIO_VORBIS);
    #[classattr]
    const VIDEO_H261: Self = Self(zenoh::encoding::Encoding::VIDEO_H261);
    #[classattr]
    const VIDEO_H263: Self = Self(zenoh::encoding::Encoding::VIDEO_H263);
    #[classattr]
    const VIDEO_H264: Self = Self(zenoh::encoding::Encoding::VIDEO_H264);
    #[classattr]
    const VIDEO_H265: Self = Self(zenoh::encoding::Encoding::VIDEO_H265);
    #[classattr]
    const VIDEO_H266: Self = Self(zenoh::encoding::Encoding::VIDEO_H266);
    #[classattr]
    const VIDEO_MP4: Self = Self(zenoh::encoding::Encoding::VIDEO_MP4);
    #[classattr]
    const VIDEO_OGG: Self = Self(zenoh::encoding::Encoding::VIDEO_OGG);
    #[classattr]
    const VIDEO_RAW: Self = Self(zenoh::encoding::Encoding::VIDEO_RAW);
    #[classattr]
    const VIDEO_VP8: Self = Self(zenoh::encoding::Encoding::VIDEO_VP8);
    #[classattr]
    const VIDEO_VP9: Self = Self(zenoh::encoding::Encoding::VIDEO_VP9);
}
