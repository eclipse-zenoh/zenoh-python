use std::collections::HashMap;

use super::encoding::Encoding;
use super::sample_kind::SampleKind;
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
use super::types::{
    zkey_expr_of_pyany, CongestionControl, Priority, Query, QueryConsolidation, QueryTarget,
    Queryable, Reply, Sample, Subscriber, ZnSubOps,
};
use crate::types::{Reliability, SubMode};
use crate::{to_pyerr, ZError};
use async_std::channel::bounded;
use async_std::task;
use futures::prelude::*;
use futures::select;
use log::warn;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyTuple};
use zenoh::prelude::{ExprId, ZFuture, ZInt};

/// A zenoh-net session.
#[pyclass]
pub struct Session {
    s: Option<zenoh::Session>,
}

#[pymethods]
impl Session {
    /// Close the zenoh-net Session.
    pub fn close(&mut self) -> PyResult<()> {
        let s = self.take()?;
        s.close().wait().map_err(to_pyerr)
    }

    /// Get informations about the zenoh-net Session.
    ///
    /// :rtype: dict {str: str}
    ///
    /// :Example:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open({})
    /// >>> info = s.info()
    /// >>> for key in info:
    /// >>>    print("{} : {}".format(key, info[key]))
    pub fn info(&self, py: Python) -> PyResult<PyObject> {
        use zenoh_util::properties::KeyTranscoder;
        let s = self.as_ref()?;
        let props = s.info().wait();
        let pydict: HashMap<String, String> = props
            .0
            .into_iter()
            .filter_map(|(k, v)| zenoh::info::InfoTranscoder::decode(k).map(|k| (k, v)))
            .collect();
        Ok(pydict.into_py_dict(py).to_object(py))
    }

    /// Put data.
    ///
    /// The *resource* parameter also accepts the following types that can be converted to a :class:`KeyExpr`:
    ///
    /// * **int** for a ``KeyExpr.Rid(int)``
    /// * **str** for a ``KeyExpr.RName(str)``
    /// * **(int, str)** for a ``KeyExpr.RIdWithSuffix(int, str)``
    ///
    /// :param resource: The resource key to put
    /// :type resource: KeyExpr
    /// :param payload: The value to put
    /// :type payload: bytes
    /// :param encoding: The encoding of the value
    /// :type encoding: int, optional
    /// :param kind: The kind of value
    /// :type kind: int, optional
    /// :param congestion_control: The value for the congestion control
    /// :type congestion_control: CongestionControl, optional
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open({})
    /// >>> s.put('/resource/name', bytes('value', encoding='utf8'))
    #[pyo3(text_signature = "(self, resource, payload, **kwargs)")]
    #[args(kwargs = "**")]
    pub fn put(&self, resource: &PyAny, payload: &[u8], kwargs: Option<&PyDict>) -> PyResult<()> {
        let s = self.as_ref()?;
        let k = zkey_expr_of_pyany(resource)?;
        let mut encoding: Option<Encoding> = None;
        let mut kind: Option<SampleKind> = None;
        let mut congestion_control: Option<CongestionControl> = None;
        let mut priority: Option<Priority> = None;
        if let Some(kwargs) = kwargs {
            if let Some(e) = kwargs.get_item("encoding") {
                encoding = e.extract().ok()
            }
            if let Some(k) = kwargs.get_item("kind") {
                kind = k.extract().ok()
            }
            if let Some(cc) = kwargs.get_item("congestion_control") {
                congestion_control = cc.extract().ok()
            }
            if let Some(p) = kwargs.get_item("priority") {
                priority = p.extract().ok()
            }
        }
        let value =
            zenoh::prelude::Value::from(payload).encoding(encoding.unwrap_or_default().into());
        s.put(k, value)
            .kind(kind.unwrap_or_default().kind)
            .congestion_control(congestion_control.unwrap_or_default().cc)
            .priority(priority.unwrap_or_default().p)
            .wait()
            .map_err(to_pyerr)
    }

