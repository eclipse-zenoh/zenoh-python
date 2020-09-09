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
use pyo3::create_exception;
use pyo3::prelude::*;
use pyo3::{exceptions, wrap_pyfunction, wrap_pymodule};

mod zenoh_net;
use zenoh_net::*;

#[pymodule]
fn zenoh(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(net))?;
    // force addition of "zenoh.net" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.net'] = net
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_wrapped(wrap_pyfunction!(init_logger))?;
    Ok(())
}

create_exception!(zenoh, ZError, exceptions::Exception);

fn to_pyerr(err: zenoh::ZError) -> PyErr {
    PyErr::new::<ZError, _>(err.to_string())
}

#[pyfunction]
fn init_logger() {
    env_logger::init();
}
