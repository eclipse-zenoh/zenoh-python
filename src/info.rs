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
use pyo3::{prelude::*, types::PyList};
use zenoh_core::SyncResolve;

use crate::{
    config::ZenohId,
    utils::{wrapper, IntoPython},
};

wrapper!(zenoh::info::SessionInfo<'static>);

#[pymethods]
impl SessionInfo {
    fn zid(&self, py: Python) -> ZenohId {
        py.allow_threads(|| self.0.zid().res_sync()).into()
    }

    fn routers_zid<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let list = PyList::empty_bound(py);
        for zid in py.allow_threads(|| self.0.routers_zid().res_sync()) {
            list.append(zid.into_pyobject(py))?;
        }
        Ok(list)
    }

    fn peers_zid<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let list = PyList::empty_bound(py);
        for zid in py.allow_threads(|| self.0.peers_zid().res_sync()) {
            list.append(zid.into_pyobject(py))?;
        }
        Ok(list)
    }
}
