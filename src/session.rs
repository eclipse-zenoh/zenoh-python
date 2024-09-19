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
use std::time::Duration;

use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyDict, PyList, PyTuple},
};
use zenoh::Wait;

use crate::{
    bytes::{Encoding, ZBytes},
    config::{Config, ZenohId},
    handlers::{into_handler, HandlerImpl},
    key_expr::KeyExpr,
    macros::{build, with, wrapper},
    pubsub::{Publisher, Subscriber},
    qos::{CongestionControl, Priority, Reliability},
    query::{QueryConsolidation, QueryTarget, Queryable, Reply, Selector},
    time::Timestamp,
    utils::{wait, IntoPython, MapInto},
};

#[pyclass]
pub(crate) struct Session(zenoh::Session);

#[pymethods]
impl Session {
    fn __enter__<'a, 'py>(this: &'a Bound<'py, Self>) -> &'a Bound<'py, Self> {
        this
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
        Ok(self.0.zid().into())
    }

    fn close(&self, py: Python) -> PyResult<()> {
        wait(py, self.0.close())
    }

    fn is_closed(&self) -> bool {
        self.0.is_closed()
    }

    fn undeclare(&self, obj: &Bound<PyAny>) -> PyResult<()> {
        if let Ok(key_expr) = KeyExpr::from_py(obj) {
            return wait(obj.py(), self.0.undeclare(key_expr.0));
        }
        obj.call_method0("undeclare")?;
        Ok(())
    }

    fn new_timestamp(&self) -> Timestamp {
        self.0.new_timestamp().into()
    }

    fn declare_keyexpr(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
    ) -> PyResult<KeyExpr> {
        wait(py, self.0.declare_keyexpr(key_expr)).map_into()
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (key_expr, payload, *, encoding = None, congestion_control = None, priority = None, express = None, attachment = None))]
    fn put(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        #[pyo3(from_py_with = "ZBytes::from_py")] payload: ZBytes,
        #[pyo3(from_py_with = "Encoding::from_py_opt")] encoding: Option<Encoding>,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
        #[pyo3(from_py_with = "ZBytes::from_py_opt")] attachment: Option<ZBytes>,
    ) -> PyResult<()> {
        let build = build!(
            self.0.put(key_expr, payload),
            encoding,
            congestion_control,
            priority,
            express,
            attachment,
        );
        wait(py, build)
    }

    #[pyo3(signature = (key_expr, *, congestion_control = None, priority = None, express = None, attachment = None))]
    fn delete(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
        #[pyo3(from_py_with = "ZBytes::from_py_opt")] attachment: Option<ZBytes>,
    ) -> PyResult<()> {
        let build = build!(
            self.0.delete(key_expr),
            congestion_control,
            priority,
            express,
            attachment,
        );
        wait(py, build)
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (selector, handler = None, *, target = None, consolidation = None, timeout = None, congestion_control = None, priority = None, express = None, payload = None, encoding = None, attachment = None))]
    fn get(
        &self,
        py: Python,
        #[pyo3(from_py_with = "Selector::from_py")] selector: Selector,
        handler: Option<&Bound<PyAny>>,
        target: Option<QueryTarget>,
        #[pyo3(from_py_with = "QueryConsolidation::from_py_opt")] consolidation: Option<
            QueryConsolidation,
        >,
        #[pyo3(from_py_with = "timeout")] timeout: Option<Duration>,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
        #[pyo3(from_py_with = "ZBytes::from_py_opt")] payload: Option<ZBytes>,
        #[pyo3(from_py_with = "Encoding::from_py_opt")] encoding: Option<Encoding>,
        #[pyo3(from_py_with = "ZBytes::from_py_opt")] attachment: Option<ZBytes>,
    ) -> PyResult<HandlerImpl<Reply>> {
        let builder = build!(
            self.0.get(selector),
            target,
            consolidation,
            timeout,
            congestion_control,
            priority,
            express,
            payload,
            encoding,
            attachment,
        )
        .with(into_handler(py, handler)?.0);
        wait(py, builder).map_into()
    }

    #[getter]
    fn info(&self) -> SessionInfo {
        self.0.info().into()
    }

    #[pyo3(signature = (key_expr, handler = None))]
    fn declare_subscriber(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        handler: Option<&Bound<PyAny>>,
    ) -> PyResult<Subscriber> {
        let builder = self.0.declare_subscriber(key_expr);
        with!(builder, py, handler);
        wait(py, builder).map_into()
    }

    #[pyo3(signature = (key_expr, handler = None, *, complete = None))]
    fn declare_queryable(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        handler: Option<&Bound<PyAny>>,
        complete: Option<bool>,
    ) -> PyResult<Queryable> {
        let builder = build!(self.0.declare_queryable(key_expr), complete);
        with!(builder, py, handler);
        wait(py, builder).map_into()
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (key_expr, *, encoding = None, congestion_control = None, priority = None, express = None, reliability = None))]
    fn declare_publisher(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        #[pyo3(from_py_with = "Encoding::from_py_opt")] encoding: Option<Encoding>,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
        reliability: Option<Reliability>,
    ) -> PyResult<Publisher> {
        let builder = build!(
            self.0.declare_publisher(key_expr),
            encoding,
            congestion_control,
            priority,
            express,
            reliability,
        );
        wait(py, builder).map_into()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.0))
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        Python::with_gil(|gil| self.close(gil)).unwrap()
    }
}

#[pyfunction]
pub(crate) fn open(py: Python, config: Config) -> PyResult<Session> {
    wait(py, zenoh::open(config)).map(Session)
}

pub(crate) fn timeout(obj: &Bound<PyAny>) -> PyResult<Option<Duration>> {
    if obj.is_none() {
        return Ok(None);
    }
    Duration::try_from_secs_f64(f64::extract_bound(obj)?)
        .map(Some)
        .map_err(|_| PyValueError::new_err("negative timeout"))
}

wrapper!(zenoh::session::SessionInfo);

#[pymethods]
impl SessionInfo {
    fn zid(&self, py: Python) -> ZenohId {
        py.allow_threads(|| self.0.zid().wait()).into()
    }

    fn routers_zid<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let list = PyList::empty_bound(py);
        for zid in py.allow_threads(|| self.0.routers_zid().wait()) {
            list.append(zid.into_pyobject(py))?;
        }
        Ok(list)
    }

    fn peers_zid<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let list = PyList::empty_bound(py);
        for zid in py.allow_threads(|| self.0.peers_zid().wait()) {
            list.append(zid.into_pyobject(py))?;
        }
        Ok(list)
    }

    // TODO __repr__
}
