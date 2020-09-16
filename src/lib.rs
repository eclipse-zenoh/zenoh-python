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
#![recursion_limit = "256"]

use async_std::task;
use pyo3::create_exception;
use pyo3::prelude::*;
use pyo3::{exceptions, wrap_pyfunction, wrap_pymodule};
use std::collections::HashMap;
use std::convert::TryFrom;

mod zenoh_net;
use zenoh_net::types::*;
use zenoh_net::*;

mod types;
use types::*;
mod workspace;
use workspace::*;

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

    m.add_class::<Zenoh>()?;
    m.add_class::<Workspace>()?;
    m.add_class::<Value>()?;

    Ok(())
}

create_exception!(zenoh, ZError, exceptions::PyException);

fn to_pyerr(err: zenoh::ZError) -> PyErr {
    PyErr::new::<ZError, _>(err.to_string())
}

#[pyfunction]
fn init_logger() {
    env_logger::init();
}

#[pyclass]
pub(crate) struct Zenoh {
    z: Option<zenoh::Zenoh>,
}

#[pymethods]
impl Zenoh {
    #[new]
    fn new(config: Config, properties: Option<HashMap<String, String>>) -> PyResult<Zenoh> {
        let props: Option<zenoh::Properties> = properties.map(|p| p.into());
        let z = task::block_on(zenoh::Zenoh::new(config.c, props)).map_err(to_pyerr)?;
        Ok(Zenoh { z: Some(z) })
    }

    fn close(&mut self) -> PyResult<()> {
        let z = self.take()?;
        task::block_on(z.close()).map_err(to_pyerr)
    }

    fn workspace(&self, prefix: Option<String>) -> PyResult<Workspace> {
        let p = prefix.map(|s| path_of_string(s)).transpose()?;
        let z = self.as_ref()?;
        let workspace = task::block_on(z.workspace(p)).map_err(to_pyerr)?;

        // Note: this is a workaround for pyo3 not supporting lifetime in PyClass. See https://github.com/PyO3/pyo3/issues/502.
        // We extend zenoh::Workspace's lifetime to 'static to be wrapped in Publisher PyClass
        let w = unsafe {
            std::mem::transmute::<zenoh::Workspace<'_>, zenoh::Workspace<'static>>(workspace)
        };

        Ok(Workspace { w })
    }
}

impl Zenoh {
    #[inline]
    fn as_ref(&self) -> PyResult<&zenoh::Zenoh> {
        self.z
            .as_ref()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh was closed"))
    }

    #[inline]
    fn take(&mut self) -> PyResult<zenoh::Zenoh> {
        self.z
            .take()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh was closed"))
    }
}
