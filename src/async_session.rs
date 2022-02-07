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
use super::encoding::Encoding;
use super::sample_kind::SampleKind;
use super::types::{
    zkey_expr_of_pyany, zvalue_of_pyany, CongestionControl, Priority, Query, QueryConsolidation,
    QueryTarget, Queryable, Reply, Sample, Subscriber, ZnSubOps,
};
use crate::types::{KeyExpr, Period, Reliability, SubMode};
use crate::{to_pyerr, ZError};
use async_std::channel::bounded;
use async_std::sync::Arc;
use async_std::task;
use futures::prelude::*;
use futures::select;
use log::warn;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyTuple};
use pyo3_asyncio::async_std::future_into_py;
use std::collections::HashMap;
use zenoh::prelude::{ExprId, KeyExpr as ZKeyExpr, ZFuture, ZInt};

/// A zenoh-net session.
#[pyclass]
pub struct AsyncSession {
    s: Option<Arc<zenoh::Session>>,
}

#[pymethods]
impl AsyncSession {
    // NOTE: See https://github.com/awestlake87/pyo3-asyncio/issues/50 for the options
    // to implement asyncronous methods with async-pyo3.
    // Here we choosed to wrap the Session in an Arc, and move a clone of it in each `future_into_py()` call.
    // Similarly, each argument coming from Python is converted to Rust and cloned (or Arc-wrapped) before
    // moving into the `future_into_py()` call.

