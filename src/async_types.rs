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
use crate::types::*;
use async_std::channel::Sender;
use log::warn;
use pyo3::prelude::*;
use pyo3_asyncio::async_std::future_into_py;

/// A subscriber to be used with asyncio.
#[pyclass]
pub(crate) struct AsyncSubscriber {
    pub(crate) unregister_tx: Sender<ZnSubOps>,
    pub(crate) loop_handle: Option<async_std::task::JoinHandle<()>>,
}

#[pymethods]
impl AsyncSubscriber {
    /// Pull available data for a pull-mode subscriber.
    ///
    /// This method is a **coroutine**.
    fn pull<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let s = self.unregister_tx.clone();
        future_into_py(py, async move {
            if let Err(e) = s.send(ZnSubOps::Pull).await {
                warn!("Error in Subscriber::pull() : {}", e);
            }
            Ok(())
        })
    }

    /// Close the subscriber.
    ///
    /// This method is a **coroutine**.
    fn close<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        if let Some(handle) = self.loop_handle.take() {
            let s = self.unregister_tx.clone();
            future_into_py(py, async move {
                if let Err(e) = s.send(ZnSubOps::Unregister).await {
                    warn!("Error in Subscriber::close() : {}", e);
                }
                handle.await;
                Ok(())
            })
        } else {
            future_into_py(py, async move { Ok(()) })
        }
    }
}

/// A queryable to be used with asyncio.
#[pyclass]
pub(crate) struct AsyncQueryable {
    pub(crate) unregister_tx: Sender<bool>,
    pub(crate) loop_handle: Option<async_std::task::JoinHandle<()>>,
}

#[pymethods]
impl AsyncQueryable {
    /// Close the queryable.
    ///
    /// This method is a **coroutine**.
    fn close<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        if let Some(handle) = self.loop_handle.take() {
            let s = self.unregister_tx.clone();
            future_into_py(py, async move {
                if let Err(e) = s.send(true).await {
                    warn!("Error in Queryable::close() : {}", e);
                }
                handle.await;
                Ok(())
            })
        } else {
            future_into_py(py, async move { Ok(()) })
        }
    }
}