    /// Associate a numerical Id with the given key expression.
    ///
    /// This numerical Id will be used on the network to save bandwidth and
    /// ease the retrieval of the concerned resource in the routing tables.
    ///
    /// The *key_expr* parameter also accepts the following types that can be converted to a :class:`KeyExpr`:
    ///
    /// * **int** for a mapped key expression
    /// * **str** for a literal key expression
    /// * **(int, str)** for a mapped key expression with suffix
    ///
    /// :param key_expr: The key expression to map to a numerical Id
    /// :type key_expr: KeyExpr
    /// :rtype: int
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open({})
    /// >>> rid = s.declare_expr('/key/expression')
    #[pyo3(text_signature = "(self, key_expr)")]
    pub fn declare_expr(&self, key_expr: &PyAny) -> PyResult<ExprId> {
        let s = self.as_ref()?;
        let k = zkey_expr_of_pyany(key_expr)?;
        s.declare_expr(&k).wait().map_err(to_pyerr)
    }

    /// Undeclare the *numerical Id/key expression* association previously declared
    /// with :meth:`declare_expr`.
    ///
    /// :param rid: The numerical Id to unmap
    /// :type rid: ExprId
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open({})
    /// >>> rid = s.declare_expr('/resource/name')
    /// >>> s.undeclare_expr(rid)
    #[pyo3(text_signature = "(self, rid)")]
    pub fn undeclare_expr(&self, rid: ExprId) -> PyResult<()> {
        let s = self.as_ref()?;
        s.undeclare_expr(rid).wait().map_err(to_pyerr)
    }

    /// Declare a publication for the given key expression.
    ///
    /// Written resources that match the given key expression will only be sent on the network
    /// if matching subscribers exist in the system.
    ///
    /// The *resource* parameter also accepts the following types that can be converted to a :class:`KeyExpr`:
    ///
    /// * **int** for a ``KeyExpr.Rid(int)``
    /// * **str** for a ``KeyExpr.RName(str)``
    /// * **(int, str)** for a ``KeyExpr.RIdWithSuffix(int, str)``
    ///
    /// :param key_expr: The key expression to publish
    /// :type key_expr: KeyExpr
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open({})
    /// >>> rid = s.declare_publication('/key/expression')
    /// >>> s.put('/key/expression', bytes('value', encoding='utf8'))
    #[pyo3(text_signature = "(self, key_expr)")]
    fn declare_publication(&self, key_expr: &PyAny) -> PyResult<()> {
        let s = self.as_ref()?;
        let k = zkey_expr_of_pyany(key_expr)?;
        s.declare_publication(&k).wait().map_err(to_pyerr)?;
        Ok(())
    }
    #[pyo3(text_signature = "(self, key_expr)")]
    fn undeclare_publication(&self, key_expr: &PyAny) -> PyResult<()> {
        let s = self.as_ref()?;
        let k = zkey_expr_of_pyany(key_expr)?;
        s.undeclare_publication(&k).wait().map_err(to_pyerr)?;
        Ok(())
    }

