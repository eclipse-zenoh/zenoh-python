//
// Copyright (c) 2017, 2022 ZettaScale Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh team, <zenoh@zettascale.tech>
//
use crate::ToPyErr;
use pyo3::prelude::*;
use zenoh::prelude::{Encoding, Priority, SampleKind};
use zenoh::publication::CongestionControl;
use zenoh::query::{ConsolidationMode, QueryTarget};
use zenoh::subscriber::Reliability;

#[pyclass(subclass)]
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct _Encoding(pub(crate) Encoding);
#[pymethods]
impl _Encoding {
    #[new]
    pub fn new(this: Self) -> Self {
        this
    }
    fn __richcmp__(&self, other: &Self, op: pyo3::pyclass::CompareOp) -> PyResult<bool> {
        match op {
            pyo3::pyclass::CompareOp::Eq => Ok(self == other),
            pyo3::pyclass::CompareOp::Ne => Ok(self != other),
            _ => Err(zenoh_core::zerror!("Encoding does not support comparison").to_pyerr()),
        }
    }
    #[classattr]
    pub const ZENOH_BYTES: Self = Self(Encoding::ZENOH_BYTES);
    #[classattr]
    pub const ZENOH_INT: Self = Self(Encoding::ZENOH_INT);
    #[classattr]
    pub const ZENOH_UINT: Self = Self(Encoding::ZENOH_UINT);
    #[classattr]
    pub const ZENOH_FLOAT: Self = Self(Encoding::ZENOH_FLOAT);
    #[classattr]
    pub const ZENOH_BOOL: Self = Self(Encoding::ZENOH_BOOL);
    #[classattr]
    pub const ZENOH_STRING: Self = Self(Encoding::ZENOH_STRING);
    #[classattr]
    pub const ZENOH_ERROR: Self = Self(Encoding::ZENOH_ERROR);
    #[classattr]
    pub const APPLICATION_OCTET_STREAM: Self = Self(Encoding::APPLICATION_OCTET_STREAM);
    #[classattr]
    pub const TEXT_PLAIN: Self = Self(Encoding::TEXT_PLAIN);
    #[classattr]
    pub const APPLICATION_JSON: Self = Self(Encoding::APPLICATION_JSON);
    #[classattr]
    pub const TEXT_JSON: Self = Self(Encoding::TEXT_JSON);
    #[classattr]
    pub const APPLICATION_CDR: Self = Self(Encoding::APPLICATION_CDR);
    #[classattr]
    pub const APPLICATION_CBOR: Self = Self(Encoding::APPLICATION_CBOR);
    #[classattr]
    pub const APPLICATION_YAML: Self = Self(Encoding::APPLICATION_YAML);
    #[classattr]
    pub const TEXT_YAML: Self = Self(Encoding::TEXT_YAML);
    #[classattr]
    pub const TEXT_JSON5: Self = Self(Encoding::TEXT_JSON5);
    pub const APPLICATION_PYTHON_SERIALIZED_OBJECT: Self =
        Self(Encoding::APPLICATION_PYTHON_SERIALIZED_OBJECT);
    #[classattr]
    pub const APPLICATION_PROTOBUF: Self = Self(Encoding::APPLICATION_PROTOBUF);
    #[classattr]
    pub const APPLICATION_JAVA_SERIALIZED_OBJECT: Self =
        Self(Encoding::APPLICATION_JAVA_SERIALIZED_OBJECT);
    #[classattr]
    pub const APPLICATION_OPENMETRICS_TEXT: Self = Self(Encoding::APPLICATION_OPENMETRICS_TEXT);
    #[classattr]
    pub const IMAGE_PNG: Self = Self(Encoding::IMAGE_PNG);
    #[classattr]
    pub const IMAGE_JPEG: Self = Self(Encoding::IMAGE_JPEG);
    #[classattr]
    pub const IMAGE_GIF: Self = Self(Encoding::IMAGE_GIF);
    #[classattr]
    pub const IMAGE_BMP: Self = Self(Encoding::IMAGE_BMP);
    #[classattr]
    pub const IMAGE_WEBP: Self = Self(Encoding::IMAGE_WEBP);
    #[classattr]
    pub const APPLICATION_XML: Self = Self(Encoding::APPLICATION_XML);
    #[classattr]
    pub const APPLICATION_X_WWW_FORM_URLENCODED: Self =
        Self(Encoding::APPLICATION_X_WWW_FORM_URLENCODED);
    #[classattr]
    pub const TEXT_HTML: Self = Self(Encoding::TEXT_HTML);
    #[classattr]
    pub const TEXT_XML: Self = Self(Encoding::TEXT_XML);
    #[classattr]
    pub const TEXT_CSS: Self = Self(Encoding::TEXT_CSS);
    #[classattr]
    pub const TEXT_JAVASCRIPT: Self = Self(Encoding::TEXT_JAVASCRIPT);
    #[classattr]
    pub const TEXT_MARKDOWN: Self = Self(Encoding::TEXT_MARKDOWN);
    #[classattr]
    pub const TEXT_CSV: Self = Self(Encoding::TEXT_CSV);
    #[classattr]
    pub const APPLICATION_SQL: Self = Self(Encoding::APPLICATION_SQL);
    #[classattr]
    pub const APPLICATION_COAP_PAYLOAD: Self = Self(Encoding::APPLICATION_COAP_PAYLOAD);
    #[classattr]
    pub const APPLICATION_JSON_PATCH_JSON: Self = Self(Encoding::APPLICATION_JSON_PATCH_JSON);
    #[classattr]
    pub const APPLICATION_JSON_SEQ: Self = Self(Encoding::APPLICATION_JSON_SEQ);
    #[classattr]
    pub const APPLICATION_JSONPATH: Self = Self(Encoding::APPLICATION_JSONPATH);
    #[classattr]
    pub const APPLICATION_JWT: Self = Self(Encoding::APPLICATION_JWT);
    #[classattr]
    pub const APPLICATION_MP4: Self = Self(Encoding::APPLICATION_MP4);
    #[classattr]
    pub const APPLICATION_SOAP_XML: Self = Self(Encoding::APPLICATION_SOAP_XML);
    #[classattr]
    pub const APPLICATION_YANG: Self = Self(Encoding::APPLICATION_YANG);
    #[classattr]
    pub const AUDIO_AAC: Self = Self(Encoding::AUDIO_AAC);
    #[classattr]
    pub const AUDIO_FLAC: Self = Self(Encoding::AUDIO_FLAC);
    #[classattr]
    pub const AUDIO_MP4: Self = Self(Encoding::AUDIO_MP4);
    #[classattr]
    pub const AUDIO_OGG: Self = Self(Encoding::AUDIO_OGG);
    #[classattr]
    pub const AUDIO_VORBIS: Self = Self(Encoding::AUDIO_VORBIS);
    #[classattr]
    pub const VIDEO_H261: Self = Self(Encoding::VIDEO_H261);
    #[classattr]
    pub const VIDEO_H263: Self = Self(Encoding::VIDEO_H263);
    #[classattr]
    pub const VIDEO_H264: Self = Self(Encoding::VIDEO_H264);
    #[classattr]
    pub const VIDEO_H265: Self = Self(Encoding::VIDEO_H265);
    #[classattr]
    pub const VIDEO_H266: Self = Self(Encoding::VIDEO_H266);
    #[classattr]
    pub const VIDEO_MP4: Self = Self(Encoding::VIDEO_MP4);
    #[classattr]
    pub const VIDEO_OGG: Self = Self(Encoding::VIDEO_OGG);
    #[classattr]
    pub const VIDEO_RAW: Self = Self(Encoding::VIDEO_RAW);
    #[classattr]
    pub const VIDEO_VP8: Self = Self(Encoding::VIDEO_VP8);
    #[classattr]
    pub const VIDEO_VP9: Self = Self(Encoding::VIDEO_VP9);
    #[staticmethod]
    pub fn from_str(s: String) -> Self {
        Self(s.into())
    }
    pub fn __str__(&self) -> String {
        self.0.to_string()
    }
    pub fn with_schema(&self, suffix: String) -> Self {
        Self(self.0.clone().with_schema(suffix))
    }
}

