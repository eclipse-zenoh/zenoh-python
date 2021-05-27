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
use zenoh::net::ZInt;

// zenoh.net.data_kind (simulate the package as a class, and consts as class attributes)
/// Constants defining the different data kinds.
#[allow(non_camel_case_types)]
#[pyclass]
pub(crate) struct data_kind {}

#[allow(non_snake_case)]
#[pymethods]
impl data_kind {
    #[classattr]
    fn PUT() -> ZInt {
        zenoh::net::data_kind::PUT
    }

    #[classattr]
    fn PATCH() -> ZInt {
        zenoh::net::data_kind::PATCH
    }

    #[classattr]
    fn DELETE() -> ZInt {
        zenoh::net::data_kind::DELETE
    }

    #[classattr]
    fn DEFAULT() -> ZInt {
        zenoh::net::data_kind::DEFAULT
    }

    #[allow(clippy::wrong_self_convention)]
    #[staticmethod]
    fn to_string(i: ZInt) -> String {
        zenoh::net::data_kind::to_string(i)
    }
}