    /// Create a Subscriber for the given key expression.
    ///
    /// The *key_expr* parameter also accepts the following types that can be converted to a :class:`KeyExpr`:
    ///
    /// * **int** for a mapped key expression
    /// * **str** for a literal key expression
    /// * **(int, str)** for a mapped key expression with suffix
    ///
    /// :param key_expr: The key expression to subscribe
    /// :type key_expr: KeyExpr
    /// :param info: The :class:`SubInfo` to configure the subscription
    /// :type info: SubInfo
    /// :param callback: the subscription callback
    /// :type callback: function(:class:`Sample`)
    /// :rtype: Subscriber
    ///
    /// :Examples:
    ///
    /// >>> import zenoh, time
    /// >>> from zenoh import SubInfo, Reliability, SubMode
    /// >>>
    /// >>> s = zenoh.open({})
    /// >>> sub_info = SubInfo(Reliability.Reliable, SubMode.Push)
    /// >>> sub = s.subscribe('/key/expression', sub_info, lambda sample:
    /// ...     print("Received : {}".format(sample)))
    /// >>> time.sleep(60)
    #[pyo3(text_signature = "(self, key_expr, callback, **kwargs)")]
    #[args(kwargs = "**")]
    fn subscribe(
        &self,
        key_expr: &PyAny,
        callback: &PyAny,
        kwargs: Option<&PyDict>,
    ) -> PyResult<Subscriber> {
        let s = self.as_ref()?;
        let k = zkey_expr_of_pyany(key_expr)?;
        let mut reliability: Option<Reliability> = None;
        let mut mode: Option<SubMode> = None;
        if let Some(kwargs) = kwargs {
            if let Some(rarg) = kwargs.get_item("reliability") {
                reliability = rarg.extract().ok()
            }
            if let Some(marg) = kwargs.get_item("mode") {
                mode = marg.extract().ok()
            }
        }
        let zn_sub = s
            .subscribe(&k)
            .reliability(reliability.unwrap_or_default().r)
            .mode(mode.unwrap_or_default().m)
            .wait()
            .map_err(to_pyerr)?;
        // Note: workaround to allow moving of zn_sub into the task below.
        // Otherwise, s is moved also, but can't because it doesn't have 'static lifetime.
        let mut static_zn_sub = unsafe {
            std::mem::transmute::<
                zenoh::subscriber::Subscriber<'_>,
                zenoh::subscriber::Subscriber<'static>,
            >(zn_sub)
        };

        // Note: callback cannot be passed as such in task below because it's not Send
        let cb_obj: Py<PyAny> = callback.into();

        let (unregister_tx, unregister_rx) = bounded::<ZnSubOps>(8);
        // Note: This is done to ensure that even if the call-back into Python
        // does any blocking call we do not incour the risk of blocking
        // any of the task resolving futures.
        let loop_handle = task::spawn_blocking(move || {
            task::block_on(async move {
                loop {
                    select!(
                        s = static_zn_sub.receiver().next().fuse() => {
                            // Acquire Python GIL to call the callback
                            let gil = Python::acquire_gil();
                            let py = gil.python();
                            let cb_args = PyTuple::new(py, &[Sample { s: s.unwrap() }]);
                            if let Err(e) = cb_obj.as_ref(py).call1(cb_args) {
                                warn!("Error calling subscriber callback:");
                                e.print(py);
                            }
                        },
                        op = unregister_rx.recv().fuse() => {
                            match op {
                                Ok(ZnSubOps::Pull) => {
                                    if let Err(e) = static_zn_sub.pull().await {
                                        warn!("Error pulling the subscriber: {}", e);
                                    }
                                },
                                Ok(ZnSubOps::Unregister) => {
                                    if let Err(e) = static_zn_sub.close().await {
                                        warn!("Error undeclaring subscriber: {}", e);
                                    }
                                    return
                                },
                                _ => return
                            }
                        }
                    )
                }
            })
        });
        Ok(Subscriber {
            unregister_tx,
            loop_handle: Some(loop_handle),
        })
    }

