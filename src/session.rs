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
    resolve::{resolve, Resolve},
    sample::Sample,
    selector::Selector,
    subscriber::{Reliability, Subscriber},
    utils::{bail, build, build_with, opt_wrapper, IntoPython},
};

opt_wrapper!(Session, Arc<zenoh::Session>, "Closed session");

impl IntoPython for zenoh::Session {
    type Into = Session;

    fn into_python(self) -> Self::Into {
        self.into_arc().into()
    }
}

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
        self.close(py)?.wait(py)
    }

    fn zid(&self) -> PyResult<ZenohId> {
        Ok(self.get_ref()?.zid().into())
    }

    // TODO HLC

    fn close(&mut self, py: Python) -> PyResult<Resolve> {
        match Arc::try_unwrap(self.take()?) {
            Ok(session) => resolve(py, || session.close()),
            Err(session) => {
                self.0 = Some(session);
                bail!("Cannot close session because it is still borrowed");
            }
        }
    }

    fn undeclare(&self, obj: &Bound<PyAny>) -> PyResult<Resolve> {
        if let Ok(key_expr) = KeyExpr::from_py(obj) {
            let this = self.get_ref()?;
            return resolve(obj.py(), || this.undeclare(key_expr.0));
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
    ) -> PyResult<Resolve<KeyExpr>> {
        let this = self.get_ref()?;
        resolve(py, || this.declare_keyexpr(key_expr))
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (key_expr, payload, *, encoding = None, congestion_control = None, priority = None, express = None))]
    fn put(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        #[pyo3(from_py_with = "into_payload")] payload: Payload,
        #[pyo3(from_py_with = "Encoding::from_py_opt")] encoding: Option<Encoding>,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
    ) -> PyResult<Resolve> {
        let this = self.get_ref()?;
        let build = build!(
            this.put(key_expr, payload),
            encoding,
            congestion_control,
            priority,
            express
        );
        resolve(py, build)
    }

    #[pyo3(signature = (key_expr, *, congestion_control = None, priority = None, express = None))]
    fn delete(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
    ) -> PyResult<Resolve> {
        let this = self.get_ref()?;
        let build = build!(this.delete(key_expr), congestion_control, priority, express);
        resolve(py, build)
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
        #[pyo3(from_py_with = "into_payload_opt")] payload: Option<Payload>,
        #[pyo3(from_py_with = "Encoding::from_py_opt")] encoding: Option<Encoding>,
    ) -> PyResult<Resolve<HandlerImpl<Reply>>> {
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
        resolve(py, build)
    }

    #[getter]
    fn info(&self) -> PyResult<SessionInfo> {
        Ok(self.get_ref()?.info().into())
    }

    #[pyo3(signature = (key_expr, handler = None, reliability = None))]
    fn declare_subscriber(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        #[pyo3(from_py_with = "into_handler::<Sample>")] handler: Option<IntoHandlerImpl<Sample>>,
        reliability: Option<Reliability>,
    ) -> PyResult<Resolve<Subscriber>> {
        let this = self.get_ref()?;
        let build = build_with!(
            handler_or_default(py, handler),
            this.declare_subscriber(key_expr),
            reliability
        );
        resolve(py, build)
    }

    #[pyo3(signature = (key_expr, handler = None, complete = None))]
    fn declare_queryable(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        #[pyo3(from_py_with = "into_handler::<Query>")] handler: Option<IntoHandlerImpl<Query>>,
        complete: Option<bool>,
    ) -> PyResult<Resolve<Queryable>> {
        let this = self.get_ref()?;
        let build = build_with!(
            handler_or_default(py, handler),
            this.declare_queryable(key_expr),
            complete
        );
        resolve(py, build)
    }

    #[pyo3(signature = (key_expr, *, congestion_control = None, priority = None, express = None))]
    fn declare_publisher(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
    ) -> PyResult<Resolve<Publisher>> {
        let this = self.get_ref()?;
        let build = build!(
            this.declare_publisher(key_expr),
            congestion_control,
            priority,
            express
        );
        resolve(py, build)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyfunction]
pub(crate) fn open(py: Python, config: Option<Config>) -> PyResult<Resolve<Session>> {
    resolve(py, || zenoh::open(config.unwrap_or_default()))
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