#[pyclass(subclass)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct _Priority(pub(crate) Priority);
#[pymethods]
impl _Priority {
    #[new]
    pub fn new(this: Self) -> Self {
        this
    }
    fn __richcmp__(&self, other: &Self, op: pyo3::pyclass::CompareOp) -> bool {
        match op {
            pyo3::pyclass::CompareOp::Lt => self < other,
            pyo3::pyclass::CompareOp::Le => self <= other,
            pyo3::pyclass::CompareOp::Eq => self == other,
            pyo3::pyclass::CompareOp::Ne => self != other,
            pyo3::pyclass::CompareOp::Gt => self > other,
            pyo3::pyclass::CompareOp::Ge => self >= other,
        }
    }
    #[classattr]
    pub const REAL_TIME: Self = Self(Priority::RealTime);
    #[classattr]
    pub const INTERACTIVE_HIGH: Self = Self(Priority::InteractiveHigh);
    #[classattr]
    pub const INTERACTIVE_LOW: Self = Self(Priority::InteractiveLow);
    #[classattr]
    pub const DATA_HIGH: Self = Self(Priority::DataHigh);
    #[classattr]
    pub const DATA: Self = Self(Priority::Data);
    #[classattr]
    pub const DATA_LOW: Self = Self(Priority::DataLow);
    #[classattr]
    pub const BACKGROUND: Self = Self(Priority::Background);
    pub fn __str__(&self) -> &'static str {
        match self.0 {
            Priority::RealTime => "REAL_TIME",
            Priority::InteractiveHigh => "INTERACTIVE_HIGH",
            Priority::InteractiveLow => "INTERACTIVE_LOW",
            Priority::DataHigh => "DATA_HIGH",
            Priority::Data => "DATA",
            Priority::DataLow => "DATA_LOW",
            Priority::Background => "BACKGROUND",
        }
    }
}
impl PartialOrd for _Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.0 as u8).partial_cmp(&(other.0 as u8))
    }
}

