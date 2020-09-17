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
use crate::types::*;
use async_std::sync::channel;
use async_std::task;
use futures::prelude::*;
use futures::select;
use log::warn;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

#[pyclass]
pub(crate) struct Workspace {
    pub(crate) w: zenoh::Workspace<'static>,
}

#[pymethods]
impl Workspace {
    fn put(&self, path: String, value: &PyAny) -> PyResult<()> {
        let p = path_of_string(path)?;
        let v = zvalue_of_pyany(value)?;
        task::block_on(self.w.put(&p, v)).map_err(to_pyerr)
    }

    fn delete(&self, path: String) -> PyResult<()> {
        let p = path_of_string(path)?;
        task::block_on(self.w.delete(&p)).map_err(to_pyerr)
    }

    fn get(&self, selector: String) -> PyResult<Vec<Data>> {
        let s = selector_of_string(selector)?;
        task::block_on(async {
            let mut data_stream = self.w.get(&s).await.map_err(to_pyerr)?;
            let mut result = vec![];
            while let Some(d) = data_stream.next().await {
                result.push(Data { d })
            }
            Ok(result)
        })
    }

    fn subscribe(&self, selector: String, callback: &PyAny) -> PyResult<Subscriber> {
        let s = selector_of_string(selector)?;
        let stream = task::block_on(self.w.subscribe(&s)).map_err(to_pyerr)?;
        // Note: workaround to allow moving of stream into the task below.
        // Otherwise, s is moved also, but can't because it doesn't have 'static lifetime.
        let mut static_stream = unsafe {
            std::mem::transmute::<zenoh::ChangeStream<'_>, zenoh::ChangeStream<'static>>(stream)
        };

        // Note: callback cannot be passed as such in task below because it's not Send
        let cb_obj: Py<PyAny> = callback.into();

        let (close_tx, close_rx) = channel::<bool>(1);
        // Note: This is done to ensure that even if the call-back into Python
        // does any blocking call we do not incour the risk of blocking
        // any of the task resolving futures.
        let loop_handle = task::spawn_blocking(move || {
            task::block_on(async move {
                loop {
                    select!(
                        change = static_stream.next().fuse() => {
                            // Acquire Python GIL to call the callback
                            let gil = Python::acquire_gil();
                            let py = gil.python();
                            let cb_args = PyTuple::new(py, &[Change { c: change.unwrap() }]);
                            if let Err(e) = cb_obj.as_ref(py).call1(cb_args) {
                                warn!("Error calling Subscriber callback:");
                                e.print(py);
                            }
                        },
                        _ = close_rx.recv().fuse() => {
                            if let Err(e) = static_stream.close().await {
                                warn!("Error closing Subscriber");
                            }
                            return()
                        }
                    )
                }
            })
        });
        Ok(Subscriber {
            close_tx,
            loop_handle: Some(loop_handle),
        })
    }

    fn register_eval(&self, path_expr: String, callback: &PyAny) -> PyResult<Eval> {
        let p = pathexpr_of_string(path_expr)?;
        let stream = task::block_on(self.w.register_eval(&p)).map_err(to_pyerr)?;
        // Note: workaround to allow moving of stream into the task below.
        // Otherwise, s is moved also, but can't because it doesn't have 'static lifetime.
        let mut static_stream = unsafe {
            std::mem::transmute::<zenoh::GetRequestStream<'_>, zenoh::GetRequestStream<'static>>(
                stream,
            )
        };

        // Note: callback cannot be passed as such in task below because it's not Send
        let cb_obj: Py<PyAny> = callback.into();

        let (close_tx, close_rx) = channel::<bool>(1);
        // Note: This is done to ensure that even if the call-back into Python
        // does any blocking call we do not incour the risk of blocking
        // any of the task resolving futures.
        let loop_handle = task::spawn_blocking(move || {
            task::block_on(async move {
                loop {
                    select!(
                        req = static_stream.next().fuse() => {
                            // Acquire Python GIL to call the callback
                            let gil = Python::acquire_gil();
                            let py = gil.python();
                            let cb_args = PyTuple::new(py, &[GetRequest { r: req.unwrap() }]);
                            if let Err(e) = cb_obj.as_ref(py).call1(cb_args) {
                                warn!("Error calling Subscriber callback:");
                                e.print(py);
                            }
                        },
                        _ = close_rx.recv().fuse() => {
                            if let Err(e) = static_stream.close().await {
                                warn!("Error closing Subscriber");
                            }
                            return()
                        }
                    )
                }
            })
        });
        Ok(Eval {
            close_tx,
            loop_handle: Some(loop_handle),
        })
    }
}
