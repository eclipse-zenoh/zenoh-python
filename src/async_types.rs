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
use crate::{to_pyerr, ZError};
use futures::prelude::*;
use pyo3::prelude::*;
use pyo3_asyncio::async_std::future_into_py;
use zenoh::prelude::r#async::AsyncResolve;
use zenoh::queryable::CallbackQueryable;
use zenoh::subscriber::CallbackSubscriber;

/// A subscriber to be used with asyncio.
#[pyclass]
pub(crate) struct AsyncSubscriber {
    pub(crate) inner: Option<CallbackSubscriber<'static>>,
}

#[pymethods]
impl AsyncSubscriber {
    /// Pull available data for a pull-mode subscriber.
    ///
    /// This method is NOT a **coroutine**.
    fn pull(&self, _py: Python) -> PyResult<()> {
        let inner = self
            .inner
            .as_ref()
            .ok_or_else(|| PyErr::new::<ZError, _>("the AsyncSubscriber was closed"))?;
        // no choice but to call pull() as sync:
        // self.inner cannot be passed into a future because self has an anonymous lifetime.
        use zenoh::prelude::sync::SyncResolve;
        inner.pull().res_sync().map_err(to_pyerr)
    }

    /// Close the subscriber.
    ///
    /// This method is a **coroutine**.
    fn close<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let inner = self
            .inner
            .take()
            .ok_or_else(|| PyErr::new::<ZError, _>("the AsyncSubscriber was already closed"))?;
        future_into_py(py, inner.close().res().map_err(to_pyerr))
    }
}

/// A queryable to be used with asyncio.
#[pyclass]
pub(crate) struct AsyncQueryable {
    pub(crate) inner: Option<CallbackQueryable<'static>>,
}

#[pymethods]
impl AsyncQueryable {
    /// Close the queryable.
    ///
    /// This method is a **coroutine**.
    fn close<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let inner = self
            .inner
            .take()
            .ok_or_else(|| PyErr::new::<ZError, _>("the AsyncQueryable was already closed"))?;
        future_into_py(py, inner.close().res().map_err(to_pyerr))
    }
}
