use pyo3::prelude::*;

use zenoh::prelude::{Encoding, KnownEncoding, Priority, SampleKind};
use zenoh::publication::CongestionControl;

#[pyclass(subclass)]
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct _Encoding(pub(crate) Encoding);
#[pymethods]
impl _Encoding {
    #[new]
    pub fn new(this: Self) -> Self {
        this
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
    #[classattr]
    pub const BLOCK: Self = Self(CongestionControl::Block);
    #[classattr]
    pub const DROP: Self = Self(CongestionControl::Drop);
}

#[test]
fn variants_exhaustivity() {
    match _Priority::REAL_TIME {
        _Priority::REAL_TIME
        | _Priority::INTERACTIVE_HIGH
        | _Priority::INTERACTIVE_LOW
        | _Priority::DATA_HIGH
        | _Priority::DATA
        | _Priority::DATA_LOW
        | _Priority::BACKGROUND => {}
    }
    match _SampleKind::PUT {
        _SampleKind::PUT | _SampleKind::DELETE => {}
    }
    match _CongestionControl::BLOCK {
        _CongestionControl::BLOCK | _CongestionControl::DROP => {}
    }
}
