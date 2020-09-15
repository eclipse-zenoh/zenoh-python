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
fn net(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<properties>()?;
    // force addition of "zenoh.net.properties" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.net.properties'] = properties
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<whatami>()?;
    // force addition of "zenoh.net.whatami" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.net.whatami'] = whatami
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<queryable>()?;
    // force addition of "zenoh.net.queryable" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.net.queryable'] = queryable
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<resource_name>()?;
    // force addition of "zenoh.net.resource_name" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.net.resource_name'] = resource_name
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<Config>()?;
    m.add_class::<ResKey>()?;
    m.add_class::<PeerId>()?;
    m.add_class::<Timestamp>()?;
    m.add_class::<DataInfo>()?;
    m.add_class::<Sample>()?;
    m.add_class::<Reliability>()?;
    m.add_class::<SubMode>()?;
    m.add_class::<Period>()?;
    m.add_class::<SubInfo>()?;
    m.add_class::<Publisher>()?;
    m.add_class::<Subscriber>()?;
    m.add_class::<Query>()?;
    m.add_class::<Queryable>()?;
    m.add_class::<Target>()?;
    m.add_class::<QueryTarget>()?;
    m.add_class::<QueryConsolidation>()?;
    m.add_class::<Reply>()?;
    m.add_class::<Session>()?;
    m.add_wrapped(wrap_pyfunction!(open))?;
    Ok(())
}

#[pyfunction]
fn open(config: Config) -> PyResult<Session> {
    let s = task::block_on(zenoh::net::open(config.c, None)).map_err(to_pyerr)?;
    Ok(Session::new(s))
}