#[pyclass(subclass)]
#[derive(Clone, PartialEq, Eq, Default)]
pub struct _SampleKind(pub(crate) SampleKind);
#[pymethods]
impl _SampleKind {
    #[new]
    pub fn new(this: Self) -> Self {
        this
    }
    #[classattr]
    pub const PUT: Self = Self(SampleKind::Put);
    #[classattr]
    pub const DELETE: Self = Self(SampleKind::Delete);
    pub fn __str__(&self) -> &'static str {
        match self.0 {
            SampleKind::Put => "PUT",
            SampleKind::Delete => "DELETE",
        }
    }
    fn __richcmp__(&self, other: &Self, op: pyo3::pyclass::CompareOp) -> PyResult<bool> {
        match op {
            pyo3::pyclass::CompareOp::Eq => Ok(self == other),
            pyo3::pyclass::CompareOp::Ne => Ok(self != other),
            _ => Err(zenoh_core::zerror!("SampleKind does not support comparison").to_pyerr()),
        }
    }
}
impl core::fmt::Debug for _SampleKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self.0, f)
    }
}

#[pyclass(subclass)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct _CongestionControl(pub(crate) CongestionControl);
#[pymethods]
impl _CongestionControl {
    #[new]
    pub fn new(this: Self) -> Self {
        this
    }
    fn __richcmp__(&self, other: &Self, op: pyo3::pyclass::CompareOp) -> PyResult<bool> {
        match op {
            pyo3::pyclass::CompareOp::Eq => Ok(self == other),
            pyo3::pyclass::CompareOp::Ne => Ok(self != other),
            _ => {
                Err(zenoh_core::zerror!("CongestionControl does not support comparison").to_pyerr())
            }
        }
    }
    #[classattr]
    pub const BLOCK: Self = Self(CongestionControl::Block);
    #[classattr]
    pub const DROP: Self = Self(CongestionControl::Drop);
    pub fn __str__(&self) -> &'static str {
        match self.0 {
            CongestionControl::Block => "BLOCK",
            CongestionControl::Drop => "DROP",
        }
    }
}

