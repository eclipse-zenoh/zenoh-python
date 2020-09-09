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
use crate::to_pyerr;
use async_std::task;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod types;
use types::*;
mod session;
use session::*;

// module zenoh.net
#[pymodule]
fn net(_: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<whatami>()?;
    m.add_class::<Config>()?;
    m.add_class::<Session>()?;
    m.add_wrapped(wrap_pyfunction!(open))?;
    Ok(())
}

#[pyfunction]
fn open(config: Config) -> PyResult<Session> {
    let s = task::block_on(zenoh::net::open(config.c, None)).map_err(to_pyerr)?;
    Ok(Session::new(s))
}
