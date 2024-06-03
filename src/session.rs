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
    exceptions::PyValueError,
    prelude::*,
    types::{PyDict, PySet, PyTuple},
};
use zenoh::{
    prelude::{QoSBuilderTrait, ValueBuilderTrait},
    SessionDeclarations,
};

use crate::{
    bytes::ZBytes,
    config::{Config, ConfigInner, ZenohId},
    encoding::Encoding,
    handlers::{handler_or_default, into_handler, HandlerImpl, IntoHandlerImpl},
    info::SessionInfo,
    key_expr::KeyExpr,
    macros::{bail, build, build_with, option_wrapper},
    publication::{CongestionControl, Priority, Publisher},
    query::{ConsolidationMode, QueryTarget, Reply},
    queryable::{Query, Queryable},
    sample::Sample,
    selector::Selector,
    subscriber::{Reliability, Subscriber},
    utils::{wait, MapInto},
};

#[pyclass]
pub(crate) struct Session {
    session: Option<Arc<zenoh::Session>>,
    pool: Py<PySet>,
}

option_wrapper!(Session.session: Arc<zenoh::Session>, "Closed session");

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
    ) -> PyResult<PyObject> {
        self.close(py)?;
        Ok(py.None())
    }

    fn zid(&self) -> PyResult<ZenohId> {
        Ok(self.get_ref()?.zid().into())
    }

    // TODO HLC

    fn close(&mut self, py: Python) -> PyResult<()> {
        for obj in self.pool.bind(py).iter() {
            obj.call_method0("_drop")?;
        }
        // can unwrap because all references have been dropped above
        let session = Arc::try_unwrap(self.take()?).unwrap();
        wait(py, || session.close())
    }

    fn undeclare(&self, obj: &Bound<PyAny>) -> PyResult<()> {
        if let Ok(key_expr) = KeyExpr::from_py(obj) {
            let this = self.get_ref()?;
            return wait(obj.py(), || this.undeclare(key_expr.0));
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
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
    ) -> PyResult<KeyExpr> {
        let this = self.get_ref()?;
        wait(py, || this.declare_keyexpr(key_expr)).map_into()
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (key_expr, payload, *, encoding = None, congestion_control = None, priority = None, express = None))]
    fn put(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        #[pyo3(from_py_with = "ZBytes::from_py")] payload: ZBytes,
        #[pyo3(from_py_with = "Encoding::from_py_opt")] encoding: Option<Encoding>,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
    ) -> PyResult<()> {
        let this = self.get_ref()?;
        let build = build!(
            this.put(key_expr, payload),
            encoding,
            congestion_control,
            priority,
            express
        );
        wait(py, build)
    }

    #[pyo3(signature = (key_expr, *, congestion_control = None, priority = None, express = None))]
    fn delete(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
    ) -> PyResult<()> {
        let this = self.get_ref()?;
        let build = build!(this.delete(key_expr), congestion_control, priority, express);
        wait(py, build)
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (selector, handler = None, *, target = None, consolidation = None, timeout = None, congestion_control = None, priority = None, express = None, payload = None, encoding = None))]
    fn get(
        &self,
        py: Python,
        #[pyo3(from_py_with = "Selector::from_py")] selector: Selector,
        #[pyo3(from_py_with = "into_handler::<Reply>")] handler: Option<IntoHandlerImpl<Reply>>,
        target: Option<QueryTarget>,
        consolidation: Option<ConsolidationMode>,
        #[pyo3(from_py_with = "timeout")] timeout: Option<Duration>,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
        #[pyo3(from_py_with = "ZBytes::from_py_opt")] payload: Option<ZBytes>,
        #[pyo3(from_py_with = "Encoding::from_py_opt")] encoding: Option<Encoding>,
    ) -> PyResult<HandlerImpl<Reply>> {
        let this = self.get_ref()?;
        let build = build_with!(
            handler_or_default(py, handler),
            this.get(selector),
            target,
            consolidation,
            timeout,
            congestion_control,
            priority,
            express,
            payload,
            encoding,
        );
        wait(py, build).map_into()
    }

    #[getter]
    fn info(&self) -> PyResult<SessionInfo> {
        Ok(SessionInfo(Arc::downgrade(self.get_ref()?)))
    }

    #[pyo3(signature = (key_expr, handler = None, reliability = None))]
    fn declare_subscriber(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        #[pyo3(from_py_with = "into_handler::<Sample>")] handler: Option<IntoHandlerImpl<Sample>>,
        reliability: Option<Reliability>,
    ) -> PyResult<Py<Subscriber>> {
        let this = self.get_ref()?;
        let build = build_with!(
            handler_or_default(py, handler),
            this.declare_subscriber(key_expr),
            reliability
        );
        let subscriber = Subscriber {
            subscriber: Some(wait(py, build)?),
            session_pool: self.pool.clone_ref(py),
        };
        let subscriber = Py::new(py, subscriber).unwrap();
        self.pool.bind(py).add(subscriber.clone_ref(py))?;
        Ok(subscriber)
    }

    #[pyo3(signature = (key_expr, handler = None, complete = None))]
    fn declare_queryable(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        #[pyo3(from_py_with = "into_handler::<Query>")] handler: Option<IntoHandlerImpl<Query>>,
        complete: Option<bool>,
    ) -> PyResult<Py<Queryable>> {
        let this = self.get_ref()?;
        let build = build_with!(
            handler_or_default(py, handler),
            this.declare_queryable(key_expr),
            complete
        );
        let queryable = Queryable {
            queryable: Some(wait(py, build)?),
            session_pool: self.pool.clone_ref(py),
        };
        let queryable = Py::new(py, queryable).unwrap();
        self.pool.bind(py).add(queryable.clone_ref(py))?;
        Ok(queryable)
    }

    #[pyo3(signature = (key_expr, *, congestion_control = None, priority = None, express = None))]
    fn declare_publisher(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
    ) -> PyResult<Py<Publisher>> {
        let this = self.get_ref()?;
        let build = build!(
            this.declare_publisher(key_expr),
            congestion_control,
            priority,
            express
        );
        let publisher = Publisher {
            publisher: Some(wait(py, build)?),
            session_pool: self.pool.clone_ref(py),
        };
        let publisher = Py::new(py, publisher).unwrap();
        self.pool.bind(py).add(publisher.clone_ref(py))?;
        Ok(publisher)
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.get_ref()?))
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if self.get_ref().is_ok() {
            Python::with_gil(|gil| self.close(gil)).unwrap();
        }
    }
}

#[pyfunction]
pub(crate) fn open(py: Python, config: Option<Config>) -> PyResult<Session> {
    Ok(Session {
        session: Some(wait(py, || zenoh::open(config.unwrap_or_default()))?.into_arc()),
        pool: PySet::empty_bound(py)?.unbind(),
    })
}

pub(crate) fn timeout(obj: &Bound<PyAny>) -> PyResult<Option<Duration>> {
    if obj.is_none() {
        return Ok(None);
    }
    Duration::try_from_secs_f64(f64::extract_bound(obj)?)
        .map(Some)
        .map_err(|_| PyValueError::new_err("negative timeout"))
}