    /// Create a Queryable for the given key expression.
    ///
    /// The *key_expr* parameter also accepts the following types that can be converted to a :class:`KeyExpr`:
    ///
    /// * **int** for a mapped key expression
    /// * **str** for a literal key expression
    /// * **(int, str)** for a mapped key expression with suffix
    ///
    /// :param key_expr: The key expression the Queryable will reply to
    /// :type key_expr: KeyExpr
    /// :param info: The kind of Queryable
    /// :type info: int
    /// :param callback: the queryable callback
    /// :type callback: function(:class:`Query`)
    /// :rtype: Queryable
    ///
    /// :Examples:
    ///
    /// >>> import zenoh, time
    /// >>> from zenoh import Sample, queryable
    /// >>> def callback(query):
    /// ...     print("Received : {}".format(query))
    /// ...     query.reply(Sample('/key/expression', bytes('value', encoding='utf8')))
    /// >>>
    /// >>> s = zenoh.open({})
    /// >>> q = s.queryable('/key/expression', queryable.EVAL, callback)
    /// >>> time.sleep(60)
    #[pyo3(text_signature = "(self, resource, kind, callback)")]
    fn queryable(&self, key_expr: &PyAny, kind: ZInt, callback: &PyAny) -> PyResult<Queryable> {
        let s = self.as_ref()?;
        let k = zkey_expr_of_pyany(key_expr)?;
        let zn_quer = s.queryable(k).kind(kind).wait().map_err(to_pyerr)?;
        // Note: workaround to allow moving of zn_quer into the task below.
        // Otherwise, s is moved also, but can't because it doesn't have 'static lifetime.
        let mut zn_quer = unsafe {
            std::mem::transmute::<
                zenoh::queryable::Queryable<'_>,
                zenoh::queryable::Queryable<'static>,
            >(zn_quer)
        };

        // Note: callback cannot be passed as such in task below because it's not Send
        let cb_obj: Py<PyAny> = callback.into();

        let (unregister_tx, unregister_rx) = bounded::<bool>(1);
        // Note: This is done to ensure that even if the call-back into Python
        // does any blocking call we do not incour the risk of blocking
        // any of the task resolving futures.
        let loop_handle = task::spawn_blocking(move || {
            task::block_on(async move {
                loop {
                    select!(
                        q = zn_quer.receiver().next().fuse() => {
                            // Acquire Python GIL to call the callback
                            let gil = Python::acquire_gil();
                            let py = gil.python();
                            let cb_args = PyTuple::new(py, &[Query { q: async_std::sync::Arc::new(q.unwrap()) }]);
                            if let Err(e) = cb_obj.as_ref(py).call1(cb_args) {
                                warn!("Error calling queryable callback:");
                                e.print(py);
                            }
                        },
                        _ = unregister_rx.recv().fuse() => {
                            if let Err(e) = zn_quer.close().await {
                                warn!("Error undeclaring queryable: {}", e);
                            }
                            return
                        }
                    )
                }
            })
        });
        Ok(Queryable {
            unregister_tx,
            loop_handle: Some(loop_handle),
        })
    }

