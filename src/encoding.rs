//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
use zenoh::prelude::{Encoding as ZEncoding, KnownEncoding as ZKnownEncoding};

/// An encoding known by zenoh and which maps to an integer for wire-efficiency.
#[pyclass]
#[derive(Clone)]
pub struct KnownEncoding {
    inner: ZKnownEncoding,
}

#[allow(non_snake_case)]
#[pymethods]
impl KnownEncoding {
    #[classattr]
    fn Empty() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::Empty,
        }
    }

    #[classattr]
    fn AppOctetStream() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::AppOctetStream,
        }
    }

    #[classattr]
    fn AppCustom() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::AppCustom,
        }
    }

    #[classattr]
    fn TextPlain() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::TextPlain,
        }
    }

    #[classattr]
    fn AppProperties() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::AppProperties,
        }
    }

    #[classattr]
    fn AppJson() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::AppJson,
        }
    }

    #[classattr]
    fn AppSql() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::AppSql,
        }
    }

    #[classattr]
    fn AppInteger() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::AppInteger,
        }
    }

    #[classattr]
    fn AppFloat() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::AppFloat,
        }
    }

    #[classattr]
    fn AppXml() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::AppXml,
        }
    }

    #[classattr]
    fn AppXhtmlXml() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::AppXhtmlXml,
        }
    }

    #[classattr]
    fn AppXWwwFormUrlencoded() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::AppXWwwFormUrlencoded,
        }
    }

    #[classattr]
    fn TextJson() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::TextJson,
        }
    }

    #[classattr]
    fn TextHtml() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::TextHtml,
        }
    }

    #[classattr]
    fn TextXml() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::TextXml,
        }
    }

    #[classattr]
    fn TextCss() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::TextCss,
        }
    }

    #[classattr]
    fn TextCsv() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::TextCsv,
        }
    }

    #[classattr]
    fn TextJavascript() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::TextJavascript,
        }
    }

    #[classattr]
    fn ImageJpeg() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::ImageJpeg,
        }
    }

    #[classattr]
    fn ImagePng() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::ImagePng,
        }
    }

    #[classattr]
    fn ImageGif() -> Self {
        KnownEncoding {
            inner: ZKnownEncoding::ImageGif,
        }
    }
}
impl From<ZKnownEncoding> for KnownEncoding {
    fn from(e: ZKnownEncoding) -> Self {
        KnownEncoding { inner: e }
    }
}
impl From<KnownEncoding> for ZKnownEncoding {
    fn from(e: KnownEncoding) -> Self {
        e.inner
    }
}

// zenoh.encoding (simulate the package as a class, and consts as class attributes)
/// A zenoh encoding is a HTTP Mime type represented, for wire efficiency,
/// as an integer prefix (that maps to a string) and a string suffix.
#[allow(non_camel_case_types)]
#[pyclass]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Encoding {
    pub(crate) e: ZEncoding,
}
impl From<ZEncoding> for Encoding {
    fn from(e: ZEncoding) -> Self {
        Encoding { e }
    }
}
impl From<Encoding> for ZEncoding {
    fn from(e: Encoding) -> Self {
        e.e
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl Encoding {
    #[new]
    fn new(prefix: KnownEncoding, suffix: Option<String>) -> Self {
        if let Some(suffix) = suffix {
            Encoding {
                e: ZEncoding::WithSuffix(prefix.inner, suffix.into()),
            }
        } else {
            Encoding {
                e: ZEncoding::Exact(prefix.inner),
            }
        }
    }

    /// the Encoding's prefix.
    ///
    /// :type: :class:`KnownEncoding`
    #[getter]
    fn prefix(&self) -> PyResult<KnownEncoding> {
        Ok((*self.e.prefix()).into())
    }

    /// the Encoding's suffix.
    ///
    /// :type: **str**
    #[getter]
    fn get_suffix(&self) -> PyResult<&str> {
        Ok(self.e.suffix())
    }

    /// Returns a copy of this Encoding, but changing its suffix.
    ///
    /// :param suffix: The new suffix
    /// :type suffix: **str**
    /// :rtype: :class:`Encoding`
    fn with_suffix(&self, suffix: String) -> Self {
        self.e.clone().with_suffix(suffix).into()
    }

    #[classattr]
    pub fn EMPTY() -> Self {
        ZEncoding::EMPTY.into()
    }

    #[classattr]
    pub fn APP_OCTET_STREAM() -> Self {
        ZEncoding::APP_OCTET_STREAM.into()
    }

    #[classattr]
    pub fn APP_CUSTOM() -> Self {
        ZEncoding::APP_CUSTOM.into()
    }

    #[classattr]
    pub fn TEXT_PLAIN() -> Self {
        ZEncoding::TEXT_PLAIN.into()
    }

    #[classattr]
    pub fn STRING() -> Self {
        ZEncoding::STRING.into()
    }

    #[classattr]
    pub fn APP_PROPERTIES() -> Self {
        ZEncoding::APP_PROPERTIES.into()
    }
    #[classattr]
    pub fn APP_JSON() -> Self {
        ZEncoding::APP_JSON.into()
    }
    #[classattr]
    pub fn APP_SQL() -> Self {
        ZEncoding::APP_SQL.into()
    }
    #[classattr]
    pub fn APP_INTEGER() -> Self {
        ZEncoding::APP_INTEGER.into()
    }
    #[classattr]
    pub fn APP_FLOAT() -> Self {
        ZEncoding::APP_FLOAT.into()
    }
    #[classattr]
    pub fn APP_XML() -> Self {
        ZEncoding::APP_XML.into()
    }
    #[classattr]
    pub fn APP_XHTML_XML() -> Self {
        ZEncoding::APP_XHTML_XML.into()
    }
    #[classattr]
    pub fn APP_X_WWW_FORM_URLENCODED() -> Self {
        ZEncoding::APP_X_WWW_FORM_URLENCODED.into()
    }
    #[classattr]
    pub fn TEXT_JSON() -> Self {
        ZEncoding::TEXT_JSON.into()
    }
    #[classattr]
    pub fn TEXT_HTML() -> Self {
        ZEncoding::TEXT_HTML.into()
    }
    #[classattr]
    pub fn TEXT_XML() -> Self {
        ZEncoding::TEXT_XML.into()
    }
    #[classattr]
    pub fn TEXT_CSS() -> Self {
        ZEncoding::TEXT_CSS.into()
    }
    #[classattr]
    pub fn TEXT_CSV() -> Self {
        ZEncoding::TEXT_CSV.into()
    }
    #[classattr]
    pub fn TEXT_JAVASCRIPT() -> Self {
        ZEncoding::TEXT_JAVASCRIPT.into()
    }
    #[classattr]
    pub fn IMG_JPG() -> Self {
        ZEncoding::IMG_JPG.into()
    }
    #[classattr]
    pub fn IMG_PNG() -> Self {
        ZEncoding::IMG_PNG.into()
    }
    #[classattr]
    pub fn IMG_GIF() -> Self {
        ZEncoding::IMG_GIF.into()
    }
    #[classattr]
    pub fn DEFAULT() -> Self {
        Self::default()
    }
    #[staticmethod]
    fn from_str(s: String) -> Self {
        ZEncoding::from(s).into()
    }
}
#[pyproto]
impl PyObjectProtocol for Encoding {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.e.to_string())
    }
}
impl std::fmt::Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.e)
    }
}
