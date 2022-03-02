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
    zkey_expr_of_pyany, zvalue_of_pyany, CongestionControl, KeyExpr, Period, Priority, Query,
    QueryConsolidation, QueryTarget, Queryable, Reliability, Reply, Sample, SubMode, Subscriber,
    ZnSubOps,
};
use super::{to_pyerr, ZError};
use async_std::channel::bounded;
use async_std::task;
use futures::prelude::*;
use futures::select;
use log::warn;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyTuple};
use std::collections::HashMap;
use zenoh::prelude::{ExprId, KeyExpr as ZKeyExpr, Selector, ZFuture, ZInt};

/// A zenoh session.
#[pyclass]
pub struct Session {
    s: Option<zenoh::Session>,
}

#[pymethods]
impl Session {
    /// Close the zenoh Session.
    pub fn close(&mut self) -> PyResult<()> {
        let s = self.take()?;
        s.close().wait().map_err(to_pyerr)
    }

    /// Get informations about the zenoh Session.
    ///
    /// :rtype: dict {str: str}
    ///
    /// :Example:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open()
    /// >>> info = s.info()
    /// >>> for key in info:
    /// >>>    print("{} : {}".format(key, info[key]))
    pub fn info(&self, py: Python) -> PyResult<PyObject> {
        use zenoh_cfg_properties::KeyTranscoder;
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
    /// The *key_expr* parameter also accepts the following types that can be converted to a :class:`KeyExpr`:
    ///
    /// * **int** for a mapped key expression
    /// * **str** for a literal key expression
    /// * **(int, str)** for a mapped key expression with suffix
    ///
    /// :param key_expr: The key expression matching resources to write
    /// :type key_expr: :class:`KeyExpr`
    /// :param value: The value to write
    /// :type value: any type convertible to a :class:`Value`
    /// :param encoding: The encoding of the value
    /// :type encoding: int, optional
    /// :param kind: The kind of value
    /// :type kind: int, optional
    /// :param congestion_control: The value for the congestion control
    /// :type congestion_control: :class:`CongestionControl`, optional
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open()
    /// >>> s.put('/key/expression', 'value')
    #[pyo3(text_signature = "(self, key_expr, value, **kwargs)")]
    #[args(kwargs = "**")]
    pub fn put(&self, key_expr: &PyAny, value: &PyAny, kwargs: Option<&PyDict>) -> PyResult<()> {
        let s = self.as_ref()?;
        let k = zkey_expr_of_pyany(key_expr)?;
        let mut v = zvalue_of_pyany(value)?;
        let mut encoding: Option<Encoding> = None;
        let mut kind: Option<SampleKind> = None;
        let mut congestion_control: Option<CongestionControl> = None;
        let mut priority: Option<Priority> = None;
        let mut local_routing: Option<bool> = None;
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
            if let Some(lr) = kwargs.get_item("local_routing") {
                local_routing = lr.extract().ok()
            }
        }
        if let Some(encoding) = encoding {
            v.encoding = encoding.into();
        }
        let mut writer = s
            .put(k, v)
            .kind(kind.unwrap_or_default().kind)
            .congestion_control(congestion_control.unwrap_or_default().cc)
            .priority(priority.unwrap_or_default().p);
        if let Some(local_routing) = local_routing {
            writer = writer.local_routing(local_routing);
        }
        writer.wait().map_err(to_pyerr)
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
    /// :type key_expr: :class:`KeyExpr`
    /// :param congestion_control: The value for the congestion control
    /// :type congestion_control: :class:`CongestionControl`, optional
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open()
    /// >>> s.delete('/key/expression')
    #[pyo3(text_signature = "(self, key_expr, **kwargs)")]
    #[args(kwargs = "**")]
    pub fn delete(&self, key_expr: &PyAny, kwargs: Option<&PyDict>) -> PyResult<()> {
        let s = self.as_ref()?;
        let k = zkey_expr_of_pyany(key_expr)?;
        let mut congestion_control: Option<CongestionControl> = None;
        let mut priority: Option<Priority> = None;
        let mut local_routing: Option<bool> = None;
        if let Some(kwargs) = kwargs {
            if let Some(cc) = kwargs.get_item("congestion_control") {
                congestion_control = cc.extract().ok()
            }
            if let Some(p) = kwargs.get_item("priority") {
                priority = p.extract().ok()
            }
            if let Some(lr) = kwargs.get_item("local_routing") {
                local_routing = lr.extract().ok()
            }
        }
        let mut writer = s
            .delete(k)
            .congestion_control(congestion_control.unwrap_or_default().cc)
            .priority(priority.unwrap_or_default().p);
        if let Some(local_routing) = local_routing {
            writer = writer.local_routing(local_routing);
        }
        writer.wait().map_err(to_pyerr)
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
    /// :type key_expr: :class:`KeyExpr`
    /// :rtype: :class:`ExprId`
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open()
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
    /// :type rid: :class:`ExprId`
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open()
    /// >>> rid = s.declare_expr('/key/expression')
    /// >>> s.undeclare_expr(rid)
    #[pyo3(text_signature = "(self, rid)")]
    pub fn undeclare_expr(&self, rid: ExprId) -> PyResult<()> {
        let s = self.as_ref()?;
        s.undeclare_expr(rid).wait().map_err(to_pyerr)
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
    /// :type key_expr: :class:`KeyExpr`
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open()
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
    /// :param callback: the subscription callback
    /// :type callback: function(:class:`Sample`)
    /// :param reliability: the subscription reliability
    /// :type reliability: :class:`Reliability`, optional
    /// :param mode: the subscription mode
    /// :type mode: :class:`SubMode`, optional
    /// :param period: the subscription period
    /// :type period: :class:`Period`, optional
    /// :param local: if the subscription is local only
    /// :type local: bool
    /// :rtype: :class:`Subscriber`
    ///
    /// :Examples:
    ///
    /// >>> import zenoh, time
    /// >>> from zenoh import Reliability, SubMode
    /// >>>
    /// >>> s = zenoh.open()
    /// >>> sub = s.subscribe('/key/expression',
    /// ...     lambda sample: print("Received : {}".format(sample)),
    /// ...     reliability=Reliability.Reliable,
    /// ...     mode=SubMode.Push)
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
        let mut sub_builder = s.subscribe(&k);
        if let Some(kwargs) = kwargs {
            if let Some(arg) = kwargs.get_item("reliability") {
                sub_builder = sub_builder.reliability(arg.extract::<Reliability>()?.r);
            }
            if let Some(arg) = kwargs.get_item("mode") {
                sub_builder = sub_builder.mode(arg.extract::<SubMode>()?.m);
            }
            if let Some(arg) = kwargs.get_item("period") {
                sub_builder = sub_builder.period(Some(arg.extract::<Period>()?.p));
            }
            if let Some(arg) = kwargs.get_item("local") {
                if arg.extract::<bool>()? {
                    sub_builder = sub_builder.local();
                }
            }
        }
        let zn_sub = sub_builder.wait().map_err(to_pyerr)?;
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
    /// :type key_expr: :class:`KeyExpr`
    /// :param info: The kind of Queryable
    /// :type info: int
    /// :param callback: the queryable callback
    /// :type callback: function(:class:`Query`)
    /// :rtype: :class:`Queryable`
    ///
    /// :Examples:
    ///
    /// >>> import zenoh, time
    /// >>> from zenoh import Sample, queryable
    /// >>> def callback(query):
    /// ...     print("Received : {}".format(query))
    /// ...     query.reply(Sample('/key/expression', bytes('value', encoding='utf8')))
    /// >>>
    /// >>> s = zenoh.open()
    /// >>> q = s.queryable('/key/expression', queryable.EVAL, callback)
    /// >>> time.sleep(60)
    #[pyo3(text_signature = "(self, key_expr, kind, callback)")]
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
    /// Replies are collected in a list.
    ///
    /// The *selector* parameter accepts the following types:
    ///
    /// * **KeyExpr** for a key expression with no value selector
    /// * **int** for a key expression id with no value selector
    /// * **str** for a litteral selector
    ///
    /// :param selector: The selection of resources to query
    /// :type selector: str
    /// :param target: The kind of queryables that should be target of this query
    /// :type target: :class:`QueryTarget`, optional
    /// :param consolidation: The kind of consolidation that should be applied on replies
    /// :type consolidation: :class:`QueryConsolidation`, optional
    /// :rtype: [:class:`Reply`]
    ///
    /// :Examples:
    ///
    /// >>> import zenoh, time
    /// >>>
    /// >>> s = zenoh.open()
    /// >>> replies = s.get('/key/selector?value_selector')
    /// >>> for reply in replies:
    /// ...    print("Received : {}".format(reply.data))
    #[pyo3(
        text_signature = "(self, selector, target=None, consolidation=None, local_routing=True)"
    )]
    fn get(
        &self,
        selector: &PyAny,
        target: Option<QueryTarget>,
        consolidation: Option<QueryConsolidation>,
        local_routing: Option<bool>,
    ) -> PyResult<Py<PyList>> {
        let s = self.as_ref()?;
        task::block_on(async {
            let selector: Selector = match selector.get_type().name()? {
                "KeyExpr" => {
                    let key_expr: PyRef<KeyExpr> = selector.extract()?;
                    key_expr.inner.clone().into()
                }
                "int" => {
                    let id: u64 = selector.extract()?;
                    ZKeyExpr::from(id).into()
                }
                "str" => {
                    let name: &str = selector.extract()?;
                    Selector::from(name)
                }
                x => {
                    return Err(PyErr::new::<exceptions::PyValueError, _>(format!(
                        "Cannot convert type '{}' to a zenoh Selector",
                        x
                    )))
                }
            };
            let mut getter = s.get(selector).target(target.unwrap_or_default().t);
            if let Some(consolidation) = consolidation {
                getter = getter.consolidation(consolidation.c);
            }
            if let Some(local_routing) = local_routing {
                getter = getter.local_routing(local_routing)
            }
            let mut replies = getter.wait().map_err(to_pyerr)?;
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
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh session was closed"))
    }

    #[inline]
    fn take(&mut self) -> PyResult<zenoh::Session> {
        self.s
            .take()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh session was closed"))
    }
}
