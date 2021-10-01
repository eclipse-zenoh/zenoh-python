use std::collections::HashMap;

use super::data_kind::SampleKind;
use super::encoding::Encoding;
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
    znreskey_of_pyany, CongestionControl, Publisher, Query, QueryConsolidation, QueryTarget,
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
use pyo3::types::{IntoPyDict, PyList, PyTuple};
use zenoh::prelude::KeyedSelector;
use zenoh::prelude::{ResourceId, ZFuture, ZInt};

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

    /// Write data.
    ///
    /// The *resource* parameter also accepts the following types that can be converted to a :class:`ResKey`:
    ///
    /// * **int** for a ``ResKey.Rid(int)``
    /// * **str** for a ``ResKey.RName(str)``
    /// * **(int, str)** for a ``ResKey.RIdWithSuffix(int, str)``
    ///
    /// :param resource: The resource key to write
    /// :type resource: ResKey
    /// :param payload: The value to write
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
    /// >>> s.write('/resource/name', bytes('value', encoding='utf8'))
    #[text_signature = "(self, resource, payload)"]
    pub fn write(
        &self,
        resource: &PyAny,
        payload: &[u8],
        encoding: Option<Encoding>,
        kind: Option<SampleKind>,
        congestion_control: Option<CongestionControl>,
    ) -> PyResult<()> {
        let s = self.as_ref()?;
        let k = znreskey_of_pyany(resource)?;
        let encoding = encoding.unwrap_or_default();
        let value = zenoh::prelude::Value::from(payload).encoding(encoding.into());
        s.put(k, value)
            .kind(kind.unwrap_or_default().kind)
            .congestion_control(congestion_control.unwrap_or_default().cc)
            .wait()
            .map_err(to_pyerr)
    }

    /// Associate a numerical Id with the given resource key.
    ///
    /// This numerical Id will be used on the network to save bandwidth and
    /// ease the retrieval of the concerned resource in the routing tables.
    ///
    /// The *resource* parameter also accepts the following types that can be converted to a :class:`ResKey`:
    ///
    /// * **int** for a ``ResKey.Rid(int)``
    /// * **str** for a ``ResKey.RName(str)``
    /// * **(int, str)** for a ``ResKey.RIdWithSuffix(int, str)``
    ///
    /// :param resource: The resource key to map to a numerical Id
    /// :type resource: ResKey
    /// :rtype: int
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open({})
    /// >>> rid = s.register_resource('/resource/name')
    #[text_signature = "(self, resource)"]
    pub fn register_resource(&self, resource: &PyAny) -> PyResult<ResourceId> {
        let s = self.as_ref()?;
        let k = znreskey_of_pyany(resource)?;
        s.register_resource(&k).wait().map_err(to_pyerr)
    }

    /// Undeclare the *numerical Id/resource key* association previously declared
    /// with :meth:`register_resource`.
    ///
    /// :param rid: The numerical Id to unmap
    /// :type rid: ResKey
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open({})
    /// >>> rid = s.register_resource('/resource/name')
    /// >>> s.unregister_resource(rid)
    #[text_signature = "(self, rid)"]
    pub fn unregister_resource(&self, rid: ResourceId) -> PyResult<()> {
        let s = self.as_ref()?;
        s.unregister_resource(rid).wait().map_err(to_pyerr)
    }

    /// Declare a Publisher for the given resource key.
    ///
    /// Written resources that match the given key will only be sent on the network
    /// if matching subscribers exist in the system.
    ///
    /// The *resource* parameter also accepts the following types that can be converted to a :class:`ResKey`:
    ///
    /// * **int** for a ``ResKey.Rid(int)``
    /// * **str** for a ``ResKey.RName(str)``
    /// * **(int, str)** for a ``ResKey.RIdWithSuffix(int, str)``
    ///
    /// :param resource: The resource key to publish
    /// :type resource: ResKey
    /// :rtype: Publisher
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.open({})
    /// >>> rid = s.publishing('/resource/name')
    /// >>> s.write('/resource/name', bytes('value', encoding='utf8'))
    #[text_signature = "(self, resource)"]
    fn publishing(&self, resource: &PyAny) -> PyResult<Publisher> {
        let s = self.as_ref()?;
        let k = znreskey_of_pyany(resource)?;
        let zn_pub = s.publishing(&k).wait().map_err(to_pyerr)?;

        // Note: this is a workaround for pyo3 not supporting lifetime in PyClass. See https://github.com/PyO3/pyo3/issues/502.
        // We extend zenoh::publisher::Publisher's lifetime to 'static to be wrapped in Publisher PyClass
        let static_zn_pub = unsafe {
            std::mem::transmute::<
                zenoh::publisher::Publisher<'_>,
                zenoh::publisher::Publisher<'static>,
            >(zn_pub)
        };
        Ok(Publisher {
            p: Some(static_zn_pub),
        })
    }

    /// Declare a Subscxriber for the given resource key.
    ///
    /// The *resource* parameter also accepts the following types that can be converted to a :class:`ResKey`:
    ///
    /// * **int** for a ``ResKey.Rid(int)``
    /// * **str** for a ``ResKey.RName(str)``
    /// * **(int, str)** for a ``ResKey.RIdWithSuffix(int, str)``
    ///
    /// :param resource: The resource key to subscribe
    /// :type resource: ResKey
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
    /// >>> sub = s.subscribe('/resource/name', sub_info, lambda sample:
    /// ...     print("Received : {}".format(sample)))
    /// >>> time.sleep(60)
    #[text_signature = "(self, resource, callback, reliability, mode)"]
    fn subscribe(
        &self,
        resource: &PyAny,
        callback: &PyAny,
        reliability: Option<Reliability>,
        mode: Option<SubMode>,
    ) -> PyResult<Subscriber> {
        let s = self.as_ref()?;
        let k = znreskey_of_pyany(resource)?;
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

        let (undeclare_tx, undeclare_rx) = bounded::<ZnSubOps>(8);
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
                        op = undeclare_rx.recv().fuse() => {
                            match op {
                                Ok(ZnSubOps::Pull) => {
                                    if let Err(e) = static_zn_sub.pull().await {
                                        warn!("Error pulling the subscriber: {}", e);
                                    }
                                },
                                Ok(ZnSubOps::Undeclare) => {
                                    if let Err(e) = static_zn_sub.unregister().await {
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
            undeclare_tx,
            loop_handle: Some(loop_handle),
        })
    }

    /// Declare a Queryable for the given resource key.
    ///
    /// The *resource* parameter also accepts the following types that can be converted to a :class:`ResKey`:
    ///
    /// * **int** for a ``ResKey.Rid(int)``
    /// * **str** for a ``ResKey.RName(str)``
    /// * **(int, str)** for a ``ResKey.RIdWithSuffix(int, str)``
    ///
    /// :param resource: The resource key the Queryable will reply to
    /// :type resource: ResKey
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
    /// ...     query.reply(Sample('/resource/name', bytes('value', encoding='utf8')))
    /// >>>
    /// >>> s = zenoh.open({})
    /// >>> q = s.register_queryable('/resource/name', queryable.EVAL, callback)
    /// >>> time.sleep(60)
    #[text_signature = "(self, resource, kind, callback)"]
    fn register_queryable(
        &self,
        resource: &PyAny,
        kind: ZInt,
        callback: &PyAny,
    ) -> PyResult<Queryable> {
        let s = self.as_ref()?;
        let k = znreskey_of_pyany(resource)?;
        let zn_quer = s
            .register_queryable(k)
            .kind(kind)
            .wait()
            .map_err(to_pyerr)?;
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

        let (undeclare_tx, undeclare_rx) = bounded::<bool>(1);
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
                        _ = undeclare_rx.recv().fuse() => {
                            if let Err(e) = zn_quer.unregister().await {
                                warn!("Error undeclaring queryable: {}", e);
                            }
                            return
                        }
                    )
                }
            })
        });
        Ok(Queryable {
            undeclare_tx,
            loop_handle: Some(loop_handle),
        })
    }

    /// Query data from the matching queryables in the system.
    ///
    /// The replies are provided by calling the provided ``callback`` for each reply.
    /// The ``callback`` is called a last time with ``None`` when the query is complete.
    ///
    /// The *resource* parameter also accepts the following types that can be converted to a :class:`ResKey`:
    ///
    /// * **int** for a ``ResKey.Rid(int)``
    /// * **str** for a ``ResKey.RName(str)``
    /// * **(int, str)** for a ``ResKey.RIdWithSuffix(int, str)``
    ///
    /// :param resource: The resource key to query
    /// :type resource: ResKey
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
    /// >>> s.query('/resource/name', 'predicate', lambda reply:
    /// ...    print("Received : {}".format(
    /// ...        reply.data if reply is not None else "FINAL")))
    #[text_signature = "(self, resource, predicate, callback, target=None, consolidation=None)"]
    fn query(
        &self,
        resource: &PyAny,
        predicate: &str,
        callback: &PyAny,
        target: Option<QueryTarget>,
        consolidation: Option<QueryConsolidation>,
    ) -> PyResult<()> {
        let s = self.as_ref()?;
        let k = znreskey_of_pyany(resource)?;
        let mut zn_recv = s
            .get(KeyedSelector::from(k).with_value_selector(predicate))
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
    /// The *resource* parameter also accepts the following types that can be converted to a :class:`ResKey`:
    ///
    /// * **int** for a ``ResKey.Rid(int)``
    /// * **str** for a ``ResKey.RName(str)``
    /// * **(int, str)** for a ``ResKey.RIdWithSuffix(int, str)``
    ///
    /// :param resource: The resource key to query
    /// :type resource: ResKey
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
    /// >>> replies = s.query_collect('/resource/name', 'predicate')
    /// >>> for reply in replies:
    /// ...    print("Received : {}".format(reply.data))
    #[text_signature = "(self, resource, predicate, target=None, consolidation=None)"]
    fn query_collect(
        &self,
        resource: &PyAny,
        predicate: &str,
        target: Option<QueryTarget>,
        consolidation: Option<QueryConsolidation>,
    ) -> PyResult<Py<PyList>> {
        let s = self.as_ref()?;
        let k = znreskey_of_pyany(resource)?;
        task::block_on(async {
            let mut replies = s
                .get(KeyedSelector::from(k).with_value_selector(predicate))
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