    /// Close the zenoh-net Session.
    pub fn close<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        // NOTE: should be sufficient to take the Arc<Session>. Once all arcs are dropped, Session will close.
        // Still, we should provide a wait to await for the actual closure...
        let _s = self.try_take()?;
        future_into_py(py, async move { Ok(()) })
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
    pub fn info<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let s = self.try_clone()?;
        future_into_py(py, async move {
            use zenoh_cfg_properties::KeyTranscoder;
            let result = s
                .info()
                .map(|props| {
                    let hashmap: HashMap<String, String> = props
                        .0
                        .into_iter()
                        .filter_map(|(k, v)| zenoh::info::InfoTranscoder::decode(k).map(|k| (k, v)))
                        .collect();
                    Python::with_gil(|py| hashmap.into_py_dict(py).to_object(py))
                })
                .await;
            Ok(result)
        })
    }

    /// Put data.
    ///
    /// The *key_expr* parameter also accepts the following types that can be converted to a :class:`KeyExpr`:
    ///
    /// * **int** for a mapped key expression
    /// * **str** for a literal key expression
    /// * **(int, str)** for a mapped key expression with suffix
    ///
    /// :param key_expr: The key expression matching resources to write
    /// :type key_expr: KeyExpr
    /// :param value: The value to write
    /// :type value: Value
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
    /// >>> s.put('/key/expression', bytes('value', encoding='utf8'))
    #[pyo3(text_signature = "(self, key_expr, value, **kwargs)")]
    #[args(kwargs = "**")]
    pub fn put<'p>(
        &self,
        key_expr: &PyAny,
        value: &PyAny,
        kwargs: Option<&PyDict>,
        py: Python<'p>,
    ) -> PyResult<&'p PyAny> {
        let s = self.try_clone()?;
        let k = zkey_expr_of_pyany(key_expr)?.to_owned();
        let mut v = zvalue_of_pyany(value)?;
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
        if let Some(encoding) = encoding {
            v.encoding = encoding.into();
        }
        future_into_py(py, async move {
            s.put(k, v)
                .kind(kind.unwrap_or_default().kind)
                .congestion_control(congestion_control.unwrap_or_default().cc)
                .priority(priority.unwrap_or_default().p)
                .await
                .map_err(to_pyerr)
        })
    }

    /// Delete data.
    ///
    /// The *key_expr* parameter also accepts the following types that can be converted to a :class:`KeyExpr`:
    ///
    /// * **int** for a mapped key expression
    /// * **str** for a literal key expression
    /// * **(int, str)** for a mapped key expression with suffix
    ///
    /// :param key_expr: The key expression matching resources to delete
    /// :type key_expr: KeyExpr
    /// :param congestion_control: The value for the congestion control
    /// :type congestion_control: CongestionControl, optional
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open({})
    /// >>> s.delete('/key/expression')
    #[pyo3(text_signature = "(self, key_expr, **kwargs)")]
    #[args(kwargs = "**")]
    pub fn delete<'p>(
        &self,
        key_expr: &PyAny,
        kwargs: Option<&PyDict>,
        py: Python<'p>,
    ) -> PyResult<&'p PyAny> {
        let s = self.try_clone()?;
        let k = zkey_expr_of_pyany(key_expr)?.to_owned();
        let mut congestion_control: Option<CongestionControl> = None;
        let mut priority: Option<Priority> = None;
        if let Some(kwargs) = kwargs {
            if let Some(cc) = kwargs.get_item("congestion_control") {
                congestion_control = cc.extract().ok()
            }
            if let Some(p) = kwargs.get_item("priority") {
                priority = p.extract().ok()
            }
        }
        future_into_py(py, async move {
            s.delete(k)
                .congestion_control(congestion_control.unwrap_or_default().cc)
                .priority(priority.unwrap_or_default().p)
                .await
                .map_err(to_pyerr)
        })
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
    pub fn declare_expr<'p>(&self, key_expr: &PyAny, py: Python<'p>) -> PyResult<&'p PyAny> {
        let s = self.try_clone()?;
        let k = zkey_expr_of_pyany(key_expr)?.to_owned();
        future_into_py(py, async move { s.declare_expr(k).await.map_err(to_pyerr) })
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
    /// >>> rid = s.declare_expr('/key/expression')
    /// >>> s.undeclare_expr(rid)
    #[pyo3(text_signature = "(self, rid)")]
    pub fn undeclare_expr<'p>(&self, rid: ExprId, py: Python<'p>) -> PyResult<&'p PyAny> {
        let s = self.try_clone()?;
        future_into_py(
            py,
            async move { s.undeclare_expr(rid).wait().map_err(to_pyerr) },
        )
    }

    /// Declare a publication for the given key expression.
    ///
    /// Written expressions that match the given key expression will only be sent on the network
    /// if matching subscribers exist in the system.
    ///
    /// The *key_expr* parameter also accepts the following types that can be converted to a :class:`KeyExpr`:
    ///
    /// * **int** for a mapped key expression
    /// * **str** for a literal key expression
    /// * **(int, str)** for a mapped key expression with suffix
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
    fn declare_publication<'p>(&self, key_expr: &PyAny, py: Python<'p>) -> PyResult<&'p PyAny> {
        let s = self.try_clone()?;
        let k = zkey_expr_of_pyany(key_expr)?.to_owned();
        future_into_py(py, async move {
            s.declare_publication(k).await.map_err(to_pyerr)
        })
    }

    #[pyo3(text_signature = "(self, key_expr)")]
    fn undeclare_publication<'p>(&self, key_expr: &PyAny, py: Python<'p>) -> PyResult<&'p PyAny> {
        let s = self.try_clone()?;
        let k = zkey_expr_of_pyany(key_expr)?.to_owned();
        future_into_py(py, async move {
            s.undeclare_publication(k).await.map_err(to_pyerr)
        })
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
    /// >>> sub = s.subscribe('/key/expression',
    /// ...     lambda sample: print("Received : {}".format(sample)),
    /// ...     reliability=Reliability.Reliable,
    /// ...     mode=SubMode.Push)
    /// >>> time.sleep(60)
    #[pyo3(text_signature = "(self, key_expr, callback, **kwargs)")]
    #[args(kwargs = "**")]
    fn subscribe<'p>(
        &self,
        key_expr: &PyAny,
        callback: &PyAny,
        kwargs: Option<&PyDict>,
        py: Python<'p>,
    ) -> PyResult<&'p PyAny> {
        let s = self.try_clone()?;
        let k = zkey_expr_of_pyany(key_expr)?.to_owned();
        // Note: callback cannot be passed as such in task below because it's not Send
        let cb_obj: Py<PyAny> = callback.into();
        // note: extract from kwargs here because it's not Send and cannot be moved into future_into_py(py, F)
        let mut reliability: Option<Reliability> = None;
        let mut mode: Option<SubMode> = None;
        let mut period: Option<Period> = None;
        let mut local = false;
        if let Some(kwargs) = kwargs {
            if let Some(r) = kwargs.get_item("reliability") {
                reliability = Some(r.extract()?);
            }
            if let Some(m) = kwargs.get_item("mode") {
                mode = Some(m.extract()?);
            }
            if let Some(p) = kwargs.get_item("period") {
                period = Some(p.extract()?)
            }
            if let Some(p) = kwargs.get_item("local") {
                local = p.extract::<bool>()?;
            }
        }

        future_into_py(py, async move {
            // note: create SubscriberBuilder in this async block since its lifetime is bound to s which is moved here.
            let mut sub_builder = s.subscribe(&k);
            if let Some(r) = reliability {
                sub_builder = sub_builder.reliability(r.r);
            }
            if let Some(m) = mode {
                sub_builder = sub_builder.mode(m.m);
            }
            if let Some(p) = period {
                sub_builder = sub_builder.period(Some(p.p));
            }
            if local {
                sub_builder = sub_builder.local();
            }

            let zn_sub = sub_builder.await.map_err(to_pyerr)?;
            // Note: workaround to allow moving of zn_sub into the task below.
            // Otherwise, s is moved also, but can't because it doesn't have 'static lifetime.
            let mut static_zn_sub = unsafe {
                std::mem::transmute::<
                    zenoh::subscriber::Subscriber<'_>,
                    zenoh::subscriber::Subscriber<'static>,
                >(zn_sub)
            };

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
    #[pyo3(text_signature = "(self, key_expr, kind, callback)")]
    fn queryable<'p>(
        &self,
        key_expr: &PyAny,
        kind: ZInt,
        callback: &PyAny,
        py: Python<'p>,
    ) -> PyResult<&'p PyAny> {
        let s = self.try_clone()?;
        let k = zkey_expr_of_pyany(key_expr)?.to_owned();
        // Note: callback cannot be passed as such in task below because it's not Send
        let cb_obj: Py<PyAny> = callback.into();

        future_into_py(py, async move {
            let zn_quer = s.queryable(k).kind(kind).await.map_err(to_pyerr)?;
            // Note: workaround to allow moving of zn_quer into the task below.
            // Otherwise, s is moved also, but can't because it doesn't have 'static lifetime.
            let mut zn_quer = unsafe {
                std::mem::transmute::<
                    zenoh::queryable::Queryable<'_>,
                    zenoh::queryable::Queryable<'static>,
                >(zn_quer)
            };

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
        })
    }

    /// Query data from the matching queryables in the system.
    ///
    /// The replies are provided by calling the provided ``callback`` for each reply.
    /// The ``callback`` is called a last time with ``None`` when the query is complete.
    ///
    /// The *selector* parameter accepts the following types:
    ///
    /// * **KeyExpr** for a key expression with no value selector
    /// * **int** for a key expression id with no value selector
    /// * **str** for a litteral selector
    ///
    /// :param selector: The selection of resources to query
    /// :type selector: str
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
    /// >>> s.get('/key/selector?value_selector', lambda reply:
    /// ...    print("Received : {}".format(
    /// ...        reply.data if reply is not None else "FINAL")))
    #[pyo3(text_signature = "(self, selector, callback, target=None, consolidation=None)")]
    fn get(
        &self,
        selector: &PyAny,
        callback: &PyAny,
        target: Option<QueryTarget>,
        consolidation: Option<QueryConsolidation>,
    ) -> PyResult<()> {
        // TODO AS ACYNCIO
        let s = self.as_ref()?;
        let mut getter = match selector.get_type().name()? {
            "KeyExpr" => {
                let rk: PyRef<KeyExpr> = selector.extract()?;
                s.get(rk.inner.clone())
            }
            "int" => {
                let id: u64 = selector.extract()?;
                s.get(ZKeyExpr::from(id))
            }
            "str" => {
                let name: &str = selector.extract()?;
                s.get(name)
            }
            x => {
                return Err(PyErr::new::<exceptions::PyValueError, _>(format!(
                    "Cannot convert type '{}' to a zenoh Selector",
                    x
                )))
            }
        };
        if let Some(t) = target {
            getter = getter.target(t.t);
        }
        if let Some(c) = consolidation {
            getter = getter.consolidation(c.c);
        }
        let mut zn_recv = getter.wait().map_err(to_pyerr)?;

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
}

impl AsyncSession {
    pub(crate) fn new(s: zenoh::Session) -> Self {
        AsyncSession {
            s: Some(s.into_arc()),
        }
    }

    #[inline]
    fn try_clone(&self) -> PyResult<Arc<zenoh::Session>> {
        self.s
            .clone()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh-net session was closed"))
    }

    #[inline]
    fn try_take(&mut self) -> PyResult<Arc<zenoh::Session>> {
        self.s
            .take()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh-net session was closed"))
    }

    #[inline]
    fn as_ref(&self) -> PyResult<&zenoh::Session> {
        self.s
            .as_ref()
            .map(|a| a.as_ref())
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh-net session was closed"))
    }
}
