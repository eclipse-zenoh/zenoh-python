//
// Copyright (c) 2017, 2022 ZettaScale Technology
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
use super::async_types::{AsyncQueryable, AsyncSubscriber};
use super::encoding::Encoding;
use super::sample_kind::SampleKind;
use super::types::{
    zkey_expr_of_pyany, zvalue_of_pyany, CongestionControl, KeyExpr, Period, Priority, Query,
    QueryConsolidation, QueryTarget, Reliability, Reply, Sample, SubMode, ZnSubOps,
};
use super::{to_pyerr, ZError};
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
use zenoh::prelude::{ExprId, KeyExpr as ZKeyExpr, Selector, ZFuture, ZInt};

/// A zenoh session to be used with asyncio.
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

    /// Close the zenoh Session.
    pub fn close<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        // NOTE: should be sufficient to take the Arc<Session>. Once all arcs are dropped, Session will close.
        // Still, we should provide a wait to await for the actual closure...
        let _s = self.try_take()?;
        future_into_py(py, async move { Ok(()) })
    }

    /// Get informations about the zenoh Session.
    ///
    /// This method is a **coroutine**.
    ///
    /// :rtype: **dict[str, str]**
    ///
    /// :Example:
    ///
    /// >>> import asyncio, zenoh
    /// >>> async def main():
    /// >>>    s = await zenoh.async_open()
    /// >>>    info = await s.info()
    /// >>>    for key in info:
    /// >>>       print("{} : {}".format(key, info[key]))
    /// >>>
    /// >>> asyncio.run(main())
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
    /// This method is a **coroutine**.
    ///
    /// :param key_expr: The key expression matching resources to write
    /// :type key_expr: :class:`KeyExpr`
    /// :param value: The value to write
    /// :type value: any type convertible to a :class:`Value`
    /// :param \**kwargs:
    ///    See below
    ///
    /// :Keyword Arguments:
    ///    * **encoding** (:class:`Encoding`) --
    ///      Set the encoding of the written data
    ///    * **kind** ( **int** ) --
    ///      Set the kind of the written data
    ///    * **congestion_control** (:class:`CongestionControl`) --
    ///      Set the congestion control to apply when routing the data
    ///    * **priority** (:class:`Priority`) --
    ///      Set the priority of the written data
    ///    * **local_routing** ( **bool** ) --
    ///      Enable or disable local routing
    ///
    /// :raise: :class:`ZError`
    ///
    /// :Examples:
    ///
    /// >>> import asyncio, zenoh
    /// >>> async def main():
    /// >>>    s = await zenoh.async_open()
    /// >>>    await s.put('/key/expression', bytes('value', encoding='utf8'))
    /// >>>
    /// >>> asyncio.run(main())
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
        future_into_py(py, async move {
            let mut writer = s
                .put(k, v)
                .kind(kind.unwrap_or_default().kind)
                .congestion_control(congestion_control.unwrap_or_default().cc)
                .priority(priority.unwrap_or_default().p);
            if let Some(local_routing) = local_routing {
                writer = writer.local_routing(local_routing);
            }
            writer.await.map_err(to_pyerr)
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
    /// This method is a **coroutine**.
    ///
    /// :param key_expr: The key expression matching resources to delete
    /// :type key_expr: :class:`KeyExpr`
    /// :param \**kwargs:
    ///    See below
    ///
    /// :Keyword Arguments:
    ///    * **congestion_control** (:class:`CongestionControl`) --
    ///      Set the congestion control to apply when routing the data
    ///    * **priority** (:class:`Priority`) --
    ///      Set the priority of the written data
    ///    * **local_routing** ( **bool** ) --
    ///      Enable or disable local routing
    ///
    /// :raise: :class:`ZError`
    ///
    /// :Examples:
    ///
    /// >>> import asyncio, zenoh
    /// >>> async def main():
    /// >>>    s = await zenoh.async_open()
    /// >>>    await s.delete('/key/expression')
    /// >>>
    /// >>> asyncio.run(main())
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
        future_into_py(py, async move {
            let mut writer = s
                .delete(k)
                .congestion_control(congestion_control.unwrap_or_default().cc)
                .priority(priority.unwrap_or_default().p);
            if let Some(local_routing) = local_routing {
                writer = writer.local_routing(local_routing);
            }
            writer.await.map_err(to_pyerr)
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
    /// This method is a **coroutine**.
    ///
    /// :param key_expr: The key expression to map to a numerical Id
    /// :type key_expr: :class:`KeyExpr`
    /// :rtype: **int**
    /// :raise: :class:`ZError`
    ///
    /// :Examples:
    ///
    /// >>> import asyncio, zenoh
    /// >>> async def main():
    /// >>>    s = await zenoh.async_open()
    /// >>>    rid = await s.declare_expr('/key/expression')
    /// >>>
    /// >>> asyncio.run(main())
    #[pyo3(text_signature = "(self, key_expr)")]
    pub fn declare_expr<'p>(&self, key_expr: &PyAny, py: Python<'p>) -> PyResult<&'p PyAny> {
        let s = self.try_clone()?;
        let k = zkey_expr_of_pyany(key_expr)?.to_owned();
        future_into_py(py, async move { s.declare_expr(k).await.map_err(to_pyerr) })
    }

    /// Undeclare the *numerical Id/key expression* association previously declared
    /// with :meth:`declare_expr`.
    ///
    /// This method is a **coroutine**.
    ///
    /// :param rid: The numerical Id to unmap
    /// :type rid: ExprId
    /// :raise: :class:`ZError`
    ///
    /// :Examples:
    ///
    /// >>> import asyncio, zenoh
    /// >>> async def main():
    /// >>>    s = await zenoh.async_open()
    /// >>>    rid = await s.declare_expr('/key/expression')
    /// >>>    await s.undeclare_expr(rid)
    /// >>>
    /// >>> asyncio.run(main())
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
    /// This method is a **coroutine**.
    ///
    /// :param key_expr: The key expression to publish
    /// :type key_expr: :class:`KeyExpr`
    /// :raise: :class:`ZError`
    ///
    /// :Examples:
    ///
    /// >>> import asyncio, zenoh
    /// >>> async def main():
    /// >>>    s = await zenoh.async_open()
    /// >>>    rid = await s.declare_publication('/key/expression')
    /// >>>    await s.put('/key/expression', bytes('value', encoding='utf8'))
    /// >>>
    /// >>> asyncio.run(main())
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

    /// Create an AsyncSubscriber for the given key expression.
    ///
    /// The *key_expr* parameter also accepts the following types that can be converted to a :class:`KeyExpr`:
    ///
    /// * **int** for a mapped key expression
    /// * **str** for a literal key expression
    /// * **(int, str)** for a mapped key expression with suffix
    ///
    /// This method is a **coroutine**.
    ///
    /// :param key_expr: The key expression to subscribe
    /// :type key_expr: :class:`KeyExpr`
    /// :param callback: the subscription callback (must be a **coroutine**)
    /// :type callback: async function(:class:`Sample`)
    /// :param \**kwargs:
    ///    See below
    ///
    /// :Keyword Arguments:
    ///    * **reliability** (:class:`Reliability`) --
    ///      Set the subscription reliability (BestEffort by default)
    ///    * **mode** (:class:`SubMode`) --
    ///      Set the subscription mode (Push by default)
    ///    * **period** (:class:`Period`) --
    ///      Set the subscription period
    ///    * **local** ( **bool** ) --
    ///      If true make the subscription local only (false by default)
    ///
    /// :rtype: :class:`Subscriber`
    /// :raise: :class:`ZError`
    ///
    /// :Examples:
    ///
    /// >>> import asyncio, zenoh
    /// >>> from zenoh import Reliability, SubMode
    /// >>>
    /// >>> async def callback(sample):
    /// >>>    print("Received : {}".format(sample))
    /// >>>
    /// >>> async def main():
    /// >>>    s = await zenoh.async_open()
    /// >>>    sub = await s.subscribe('/key/expression', callback,
    /// ...       reliability=Reliability.Reliable,
    /// ...       mode=SubMode.Push)
    /// >>>    await asycio.sleep(60)
    /// >>>
    /// >>> asyncio.run(main())
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
                Python::with_gil(|py| {
                    // Run a Python event loop in this task, to allow coroutines execution within the callback
                    match pyo3_asyncio::async_std::run(py, async move {
                        loop {
                            select!(
                                    s = static_zn_sub.receiver().next().fuse() => {
                                        // call the async callback and transform the resulting Python awaitable into a Rust future
                                        let future = match Python::with_gil(|py| {
                                            let cb_args = PyTuple::new(py, &[Sample { s: s.unwrap() }]);
                                            cb_obj.as_ref(py).call1(cb_args).and_then(pyo3_asyncio::async_std::into_future)
                                        }) {
                                            Ok(f) => f,
                                            Err(e) => { warn!("Error calling async queryable callback: {}", e); continue }
                                        };
                                        // await the future (by default callbacks are executed in sequence)
                                        if let Err(e) = future.await {
                                            warn!("Error suring axecution of async queryable callback: {}", e);
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
                                            return Ok(())
                                        },
                                        _ => return Ok(())
                                    }
                                }
                            )
                        }
                    }) {
                        Ok(()) => warn!("Queryable loop running"),
                        Err(e) => warn!("Failed to start Queryable loop: {}", e),
                    }
                })
            });
            Ok(AsyncSubscriber {
                unregister_tx,
                loop_handle: Some(loop_handle),
            })
        })
    }

    /// Create an AsyncQueryable for the given key expression.
    ///
    /// The *key_expr* parameter also accepts the following types that can be converted to a :class:`KeyExpr`:
    ///
    /// * **int** for a mapped key expression
    /// * **str** for a literal key expression
    /// * **(int, str)** for a mapped key expression with suffix
    ///
    /// This method is a **coroutine**.
    ///
    /// :param key_expr: The key expression the Queryable will reply to
    /// :type key_expr: :class:`KeyExpr`
    /// :param callback: the queryable callback (must be a **coroutine**)
    /// :type callback: async function(:class:`Query`)
    /// :param \**kwargs:
    ///    See below
    ///
    /// :Keyword Arguments:
    ///    * **kind** ( **int** ) --
    ///      Set the queryable kind. This must be a mask of constants defined in :mod:`zenoh.queryable`)
    ///      (`queryable.EVAL` by default)
    ///    * **complete** ( **bool** ) --
    ///      Set the queryable completeness (true by default)
    ///
    /// :rtype: :class:`Queryable`
    /// :raise: :class:`ZError`
    ///
    /// :Examples:
    ///
    /// >>> import asyncio, zenoh
    /// >>> from zenoh import Sample, queryable
    /// >>>
    /// >>> async def callback(query):
    /// ...     print("Received : {}".format(query))
    /// ...     query.reply(Sample('/key/expression', bytes('value', encoding='utf8')))
    /// >>>
    /// >>> async def main():
    /// >>>    s = await zenoh.async_open()
    /// >>>    q = await s.queryable('/key/expression', queryable.EVAL, callback)
    /// >>>    await asycio.sleep(60)
    /// >>>
    /// >>> asyncio.run(main())
    #[pyo3(text_signature = "(self, key_expr, callback, **kwargs)")]
    #[args(kwargs = "**")]
    fn queryable<'p>(
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
        let mut kind: Option<ZInt> = None;
        let mut complete: Option<bool> = None;
        if let Some(kwargs) = kwargs {
            if let Some(k) = kwargs.get_item("kind") {
                kind = Some(k.extract()?);
            }
            if let Some(p) = kwargs.get_item("local") {
                complete = Some(p.extract::<bool>()?);
            }
        }

        future_into_py(py, async move {
            let mut builder = s.queryable(k);
            if let Some(k) = kind {
                builder = builder.kind(k);
            }
            if let Some(c) = complete {
                builder = builder.complete(c);
            }
            let zn_quer = builder.await.map_err(to_pyerr)?;
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
                Python::with_gil(|py| {
                    // Run a Python event loop in this task, to allow coroutines execution within the callback
                    match pyo3_asyncio::async_std::run(py, async move {
                        loop {
                            select!(
                                q = zn_quer.receiver().next().fuse() => {
                                    // call the async callback and transform the resulting Python awaitable into a Rust future
                                    let future = match Python::with_gil(|py| {
                                        let cb_args = PyTuple::new(py, &[Query { q: async_std::sync::Arc::new(q.unwrap()) }]);
                                        cb_obj.as_ref(py).call1(cb_args).and_then(pyo3_asyncio::async_std::into_future)
                                    }) {
                                        Ok(f) => f,
                                        Err(e) => { warn!("Error calling async queryable callback: {}", e); continue }
                                    };
                                    // await the future (by default callbacks are executed in sequence)
                                    if let Err(e) = future.await {
                                        warn!("Error suring axecution of async queryable callback: {}", e);
                                    }
                                },
                                _ = unregister_rx.recv().fuse() => {
                                    if let Err(e) = zn_quer.close().await {
                                        warn!("Error undeclaring queryable: {}", e);
                                    }
                                    return Ok(())
                                }
                            )
                        }
                    }) {
                        Ok(()) => warn!("Queryable loop running"),
                        Err(e) => warn!("Failed to start Queryable loop: {}", e),
                    }
                })
            });
            Ok(AsyncQueryable {
                unregister_tx,
                loop_handle: Some(loop_handle),
            })
        })
    }

    /// Query data from the matching queryables in the system.
    ///
    /// Replies are collected in a list.
    ///
    /// The *selector* parameter also accepts the following types that can be converted to a :class:`Selector`:
    ///
    /// * **KeyExpr** for a key expression with no value selector
    /// * **int** for a key expression id with no value selector
    /// * **str** for a litteral selector
    ///
    /// This method is a **coroutine**.
    ///
    /// :param selector: The selection of resources to query
    /// :type selector: :class:`Selector`
    /// :param \**kwargs:
    ///    See below
    ///
    /// :Keyword Arguments:
    ///    * **target** (:class:`QueryTarget`) --
    ///      Set the kind of queryables that should be target of this query
    ///    * **consolidation** (:class:`QueryConsolidation`) --
    ///      Set the consolidation mode of the query
    ///    * **local_routing** ( **bool** ) --
    ///      Enable or disable local routing
    ///
    /// :rtype: [:class:`Reply`]
    /// :raise: :class:`ZError`
    ///
    /// :Examples:
    ///
    /// >>> import asyncio, zenoh
    /// >>>
    /// >>> async def main():
    /// >>>    s = await zenoh.async_open()
    /// >>>    replies = await s.get('/key/selector?value_selector')
    /// >>>    for reply in replies:
    /// ...       print("Received : {}".format(reply.sample))
    /// >>>
    /// >>> asyncio.run(main())
    #[pyo3(text_signature = "(self, selector, **kwargs)")]
    #[args(kwargs = "**")]
    fn get<'p>(
        &self,
        selector: &PyAny,
        kwargs: Option<&PyDict>,
        py: Python<'p>,
    ) -> PyResult<&'p PyAny> {
        let s = self.try_clone()?;

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
        }
        .to_owned();
        // note: extract from kwargs here because it's not Send and cannot be moved into future_into_py(py, F)
        let mut target: Option<QueryTarget> = None;
        let mut consolidation: Option<QueryConsolidation> = None;
        let mut local_routing: Option<bool> = None;
        if let Some(kwargs) = kwargs {
            if let Some(arg) = kwargs.get_item("target") {
                target = Some(arg.extract::<QueryTarget>()?);
            }
            if let Some(arg) = kwargs.get_item("consolidation") {
                consolidation = Some(arg.extract::<QueryConsolidation>()?);
            }
            if let Some(arg) = kwargs.get_item("local_routing") {
                local_routing = Some(arg.extract::<bool>()?);
            }
        }

        future_into_py(py, async move {
            let mut getter = s.get(selector);
            if let Some(t) = target {
                getter = getter.target(t.t);
            }
            if let Some(c) = consolidation {
                getter = getter.consolidation(c.c);
            }
            if let Some(lr) = local_routing {
                getter = getter.local_routing(lr);
            }
            let mut reply_rcv = getter.await.map_err(to_pyerr)?;
            let mut replies: Vec<Reply> = Vec::new();

            while let Some(reply) = reply_rcv.next().await {
                replies.push(Reply { r: reply });
            }
            Ok(replies)
        })
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
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh session was closed"))
    }

    #[inline]
    fn try_take(&mut self) -> PyResult<Arc<zenoh::Session>> {
        self.s
            .take()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh session was closed"))
    }
}
