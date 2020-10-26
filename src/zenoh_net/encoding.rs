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
use pyo3::exceptions;
use pyo3::prelude::*;
use zenoh::net::ZInt;

// zenoh.net.encoding (simulate the package as a class, and consts as class attributes)
/// Constants defining the different encoding flags.
#[allow(non_camel_case_types)]
#[pyclass]
pub(crate) struct encoding {}

#[allow(non_snake_case)]
#[pymethods]
impl encoding {
    #[classattr]
    fn APP_OCTET_STREAM() -> ZInt {
        zenoh::net::encoding::APP_OCTET_STREAM
    }

    #[classattr]
    fn NONE() -> ZInt {
        zenoh::net::encoding::NONE
    }

    #[classattr]
    fn APP_CUSTOM() -> ZInt {
        zenoh::net::encoding::APP_CUSTOM
    }

    #[classattr]
    fn TEXT_PLAIN() -> ZInt {
        zenoh::net::encoding::TEXT_PLAIN
    }

    #[classattr]
    fn STRING() -> ZInt {
        zenoh::net::encoding::STRING
    }

    #[classattr]
    fn APP_PROPERTIES() -> ZInt {
        zenoh::net::encoding::APP_PROPERTIES
    }
    #[classattr]
    fn APP_JSON() -> ZInt {
        zenoh::net::encoding::APP_JSON
    }
    #[classattr]
    fn APP_SQL() -> ZInt {
        zenoh::net::encoding::APP_SQL
    }
    #[classattr]
    fn APP_INTEGER() -> ZInt {
        zenoh::net::encoding::APP_INTEGER
    }
    #[classattr]
    fn APP_FLOAT() -> ZInt {
        zenoh::net::encoding::APP_FLOAT
    }
    #[classattr]
    fn APP_XML() -> ZInt {
        zenoh::net::encoding::APP_XML
    }
    #[classattr]
    fn APP_XHTML_XML() -> ZInt {
        zenoh::net::encoding::APP_XHTML_XML
    }
    #[classattr]
    fn APP_X_WWW_FORM_URLENCODED() -> ZInt {
        zenoh::net::encoding::APP_X_WWW_FORM_URLENCODED
    }
    #[classattr]
    fn TEXT_JSON() -> ZInt {
        zenoh::net::encoding::TEXT_JSON
    }
    #[classattr]
    fn TEXT_HTML() -> ZInt {
        zenoh::net::encoding::TEXT_HTML
    }
    #[classattr]
    fn TEXT_XML() -> ZInt {
        zenoh::net::encoding::TEXT_XML
    }
    #[classattr]
    fn TEXT_CSS() -> ZInt {
        zenoh::net::encoding::TEXT_CSS
    }
    #[classattr]
    fn TEXT_CSV() -> ZInt {
        zenoh::net::encoding::TEXT_CSV
    }
    #[classattr]
    fn TEXT_JAVASCRIPT() -> ZInt {
        zenoh::net::encoding::TEXT_JAVASCRIPT
    }
    #[classattr]
    fn IMG_JPG() -> ZInt {
        zenoh::net::encoding::IMG_JPG
    }
    #[classattr]
    fn IMG_PNG() -> ZInt {
        zenoh::net::encoding::IMG_PNG
    }
    #[classattr]
    fn IMG_GIF() -> ZInt {
        zenoh::net::encoding::IMG_GIF
    }
    #[classattr]
    fn DEFAULT() -> ZInt {
        zenoh::net::encoding::DEFAULT
    }

    #[allow(clippy::wrong_self_convention)]
    #[staticmethod]
    fn to_string(i: ZInt) -> String {
        zenoh::net::encoding::to_string(i)
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<ZInt> {
        zenoh::net::encoding::from_str(s)
            .map_err(|e| PyErr::new::<exceptions::PyValueError, _>(e.to_string()))
    }
}