    /// Query data from the matching queryables in the system.
    ///
    /// The replies are provided by calling the provided ``callback`` for each reply.
    /// The ``callback`` is called a last time with ``None`` when the query is complete.
    ///
    /// The *key_expr* parameter also accepts the following types that can be converted to a :class:`KeyExpr`:
    ///
    /// * **int** for a mapped key expression
    /// * **str** for a literal key expression
    /// * **(int, str)** for a mapped key expression with suffix
    ///
    /// :param key_expr: The key expression to query
    /// :type key_expr: KeyExpr
    /// :param predicate: An indication to matching queryables about the queried data
    /// :type predicate: str
    /// :param callback: the query callback which will receive the replies
    /// :type callback: function(:class:`Reply`)
    /// :param target: The kind of queryables that should be target of this query
    /// :type target: QueryTarget, optional
    /// :param consolidation: The kind of consolidation that should be applied on replies
    /// :type consolidation: QueryConsolidation, optional
    ///
    /// :Examples:
    ///
    /// >>> import zenoh, time
    /// >>> from zenoh import QueryTarget, queryable
    /// >>>
    /// >>> s = zenoh.open({})
    /// >>> s.get('/key/expression', '?predicate', lambda reply:
    /// ...    print("Received : {}".format(
    /// ...        reply.data if reply is not None else "FINAL")))
    #[pyo3(
        text_signature = "(self, key_expr, predicate, callback, target=None, consolidation=None)"
    )]
    fn get(
        &self,
        key_expr: &PyAny,
        predicate: &str,
        callback: &PyAny,
        target: Option<QueryTarget>,
        consolidation: Option<QueryConsolidation>,
    ) -> PyResult<()> {
        let s = self.as_ref()?;
        let key_selector = zkey_expr_of_pyany(key_expr)?;
        let mut zn_recv = s
            .get(zenoh::prelude::Selector {
                key_selector,
                value_selector: predicate,
            })
            .target(target.unwrap_or_default().t)
            .consolidation(consolidation.unwrap_or_default().c)
            .wait()
            .map_err(to_pyerr)?;

        // Note: callback cannot be passed as such in task below because it's not Send
        let cb_obj: Py<PyAny> = callback.into();

        let _ = task::spawn_blocking(move || {
            task::block_on(async move {
                while let Some(reply) = zn_recv.next().await {
                    // Acquire Python GIL to call the callback
                    let gil = Python::acquire_gil();
                    let py = gil.python();
                    let cb_args = PyTuple::new(py, &[Reply { r: reply }]);
                    if let Err(e) = cb_obj.as_ref(py).call1(cb_args) {
                        warn!("Error calling queryable callback:");
                        e.print(py);
                    }
                }
                let gil = Python::acquire_gil();
                let py = gil.python();
                let cb_args = PyTuple::new(py, &[py.None()]);
                if let Err(e) = cb_obj.as_ref(py).call1(cb_args) {
                    warn!("Error calling queryable callback:");
                    e.print(py);
                }
            })
        });
        Ok(())
    }

    /// Query data from the matching queryables in the system.
    ///
    /// Replies are collected in a list.
    ///
    /// The *key_expr* parameter also accepts the following types that can be converted to a :class:`KeyExpr`:
    ///
    /// * **int** for a mapped key expression
    /// * **str** for a literal key expression
    /// * **(int, str)** for a mapped key expression with suffix
    ///
    /// :param key_expr: The key expression to query
    /// :type key_expr: KeyExpr
    /// :param predicate: An indication to matching queryables about the queried data
    /// :type predicate: str
    /// :param target: The kind of queryables that should be target of this query
    /// :type target: QueryTarget, optional
    /// :param consolidation: The kind of consolidation that should be applied on replies
    /// :type consolidation: QueryConsolidation, optional
    /// :rtype: [:class:`Reply`]
    ///
    /// :Examples:
    ///
    /// >>> import zenoh, time
    /// >>> from zenoh import QueryTarget, queryable
    /// >>>
    /// >>> s = zenoh.open({})
    /// >>> replies = s.get_collect('/key/expression', 'predicate')
    /// >>> for reply in replies:
    /// ...    print("Received : {}".format(reply.data))
    #[pyo3(text_signature = "(self, key_expr, predicate, target=None, consolidation=None)")]
    fn get_collect(
        &self,
        key_expr: &PyAny,
        predicate: &str,
        target: Option<QueryTarget>,
        consolidation: Option<QueryConsolidation>,
    ) -> PyResult<Py<PyList>> {
        let s = self.as_ref()?;
        let k = zkey_expr_of_pyany(key_expr)?;
        task::block_on(async {
            let mut replies = s
                .get(zenoh::prelude::Selector::from(k).with_value_selector(predicate))
                .target(target.unwrap_or_default().t)
                .consolidation(consolidation.unwrap_or_default().c)
                .map_err(to_pyerr)
                .await?;
            let gil = Python::acquire_gil();
            let py = gil.python();
            let result = PyList::empty(py);
            while let Some(reply) = replies.next().await {
                result.append(Reply { r: reply })?;
            }
            Ok(result.into())
        })
    }
}

impl Session {
    pub(crate) fn new(s: zenoh::Session) -> Self {
        Session { s: Some(s) }
    }

    #[inline]
    fn as_ref(&self) -> PyResult<&zenoh::Session> {
        self.s
            .as_ref()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh-net session was closed"))
    }

    #[inline]
    fn take(&mut self) -> PyResult<zenoh::Session> {
        self.s
            .take()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh-net session was closed"))
    }
}
