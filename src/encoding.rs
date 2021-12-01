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
use zenoh::prelude::Encoding as ZEncoding;

// zenoh.encoding (simulate the package as a class, and consts as class attributes)
/// Constants defining the different encoding flags.
#[allow(non_camel_case_types)]
#[pyclass]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Encoding {
    e: ZEncoding,
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
    #[classattr]
    pub fn APP_OCTET_STREAM() -> Self {
        ZEncoding::APP_OCTET_STREAM.into()
    }

    #[classattr]
    pub fn NONE() -> Self {
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
impl std::fmt::Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.e)
    }
}
