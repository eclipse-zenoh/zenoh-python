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
use zenoh::prelude::{Encoding, KnownEncoding, Priority, SampleKind};
use zenoh::publication::CongestionControl;
use zenoh::query::{ConsolidationMode, QueryTarget};
use zenoh::subscriber::Reliability;

#[pyclass(subclass)]
#[repr(transparent)]
#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub const EMPTY: Self = Self(Encoding::Exact(KnownEncoding::Empty));
    #[classattr]
    pub const APP_OCTET_STREAM: Self = Self(Encoding::Exact(KnownEncoding::AppOctetStream));
    #[classattr]
    pub const APP_CUSTOM: Self = Self(Encoding::Exact(KnownEncoding::AppCustom));
    #[classattr]
    pub const TEXT_PLAIN: Self = Self(Encoding::Exact(KnownEncoding::TextPlain));
    #[classattr]
    pub const APP_PROPERTIES: Self = Self(Encoding::Exact(KnownEncoding::AppProperties));
    #[classattr]
    pub const APP_JSON: Self = Self(Encoding::Exact(KnownEncoding::AppJson));
    #[classattr]
    pub const APP_SQL: Self = Self(Encoding::Exact(KnownEncoding::AppSql));
    #[classattr]
    pub const APP_INTEGER: Self = Self(Encoding::Exact(KnownEncoding::AppInteger));
    #[classattr]
    pub const APP_FLOAT: Self = Self(Encoding::Exact(KnownEncoding::AppFloat));
    #[classattr]
    pub const APP_XML: Self = Self(Encoding::Exact(KnownEncoding::AppXml));
    #[classattr]
    pub const APP_XHTML_XML: Self = Self(Encoding::Exact(KnownEncoding::AppXhtmlXml));
    #[classattr]
    pub const APP_X_WWW_FORM_URLENCODED: Self =
        Self(Encoding::Exact(KnownEncoding::AppXWwwFormUrlencoded));
    #[classattr]
    pub const TEXT_JSON: Self = Self(Encoding::Exact(KnownEncoding::TextJson));
    #[classattr]
    pub const TEXT_HTML: Self = Self(Encoding::Exact(KnownEncoding::TextHtml));
    #[classattr]
    pub const TEXT_XML: Self = Self(Encoding::Exact(KnownEncoding::TextXml));
    #[classattr]
    pub const TEXT_CSS: Self = Self(Encoding::Exact(KnownEncoding::TextCss));
    #[classattr]
    pub const TEXT_CSV: Self = Self(Encoding::Exact(KnownEncoding::TextCsv));
    #[classattr]
    pub const TEXT_JAVASCRIPT: Self = Self(Encoding::Exact(KnownEncoding::TextJavascript));
    #[classattr]
    pub const IMAGE_JPEG: Self = Self(Encoding::Exact(KnownEncoding::ImageJpeg));
    #[classattr]
    pub const IMAGE_PNG: Self = Self(Encoding::Exact(KnownEncoding::ImagePng));
    #[classattr]
    pub const IMAGE_GIF: Self = Self(Encoding::Exact(KnownEncoding::ImageGif));
    #[staticmethod]
    pub fn from_str(s: String) -> Self {
        Self(s.into())
    }
    pub fn __str__(&self) -> String {
        self.0.to_string()
    }
    pub fn append(&mut self, suffix: String) {
        unsafe {
            let mut tmp = std::ptr::read(&self.0);
            tmp = tmp.with_suffix(suffix);
            std::ptr::write(&mut self.0, tmp);
        }
    }
    pub fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

#[pyclass(subclass)]
#[derive(Clone, PartialEq, Eq)]
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
impl std::cmp::PartialOrd for _Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.0 as u8).partial_cmp(&(other.0 as u8))
    }
}

#[pyclass(subclass)]
#[derive(Clone, PartialEq, Eq)]
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

#[pyclass(subclass)]
#[derive(Clone, PartialEq, Eq)]
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
