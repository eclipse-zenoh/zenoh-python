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
    QueryConsolidation, QueryTarget, Reliability, Reply, Sample, SubMode,
};
use super::{to_pyerr, ZError};
use async_std::sync::Arc;
use futures::prelude::*;
use log::warn;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyTuple};
use pyo3_asyncio::async_std::future_into_py;
use std::collections::HashMap;
use zenoh::prelude::r#async::AsyncResolve;
use zenoh::prelude::{KeyExpr as ZKeyExpr, *};

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

    /// Returns the identifier for this session.
    ///
    /// :raise: :class:`ZError`
    ///
    /// :type: **str**
    #[getter]
    fn id(&self) -> PyResult<String> {
        let s = self.try_ref()?;
        Ok(s.id())
    }

    /// Close the zenoh Session.
    pub fn close<'p>(&mut self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let s = self.try_take()?;
        match Arc::try_unwrap(s) {
            Ok(s) => future_into_py(py, s.close().res().map_err(to_pyerr)),
            Err(_) => Err(PyErr::new::<exceptions::PyValueError, _>(
                "Failed to close Session: not owner of the last reference",
            )),
        }
    }

    /// Get informations about the zenoh Session.
    ///
    /// This method is a **coroutine**.
    ///
    /// :rtype: **dict[str, str]**
    ///
    /// :raise: :class:`ZError`
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
        todo!()
    }

    /// Put data.
    ///
    /// This method is a **coroutine**.
    ///
    /// :param key_expr: The key expression matching resources to write
    /// :type key_expr: a :class:`KeyExpr` or any type convertible to a :class:`KeyExpr`
    ///                 (see its constructor's accepted parameters)
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
        let v = zvalue_of_pyany(value)?;
        let mut builder = s.put(k, v);
        if let Some(kwargs) = kwargs {
            if let Some(arg) = kwargs.get_item("encoding") {
                builder = builder.encoding(arg.extract::<Encoding>()?.e);
            }
            if let Some(arg) = kwargs.get_item("kind") {
                builder = builder.kind(arg.extract::<SampleKind>()?.kind);
            }
            if let Some(arg) = kwargs.get_item("congestion_control") {
                builder = builder.congestion_control(arg.extract::<CongestionControl>()?.cc);
            }
            if let Some(arg) = kwargs.get_item("priority") {
                builder = builder.priority(arg.extract::<Priority>()?.p);
            }
            if let Some(arg) = kwargs.get_item("local_routing") {
                builder = builder.local_routing(arg.extract::<bool>()?);
            }
        }
        future_into_py(py, builder.res().map_err(to_pyerr))
    }

    /// Delete data.
    ///
    /// This method is a **coroutine**.
    ///
    /// :param key_expr: The key expression matching resources to delete
    /// :type key_expr: a :class:`KeyExpr` or any type convertible to a :class:`KeyExpr`
    ///                 (see its constructor's accepted parameters)
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
        let mut builder = s.delete(k);
        if let Some(kwargs) = kwargs {
            if let Some(arg) = kwargs.get_item("congestion_control") {
                builder = builder.congestion_control(arg.extract::<CongestionControl>()?.cc);
            }
            if let Some(arg) = kwargs.get_item("priority") {
                builder = builder.priority(arg.extract::<Priority>()?.p);
            }
            if let Some(arg) = kwargs.get_item("local_routing") {
                builder = builder.local_routing(arg.extract::<bool>()?);
            }
        }
        future_into_py(py, builder.res().map_err(to_pyerr))
    }

    /// Associate a numerical Id with the given key expression.
    ///
    /// This numerical Id will be used on the network to save bandwidth and
    /// ease the retrieval of the concerned resource in the routing tables.
    ///
    /// This method is a **coroutine**.
    ///
    /// :param key_expr: The key expression to map to a numerical Id
    /// :type key_expr: a :class:`KeyExpr` or any type convertible to a :class:`KeyExpr`
    ///                 (see its constructor's accepted parameters)
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
        future_into_py(py, async move {
            s.declare_expr(k).res().map_err(to_pyerr).await
        })
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
        future_into_py(py, async move {
            s.undeclare_expr(rid).res().map_err(to_pyerr).await
        })
    }

    /// Declare a publication for the given key expression.
    ///
    /// Written expressions that match the given key expression will only be sent on the network
    /// if matching subscribers exist in the system.
    ///
    /// This method is a **coroutine**.
    ///
    /// :param key_expr: The key expression to publish
    /// :type key_expr: a :class:`KeyExpr` or any type convertible to a :class:`KeyExpr`
    ///                 (see its constructor's accepted parameters)
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
            s.declare_publication(k).res().map_err(to_pyerr).await
        })
    }

    /// Undeclare the publication previously declared with :meth:`declare_publication`.
    ///
    /// :param key_expr: The same key expression that was used to declare the publication
    /// :type key_expr: a :class:`KeyExpr` or any type convertible to a :class:`KeyExpr`
    ///                 (see its constructor's accepted parameters)
    /// :raise: :class:`ZError`
    #[pyo3(text_signature = "(self, key_expr)")]
    fn undeclare_publication<'p>(&self, key_expr: &PyAny, py: Python<'p>) -> PyResult<&'p PyAny> {
        let s = self.try_clone()?;
        let k = zkey_expr_of_pyany(key_expr)?.to_owned();
        future_into_py(py, async move {
            s.undeclare_publication(k).res().map_err(to_pyerr).await
        })
    }

    /// Create an AsyncSubscriber for the given key expression.
    ///
    /// This method is a **coroutine**.
    ///
    /// :param key_expr: The key expression to subscribe
    /// :type key_expr: a :class:`KeyExpr` or any type convertible to a :class:`KeyExpr`
    ///                 (see its constructor's accepted parameters)
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
        let mut builder = s.subscribe(&k);
        if let Some(kwargs) = kwargs {
            if let Some(any) = kwargs.get_item("reliability") {
                builder = builder.reliability(any.extract::<Reliability>()?.r);
            }
            if let Some(any) = kwargs.get_item("mode") {
                builder = builder.mode(any.extract::<SubMode>()?.m);
            }
            if let Some(any) = kwargs.get_item("period") {
                builder = builder.period(Some(any.extract::<Period>()?.p));
            }
            if let Some(any) = kwargs.get_item("local") {
                if any.extract::<bool>()? {
                    builder = builder.local();
                }
            }
        }

        // Note: PyAny callback object cannot be passed as such in Rust callback below because it's not Send
        let cb_obj: Py<PyAny> = callback.into();
        let cbbuilder = builder.callback(move |s| {
            // Note: clone cb_obj required since it's moved into the closure below
            let cb_obj = cb_obj.clone();
            Python::with_gil(|py| {
                // Run a Python event loop to run the coroutine Python callback
                if let Err(e) = pyo3_asyncio::async_std::run(py, async move {
                    // call the async callback and transform the resulting Python awaitable into a Rust future
                    match Python::with_gil(|py| {
                        let cb_args = PyTuple::new(py, &[Sample { s }]);
                        cb_obj
                            .as_ref(py)
                            .call1(cb_args)
                            .and_then(pyo3_asyncio::async_std::into_future)
                    }) {
                        Ok(f) => f.await,
                        Err(e) => Err(e),
                    }
                }) {
                    warn!("Error calling async subscriber callback: {}", e);
                }
            })
        });

        future_into_py(
            py,
            cbbuilder
                .res()
                .map(|result| result.map(|sub| AsyncSubscriber { inner: Some(sub) }))
                .map_err(to_pyerr),
        )
    }

    /// Create an AsyncQueryable for the given key expression.
    ///
    /// This method is a **coroutine**.
    ///
    /// :param key_expr: The key expression the Queryable will reply to
    /// :type key_expr: a :class:`KeyExpr` or any type convertible to a :class:`KeyExpr`
    ///                 (see its constructor's accepted parameters)
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
        let mut builder = s.queryable(k);
        if let Some(kwargs) = kwargs {
            if let Some(any) = kwargs.get_item("complete") {
                builder = builder.complete(any.extract::<bool>()?);
            }
        }

        // Note: PyAny callback object cannot be passed as such in Rust callback below because it's not Send
        let cb_obj: Py<PyAny> = callback.into();
        let cbbuilder = builder.callback(move |q| {
            // Note: clone cb_obj required since it's moved into the closure below
            let cb_obj = cb_obj.clone();
            Python::with_gil(|py| {
                // Run a Python event loop to run the coroutine Python callback
                if let Err(e) = pyo3_asyncio::async_std::run(py, async move {
                    // call the async callback and transform the resulting Python awaitable into a Rust future
                    match Python::with_gil(|py| {
                        let cb_args = PyTuple::new(
                            py,
                            &[Query {
                                q: async_std::sync::Arc::new(q),
                            }],
                        );
                        cb_obj
                            .as_ref(py)
                            .call1(cb_args)
                            .and_then(pyo3_asyncio::async_std::into_future)
                    }) {
                        Ok(f) => f.await,
                        Err(e) => Err(e),
                    }
                }) {
                    warn!("Error calling async queryable callback: {}", e);
                }
            })
        });

        future_into_py(
            py,
            cbbuilder
                .res()
                .map(|result| result.map(|quer| AsyncQueryable { inner: Some(quer) }))
                .map_err(to_pyerr),
        )
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

        let mut builder = s.get(selector);
        if let Some(kwargs) = kwargs {
            if let Some(any) = kwargs.get_item("target") {
                builder = builder.target(any.extract::<QueryTarget>()?.t);
            }
            if let Some(any) = kwargs.get_item("consolidation") {
                builder = builder.consolidation(any.extract::<QueryConsolidation>()?.c);
            }
            if let Some(any) = kwargs.get_item("local_routing") {
                builder = builder.local_routing(any.extract::<bool>()?);
            }
        }
        let fut = builder.res().map_err(to_pyerr);
        future_into_py(py, async move {
            // let reply_rcv = builder.res().await.map_err(to_pyerr)?;
            let reply_rcv = fut.await?;
            let mut replies: Vec<Reply> = Vec::new();

            while let Ok(reply) = reply_rcv.recv_async().await {
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
    fn try_ref(&self) -> PyResult<&Arc<zenoh::Session>> {
        self.s
            .as_ref()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh session was closed"))
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
