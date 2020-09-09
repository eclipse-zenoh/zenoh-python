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
use async_std::task;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

use super::ResKey;
use crate::{to_pyerr, ZError};
use zenoh::net::{ResourceId, ZInt};

#[pyclass]
pub(crate) struct Session {
    s: Option<zenoh::net::Session>,
}

#[pymethods]
impl Session {
    fn close(&mut self) -> PyResult<()> {
        let s = self.take()?;
        task::block_on(s.close()).map_err(to_pyerr)
    }

    fn info<'p>(&self, py: Python<'p>) -> PyResult<Vec<(ZInt, &'p PyBytes)>> {
        let s = self.as_ref()?;
        let props = task::block_on(s.info());
        Ok(props
            .iter()
            .map(|(k, v)| (*k, PyBytes::new(py, v.as_slice())))
            .collect())
    }

    fn write(&self, resource: &ResKey, payload: Vec<u8>) -> PyResult<()> {
        let s = self.as_ref()?;
        task::block_on(s.write(&resource.k, payload.into())).map_err(to_pyerr)
    }

    fn declare_resource(&self, resource: &ResKey) -> PyResult<ResourceId> {
        let s = self.as_ref()?;
        task::block_on(s.declare_resource(&resource.k)).map_err(to_pyerr)
    }
}

impl Session {
    pub(crate) fn new(s: zenoh::net::Session) -> Self {
        Session { s: Some(s) }
    }

    #[inline]
    fn as_ref(&self) -> PyResult<&zenoh::net::Session> {
        self.s
            .as_ref()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh-net session was closed"))
    }

    #[inline]
    fn take(&mut self) -> PyResult<zenoh::net::Session> {
        self.s
            .take()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh-net session was closed"))
    }
}
