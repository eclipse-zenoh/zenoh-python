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
use pyo3::prelude::*;
mod config;
mod enums;
mod keyexpr;
mod session;

pyo3::create_exception!(zenoh, ZError, pyo3::exceptions::PyException);

pub(crate) trait ToPyErr {
    fn to_pyerr(self) -> PyErr;
}
impl<E: std::error::Error> ToPyErr for E {
    fn to_pyerr(self) -> PyErr {
        PyErr::new::<ZError, _>(self.to_string())
    }
}
pub(crate) trait ToPyResult<T> {
    fn to_pyres(self) -> Result<T, PyErr>;
}
impl<T, E: ToPyErr> ToPyResult<T> for Result<T, E> {
    fn to_pyres(self) -> Result<T, PyErr> {
        self.map_err(ToPyErr::to_pyerr)
    }
}

#[pymodule]
fn zenoh(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<config::_Config>()?;
    m.add_class::<keyexpr::_KeyExpr>()?;
    m.add_class::<session::_Session>()?;
    m.add_class::<enums::_Encoding>()?;
    m.add_class::<enums::_Priority>()?;
    m.add_class::<enums::_SampleKind>()?;
    m.add_class::<enums::_CongestionControl>()?;
    m.add_wrapped(wrap_pyfunction!(init_logger))?;
    Ok(())
}

/// Initialize the logger used by the Rust implementation of this API.
///
/// Once initialized, you can configure the logs displayed by the API using the ``RUST_LOG`` environment variable.
/// For instance, start python with the *debug* logs available::
///
///    $ RUST_LOG=debug python
///
/// More details on the RUST_LOG configuration on https://docs.rs/env_logger/latest/env_logger
///
#[pyfunction]
fn init_logger() {
    env_logger::init();
}

pub(crate) trait PyAnyToValue {
    fn to_value(self) -> PyResult<zenoh::prelude::Value>;
}
impl PyAnyToValue for &PyAny {
    fn to_value(self) -> PyResult<zenoh::prelude::Value> {
        todo!()
    }
}