#[pyclass(subclass)]
#[derive(Clone, PartialEq, Eq)]
pub struct _Reliability(pub(crate) Reliability);
#[pymethods]
impl _Reliability {
    #[new]
    pub fn new(this: Self) -> Self {
        this
    }
    fn __richcmp__(&self, other: &Self, op: pyo3::pyclass::CompareOp) -> PyResult<bool> {
        match op {
            pyo3::pyclass::CompareOp::Eq => Ok(self == other),
            pyo3::pyclass::CompareOp::Ne => Ok(self != other),
            _ => Err(zenoh_core::zerror!("Reliability does not support comparison").to_pyerr()),
        }
    }
    #[classattr]
    pub const BEST_EFFORT: Self = Self(Reliability::BestEffort);
    #[classattr]
    pub const RELIABLE: Self = Self(Reliability::Reliable);
    pub fn __str__(&self) -> &'static str {
        match self.0 {
            Reliability::BestEffort => "BEST_EFFORT",
            Reliability::Reliable => "RELIABLE",
        }
    }
}

#[pyclass(subclass)]
#[derive(Clone, PartialEq, Eq)]
pub struct _QueryTarget(pub(crate) QueryTarget);
#[pymethods]
impl _QueryTarget {
    #[new]
    pub fn new(this: Self) -> Self {
        this
    }
    fn __richcmp__(&self, other: &Self, op: pyo3::pyclass::CompareOp) -> PyResult<bool> {
        match op {
            pyo3::pyclass::CompareOp::Eq => Ok(self == other),
            pyo3::pyclass::CompareOp::Ne => Ok(self != other),
            _ => Err(zenoh_core::zerror!("QueryTarget does not support comparison").to_pyerr()),
        }
    }
    #[classattr]
    pub const BEST_MATCHING: Self = Self(QueryTarget::BestMatching);
    #[classattr]
    pub const ALL: Self = Self(QueryTarget::All);
    #[classattr]
    pub const ALL_COMPLETE: Self = Self(QueryTarget::AllComplete);
    pub fn __str__(&self) -> &'static str {
        match self.0 {
            QueryTarget::BestMatching => "BEST_MATCHING",
            QueryTarget::All => "ALL",
            QueryTarget::AllComplete => "ALL_COMPLETE",
            #[cfg(feature = "complete_n")]
            QueryTarget::Complete(_) => "COMPLETE_N",
        }
    }
}

#[pyclass(subclass)]
#[derive(Clone, PartialEq, Eq)]
pub struct _QueryConsolidation(pub(crate) Option<ConsolidationMode>);
#[pymethods]
impl _QueryConsolidation {
    #[new]
    pub fn new(this: Self) -> Self {
        this
    }
    fn __richcmp__(&self, other: &Self, op: pyo3::pyclass::CompareOp) -> PyResult<bool> {
        match op {
            pyo3::pyclass::CompareOp::Eq => Ok(self == other),
            pyo3::pyclass::CompareOp::Ne => Ok(self != other),
            _ => Err(
                zenoh_core::zerror!("QueryConsolidation does not support comparison").to_pyerr(),
            ),
        }
    }
    #[classattr]
    pub const AUTO: Self = Self(None);
    #[classattr]
    pub const NONE: Self = Self(Some(ConsolidationMode::None));
    #[classattr]
    pub const MONOTONIC: Self = Self(Some(ConsolidationMode::Monotonic));
    #[classattr]
    pub const LATEST: Self = Self(Some(ConsolidationMode::Latest));
}
