//
// Copyright (c) 2024 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use std::{sync::Arc, time::Duration};

use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
    types::{PyDict, PyTuple},
};
use zenoh::{
    payload::Payload,
    prelude::{QoSBuilderTrait, ValueBuilderTrait},
    SessionDeclarations,
};

use crate::{
    config::{Config, ConfigInner, ZenohId},
    encoding::Encoding,
    handlers::{handler_or_default, into_handler, HandlerImpl, IntoHandlerImpl},
    info::SessionInfo,
    key_expr::KeyExpr,
    payload::{into_payload, into_payload_opt},
    publication::{CongestionControl, Priority, Publisher},
    query::{ConsolidationMode, QueryTarget, Reply},
    queryable::{Query, Queryable},
    sample::Sample,
    selector::Selector,
    subscriber::{Reliability, Subscriber},
    utils::{allow_threads, bail, build, opt_wrapper, PySyncResolve},
};

opt_wrapper!(Session, Arc<zenoh::Session>, "Closed session");

#[pymethods]
impl Session {
    fn __enter__<'a, 'py>(this: &'a Bound<'py, Self>) -> PyResult<&'a Bound<'py, Self>> {
        Self::check(this)
    }

    #[pyo3(signature = (*_args, **_kwargs))]
    fn __exit__(
        &mut self,
        py: Python,
        _args: &Bound<PyTuple>,
        _kwargs: Option<&Bound<PyDict>>,
    ) -> PyResult<()> {
        self.close(py)
    }

    fn zid(&self) -> PyResult<ZenohId> {
        Ok(self.get_ref()?.zid().into())
    }

    // TODO HLC

    fn close(&mut self, py: Python) -> PyResult<()> {
        match Arc::try_unwrap(self.take()?) {
            Ok(session) => allow_threads(py, || session.close().py_res_sync()),
            Err(session) => {
                self.0 = Some(session);
                bail!("Cannot close session because it is still borrowed");
            }
        }
    }

    fn undeclare(&self, obj: &Bound<PyAny>) -> PyResult<()> {
        if let Ok(key_expr) = KeyExpr::new(obj) {
            return allow_threads(obj.py(), || {
                self.get_ref()?.undeclare(key_expr.0).py_res_sync()
            });
        }
        bail!("Cannot undeclare {}", obj.get_type().name()?);
    }

    fn config(&self) -> PyResult<Config> {
        Ok(Config(ConfigInner::Notifier(
            self.get_ref()?.config().clone(),
        )))
    }

    fn declare_keyexpr(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::new")] key_expr: KeyExpr,
    ) -> PyResult<KeyExpr> {
        allow_threads(py, || {
            self.get_ref()?.declare_keyexpr(key_expr).py_res_sync()
        })
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (key_expr, payload, *, encoding = None, congestion_control = None, priority = None, express = None))]
    fn put(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::new")] key_expr: KeyExpr,
        #[pyo3(from_py_with = "into_payload")] payload: Payload,
        #[pyo3(from_py_with = "Encoding::opt")] encoding: Option<Encoding>,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
    ) -> PyResult<()> {
        allow_threads(py, || {
            let mut builder = self.get_ref()?.put(key_expr, payload);
            build!(builder, encoding, congestion_control, priority, express);
            builder.py_res_sync()
        })
    }

    #[pyo3(signature = (key_expr, *, congestion_control = None, priority = None, express = None))]
    fn delete(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::new")] key_expr: KeyExpr,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
    ) -> PyResult<()> {
        allow_threads(py, || {
            let mut builder = self.get_ref()?.delete(key_expr);
            build!(builder, congestion_control, priority, express);
            builder.py_res_sync()
        })
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (selector, *, target = None, consolidation = None, timeout = None, congestion_control = None, priority = None, express = None, payload = None, encoding = None, handler = None))]
    fn get(
        &self,
        py: Python,
        #[pyo3(from_py_with = "Selector::new")] selector: Selector,
        target: Option<QueryTarget>,
        consolidation: Option<ConsolidationMode>,
        #[pyo3(from_py_with = "timeout")] timeout: Option<Duration>,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
        #[pyo3(from_py_with = "into_payload_opt")] payload: Option<Payload>,
        #[pyo3(from_py_with = "Encoding::opt")] encoding: Option<Encoding>,
        #[pyo3(from_py_with = "into_handler::<Reply>")] handler: Option<IntoHandlerImpl<Reply>>,
    ) -> PyResult<HandlerImpl<Reply>> {
        let handler = handler_or_default(py, handler);
        py.allow_threads(move || {
            let mut builder = self.get_ref()?.get(selector);
            build!(
                builder,
                target,
                consolidation,
                timeout,
                congestion_control,
                priority,
                express,
                payload,
                encoding,
            );
            builder.with(handler).py_res_sync()
        })
    }

    #[getter]
    fn info(&self) -> PyResult<SessionInfo> {
        Ok(self.get_ref()?.info().into())
    }

    fn declare_subscriber(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::new")] key_expr: KeyExpr,
        reliability: Option<Reliability>,
        #[pyo3(from_py_with = "into_handler::<Sample>")] handler: Option<IntoHandlerImpl<Sample>>,
    ) -> PyResult<Subscriber> {
        let handler = handler_or_default(py, handler);
        allow_threads(py, || {
            let mut builder = self.get_ref()?.declare_subscriber(key_expr);
            build!(builder, reliability);
            builder.with(handler).py_res_sync()
        })
    }

    fn declare_queryable(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::new")] key_expr: KeyExpr,
        complete: Option<bool>,
        #[pyo3(from_py_with = "into_handler::<Query>")] handler: Option<IntoHandlerImpl<Query>>,
    ) -> PyResult<Queryable> {
        let handler = handler_or_default(py, handler);
        allow_threads(py, || {
            let mut builder = self.get_ref()?.declare_queryable(key_expr);
            build!(builder, complete);
            builder.with(handler).py_res_sync()
        })
    }

    #[pyo3(signature = (key_expr, *, congestion_control = None, priority = None, express = None))]
    fn declare_publisher(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::new")] key_expr: KeyExpr,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
    ) -> PyResult<Publisher> {
        allow_threads(py, || {
            let mut builder = self.get_ref()?.declare_publisher(key_expr);
            build!(builder, congestion_control, priority, express);
            builder.py_res_sync()
        })
    }
}

#[pyfunction]
pub(crate) fn open(py: Python, config: Config) -> PyResult<Session> {
    allow_threads(py, || Ok(zenoh::open(config).py_res_sync()?.into_arc()))
}

pub(crate) fn timeout(obj: &Bound<PyAny>) -> PyResult<Option<Duration>> {
    if let Ok(d) = u64::extract_bound(obj) {
        Ok(Some(Duration::new(d, 0)))
    } else if let Ok(d) = f64::extract_bound(obj) {
        Ok(Some(Duration::from_secs_f64(d)))
    } else {
        Err(PyTypeError::new_err("invalid timeout"))
    }
}
