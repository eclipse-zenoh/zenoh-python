//
// Copyright (c) 2024 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use std::sync::Weak;

use pyo3::{prelude::*, types::PyList};
use zenoh::prelude::*;

use crate::{config::ZenohId, macros::zerror, utils::IntoPython};

#[pyclass]
pub(crate) struct SessionInfo(pub(crate) Weak<zenoh::Session>);

impl SessionInfo {
    fn get_info(&self) -> PyResult<zenoh::info::SessionInfo<'static>> {
        Ok(self
            .0
            .upgrade()
            .ok_or_else(|| zerror!("Closed session"))?
            .info())
    }
}

#[pymethods]
impl SessionInfo {
    fn zid(&self, py: Python) -> PyResult<ZenohId> {
        let info = self.get_info()?;
        Ok(py.allow_threads(|| info.zid().wait()).into())
    }

    fn routers_zid<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let info = self.get_info()?;
        let list = PyList::empty_bound(py);
        for zid in py.allow_threads(|| info.routers_zid().wait()) {
            list.append(zid.into_pyobject(py))?;
        }
        Ok(list)
    }

    fn peers_zid<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let info = self.get_info()?;
        let list = PyList::empty_bound(py);
        for zid in py.allow_threads(|| info.peers_zid().wait()) {
            list.append(zid.into_pyobject(py))?;
        }
        Ok(list)
    }

    // TODO __repr__
}
