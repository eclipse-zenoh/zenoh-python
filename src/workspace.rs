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
use async_std::channel::bounded;
use async_std::task;
use futures::prelude::*;
use futures::select;
use log::warn;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use zenoh::net::Receiver;
use zenoh::ZFuture;

/// A Workspace to operate on zenoh.
///
/// A Workspace has an optional path prefix from which relative paths or selectors can be used.
///
/// :Example:
///
/// >>> import zenoh
/// >>> z = zenoh.Zenoh(zenoh.net.Config())
/// >>> ### Create a Workspace using prefix '/demo/example'
/// >>> w = z.workspace('/demo/example')
/// >>> ### Put using a relative path: '/demo/example/' + 'hello'
/// >>> w.put('hello', 'Hello World!')
/// >>> ### Note that absolute paths and selectors can still be used:
/// >>> w.put('/demo/exmaple/hello2', 'Hello World!')
/// >>> z.close()
#[pyclass]
pub(crate) struct Workspace {
    pub(crate) w: zenoh::Workspace<'static>,
}

#[pymethods]
impl Workspace {
    /// Returns the prefix that was used to create this Workspace (calling [`Zenoh.workspace()`]).
    ///
    /// :rtype: str
    #[getter]
    fn prefix(&self) -> Option<&str> {
        self.w.prefix().as_ref().map(|p| p.as_str())
    }

    /// Put a path/value into zenoh.
    ///
    /// The corresponding :class:`Change` will be received by all matching subscribers and all matching storages.
    /// Note that the path can be absolute or relative to this Workspace.
    ///
    /// The *value* parameter also accepts the following types that can be converted to a :class:`Value`:
    ///
    /// * **bytes** for a ``Value.Raw(APP_OCTET_STREAM, bytes)``
    /// * **str** for a ``Value.StringUtf8(str)``
    /// * **int** for a ``Value.Integer(int)``
    /// * **float** for a ``Value.Float(int)``
    /// * **dict of str:str** for a ``Value.Properties(dict)``
    /// * **(str, bytes)** for a ``Value.Custom(str, bytes)``
    ///
    /// :param path: the path
    /// :type path: str
    /// :param value: the value as a :class:`Value`
    /// :type value: Value
    ///
    /// :Example:
    ///
    /// >>> import zenoh
    /// >>> z = zenoh.Zenoh(zenoh.net.Config())
    /// >>> w = z.workspace()
    /// >>> w.put('/demo/exmaple/hello', 'Hello World!')
    /// >>> z.close()
    #[text_signature = "(self, path, value)"]
    fn put(&self, path: String, value: &PyAny) -> PyResult<()> {
        let p = path_of_string(path)?;
        let v = zvalue_of_pyany(value)?;
        self.w.put(&p, v).wait().map_err(to_pyerr)
    }

    /// Delete a path and its value from zenoh.
    ///
    /// The corresponding :class:`Change` will be received by all matching subscribers and all matching storages.
    /// Note that the path can be absolute or relative to this Workspace.
    ///
    /// :param path: the path
    /// :type path: str
    ///
    /// :Example:
    ///
    /// >>> import zenoh
    /// >>> z = zenoh.Zenoh(zenoh.net.Config())
    /// >>> w = z.workspace()
    /// >>> w.delete('/demo/exmaple/hello')
    /// >>> z.close()
    #[text_signature = "(self, path)"]
    fn delete(&self, path: String) -> PyResult<()> {
        let p = path_of_string(path)?;
        self.w.delete(&p).wait().map_err(to_pyerr)
    }

    /// Get a selection of path/value from zenoh.
    ///
    /// The selection is returned as a list of :class:`Data`.
    /// Note that the selector can be absolute or relative to this Workspace.
    ///
    /// :param selector: the selector
    /// :type selector: str
    /// :rtype: list of Data
    ///
    /// :Example:
    ///
    /// >>> import zenoh
    /// >>> z = zenoh.Zenoh(zenoh.net.Config())
    /// >>> w = z.workspace()
    /// >>> for data in w.get('/demo/example/**'):
    /// ...     print('  {} : {}  (encoding: {} , timestamp: {})'.format(
    /// ...         data.path, data.value.get_content(), data.value.encoding_descr(), data.timestamp))
    /// >>> z.close()
    #[text_signature = "(self, selector)"]
    fn get(&self, selector: String) -> PyResult<Vec<Data>> {
        let s = selector_of_string(selector)?;
        let data_stream = self.w.get(&s).wait().map_err(to_pyerr)?;
        let mut result = vec![];
        while let Ok(d) = data_stream.recv() {
            result.push(Data { d })
        }
        Ok(result)
    }

