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
use super::types::*;
use crate::{to_pyerr, ZError};
use async_std::sync::channel;
use async_std::task;
use futures::prelude::*;
use futures::select;
use log::warn;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use zenoh::net::{ResourceId, ZInt};

/// A zenoh-net session.
#[pyclass]
pub(crate) struct Session {
    s: Option<zenoh::net::Session>,
}

#[pymethods]
impl Session {
    /// Close the zenoh-net Session.
    fn close(&mut self) -> PyResult<()> {
        let s = self.take()?;
        task::block_on(s.close()).map_err(to_pyerr)
    }

    /// Get informations about the zenoh-net Session.
    ///
    /// :rtype: list of (int, bytes)
    ///
    /// :Example:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.net.open(zenoh.net.config.default())
    /// >>> info = s.info()
    /// >>> for key, value in info:
    /// ...    print("{} : {}".format(zenoh.net.info.to_str(key), value.hex().upper()))
    fn info<'p>(&self, py: Python<'p>) -> PyResult<Vec<(ZInt, &'p PyBytes)>> {
        let s = self.as_ref()?;
        let props = task::block_on(s.info());
        Ok(props
            .iter()
            .map(|(k, v)| (*k, PyBytes::new(py, v.as_slice())))
            .collect())
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
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.net.open(zenoh.net.config.default())
    /// >>> s.write('/resource/name', bytes('value', encoding='utf8'))
    #[text_signature = "(self, resource, payload)"]
    fn write(&self, resource: &PyAny, payload: &[u8]) -> PyResult<()> {
        let s = self.as_ref()?;
        let k = znreskey_of_pyany(resource)?;
        task::block_on(s.write(&k, payload.into())).map_err(to_pyerr)
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
    /// >>> s = zenoh.net.open(zenoh.net.config.default())
    /// >>> rid = s.declare_resource('/resource/name')
    #[text_signature = "(self, resource)"]
    fn declare_resource(&self, resource: &PyAny) -> PyResult<ResourceId> {
        let s = self.as_ref()?;
        let k = znreskey_of_pyany(resource)?;
        task::block_on(s.declare_resource(&k)).map_err(to_pyerr)
    }

    /// Undeclare the *numerical Id/resource key* association previously declared
    /// with :meth:`declare_resource`.
    ///
    /// :param rid: The numerical Id to unmap
    /// :type rid: ResKey
    ///
    /// :Examples:
    ///
    /// >>> import zenoh
    /// >>> s = zenoh.net.open(zenoh.net.config.default())
    /// >>> rid = s.declare_resource('/resource/name')
    /// >>> s.undeclare_resource(rid)
    #[text_signature = "(self, rid)"]
    fn undeclare_resource(&self, rid: ResourceId) -> PyResult<()> {
        let s = self.as_ref()?;
        task::block_on(s.undeclare_resource(rid)).map_err(to_pyerr)
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
    /// >>> s = zenoh.net.open(zenoh.net.config.default())
    /// >>> rid = s.declare_publisher('/resource/name')
    /// >>> s.write('/resource/name', bytes('value', encoding='utf8'))
    #[text_signature = "(self, resource)"]
    fn declare_publisher(&self, resource: &PyAny) -> PyResult<Publisher> {
        let s = self.as_ref()?;
        let k = znreskey_of_pyany(resource)?;
        let zn_pub = task::block_on(s.declare_publisher(&k)).map_err(to_pyerr)?;

        // Note: this is a workaround for pyo3 not supporting lifetime in PyClass. See https://github.com/PyO3/pyo3/issues/502.
        // We extend zenoh::net::Publisher's lifetime to 'static to be wrapped in Publisher PyClass
        let static_zn_pub = unsafe {
            std::mem::transmute::<zenoh::net::Publisher<'_>, zenoh::net::Publisher<'static>>(zn_pub)
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
    /// :rtype: zenoh.net.Subscriber
    ///
    /// :Examples:
    ///
    /// >>> import zenoh, time
    /// >>> from zenoh.net import SubInfo, Reliability, SubMode
    /// >>> def listener(sample):
    /// ...     print("Received : {}".format(sample))
    /// >>>
    /// >>> s = zenoh.net.open(zenoh.net.config.default())
    /// >>> sub_info = SubInfo(Reliability.Reliable, SubMode.Push)
    /// >>> sub = s.declare_subscriber('/resource/name', sub_info, listener)
    /// >>> time.sleep(60)
    #[text_signature = "(self, resource, info, callback)"]
    fn declare_subscriber(
        &self,
        resource: &PyAny,
        info: &SubInfo,
        callback: &PyAny,
    ) -> PyResult<Subscriber> {
        let s = self.as_ref()?;
        let k = znreskey_of_pyany(resource)?;
        let zn_sub = task::block_on(s.declare_subscriber(&k, &info.i)).map_err(to_pyerr)?;
        // Note: workaround to allow moving of zn_sub into the task below.
        // Otherwise, s is moved also, but can't because it doesn't have 'static lifetime.
        let mut static_zn_sub = unsafe {
            std::mem::transmute::<zenoh::net::Subscriber<'_>, zenoh::net::Subscriber<'static>>(
                zn_sub,
            )
        };

        // Note: callback cannot be passed as such in task below because it's not Send
        let cb_obj: Py<PyAny> = callback.into();

        let (undeclare_tx, undeclare_rx) = channel::<ZnSubOps>(8);
        // Note: This is done to ensure that even if the call-back into Python
        // does any blocking call we do not incour the risk of blocking
        // any of the task resolving futures.
        let loop_handle = task::spawn_blocking(move || {
            task::block_on(async move {
                loop {
                    select!(
                        s = static_zn_sub.stream().next().fuse() => {
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
                                    if let Err(e) = static_zn_sub.undeclare().await {
                                        warn!("Error undeclaring subscriber: {}", e);
                                    }
                                    return()
                                },
                                _ => return ()
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
    /// >>> from zenoh.net import Sample, queryable
    /// >>> def callback(query):
    /// ...     print("Received : {}".format(query))
    /// ...     query.reply(Sample('/resource/name', bytes('value', encoding='utf8')))
    /// >>>
    /// >>> s = zenoh.net.open(zenoh.net.config.default())
    /// >>> q = s.declare_queryable('/resource/name', queryable.EVAL, callback)
    /// >>> time.sleep(60)
    #[text_signature = "(self, resource, kind, callback)"]
    fn declare_queryable(
        &self,
        resource: &PyAny,
        kind: ZInt,
        callback: &PyAny,
    ) -> PyResult<Queryable> {
        let s = self.as_ref()?;
        let k = znreskey_of_pyany(resource)?;
        let zn_quer = task::block_on(s.declare_queryable(&k, kind)).map_err(to_pyerr)?;
        // Note: workaround to allow moving of zn_quer into the task below.
        // Otherwise, s is moved also, but can't because it doesn't have 'static lifetime.
        let mut static_zn_quer = unsafe {
            std::mem::transmute::<zenoh::net::Queryable<'_>, zenoh::net::Queryable<'static>>(
                zn_quer,
            )
        };

        // Note: callback cannot be passed as such in task below because it's not Send
        let cb_obj: Py<PyAny> = callback.into();

        let (undeclare_tx, undeclare_rx) = channel::<bool>(1);
        // Note: This is done to ensure that even if the call-back into Python
        // does any blocking call we do not incour the risk of blocking
        // any of the task resolving futures.
        let loop_handle = task::spawn_blocking(move || {
            task::block_on(async move {
                loop {
                    select!(
                        q = static_zn_quer.stream().next().fuse() => {
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
                            if let Err(e) = static_zn_quer.undeclare().await {
                                warn!("Error undeclaring queryable: {}", e);
                            }
                            return()
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
    /// :rtype: Queryable
    ///
    /// :Examples:
    ///
    /// >>> import zenoh, time
    /// >>> from zenoh.net import QueryTarget, queryable
    /// >>> def query_callback(reply):
    /// ...     print("Received : {}".format(reply))
    /// >>>
    /// >>> s = zenoh.net.open(zenoh.net.config.default())
    /// >>> s.query('/resource/name', 'predicate', query_callback)
    /// >>> time.sleep(1)
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
        let mut zn_recv = task::block_on(s.query(
            &k,
            predicate,
            target.unwrap_or_default().t,
            consolidation.unwrap_or_default().c,
        ))
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
            })
        });
        Ok(())
    }
}

impl Session {
    pub(crate) fn new(s: zenoh::net::Session) -> Self {
        Session { s: Some(s) }
    }

    #[inline]
    fn as_ref(&self) -> PyResult<&zenoh::net::Session> {
        self.s
            .as_ref()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh-net session was closed"))
    }

    #[inline]
    fn take(&mut self) -> PyResult<zenoh::net::Session> {
        self.s
            .take()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh-net session was closed"))
    }
}