    /// Subscribe to changes for a selection of path/value (specified via a selector) from zenoh.
    ///
    /// The callback function will receive each :class:`Change` for a path matching the selector.
    /// Note that the selector can be absolute or relative to this Workspace.
    ///
    /// :param selector: the selector
    /// :type selector: str
    /// :param callback: the subscription callback
    /// :type callback: function(:class:`Change`)
    /// :rtype: zenoh.Subscriber
    ///
    /// :Example:
    ///
    /// >>> import zenoh, time
    /// >>> def listener(change):
    /// ...    print(">> [Subscription listener] received {:?} for {} : {} with timestamp {}"
    /// ...    .format(change.kind, change.path, '' if change.value is None else change.value.get_content(), change.timestamp))
    /// >>>
    /// >>> z = zenoh.Zenoh(zenoh.net.Config())
    /// >>> w = z.workspace()
    /// >>> sub = w.subscribe('/demo/example/**', listener)
    /// >>> time.sleep(60)
    /// >>> sub.close()
    /// >>> z.close()
    #[text_signature = "(self, selector, callback)"]
    fn subscribe(&self, selector: String, callback: &PyAny) -> PyResult<Subscriber> {
        let s = selector_of_string(selector)?;
        let receiver = self.w.subscribe(&s).wait().map_err(to_pyerr)?;
        // Note: workaround to allow moving of receiver into the task below.
        // Otherwise, s is moved also, but can't because it doesn't have 'static lifetime.
        let mut static_receiver = unsafe {
            std::mem::transmute::<zenoh::ChangeReceiver<'_>, zenoh::ChangeReceiver<'static>>(
                receiver,
            )
        };

        // Note: callback cannot be passed as such in task below because it's not Send
        let cb_obj: Py<PyAny> = callback.into();

        let (close_tx, close_rx) = bounded::<bool>(1);
        // Note: This is done to ensure that even if the call-back into Python
        // does any blocking call we do not incour the risk of blocking
        // any of the task resolving futures.
        let loop_handle = task::spawn_blocking(move || {
            task::block_on(async move {
                loop {
                    select!(
                        change = static_receiver.next().fuse() => {
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
                            if let Err(e) = static_receiver.close().await {
                                warn!("Error closing Subscriber: {}", e);
                            }
                            return
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

    /// Registers an evaluation function under the provided path expression.
    ///
    /// The callback function will receive a :class:`GetRequest` for each get operation
    /// called with a selector that patches the path expression. The callback implementation
    /// has to send replies via :meth:`GetRequest.reply`.
    /// Note that the path expression can be absolute or relative to this Workspace.
    ///
    /// :param path_expr: the path expression
    /// :type path_expr: str
    /// :param callback: the eval callback
    /// :type callback: function(:class:`GetRequest`)
    /// :rtype: zenoh.Eval
    ///
    /// :Example:
    ///
    /// >>> import zenoh, time
    /// >>> def eval_callback(get_request):
    /// ...    print(">> [Eval listener] received get with selector: {}".format(get_request.selector))
    /// ...    get_request.reply('/demo/example/eval', 'Result for get on {}'.format(get_request.selector))
    /// >>>
    /// >>> z = zenoh.Zenoh(zenoh.net.Config())
    /// >>> w = z.workspace()
    /// >>> eval = w.register_eval('/demo/example/eval', eval_callback)
    /// >>> time.sleep(60)
    /// >>> eval.close()
    /// >>> z.close()
    #[text_signature = "(self, path_expr, callback)"]
    fn register_eval(&self, path_expr: String, callback: &PyAny) -> PyResult<Eval> {
        let p = pathexpr_of_string(path_expr)?;
        let receiver = self.w.register_eval(&p).wait().map_err(to_pyerr)?;
        // Note: workaround to allow moving of stream into the task below.
        // Otherwise, s is moved also, but can't because it doesn't have 'static lifetime.
        let mut static_receiver = unsafe {
            std::mem::transmute::<zenoh::GetRequestStream<'_>, zenoh::GetRequestStream<'static>>(
                receiver,
            )
        };

        // Note: callback cannot be passed as such in task below because it's not Send
        let cb_obj: Py<PyAny> = callback.into();

        let (close_tx, close_rx) = bounded::<bool>(1);
        // Note: This is done to ensure that even if the call-back into Python
        // does any blocking call we do not incour the risk of blocking
        // any of the task resolving futures.
        let loop_handle = task::spawn_blocking(move || {
            task::block_on(async move {
                loop {
                    select!(
                        req = static_receiver.next().fuse() => {
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
                            if let Err(e) = static_receiver.close().await {
                                warn!("Error closing Subscriber: {}", e);
                            }
                            return
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
