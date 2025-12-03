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
use std::collections::HashMap;

use pyo3::{
    prelude::*,
    types::{PyDict, PyIterator, PyList, PyTuple, PyType},
    IntoPyObjectExt,
};

use crate::{
    bytes::{Encoding, ZBytes},
    cancellation::CancellationToken,
    handlers::{into_handler, HandlerImpl},
    key_expr::KeyExpr,
    macros::{build, downcast_or_new, enum_mapper, option_wrapper, wrapper},
    matching::{MatchingListener, MatchingStatus},
    qos::{CongestionControl, Priority},
    sample::SourceInfo,
    session::EntityGlobalId,
    time::Timestamp,
    utils::{generic, wait, IntoPyResult, IntoPython, IntoRust, MapInto},
};

enum_mapper!(zenoh::query::QueryTarget: u8 {
    BestMatching,
    All,
    AllComplete,
});

#[pymethods]
impl QueryTarget {
    #[classattr]
    const DEFAULT: Self = Self::BestMatching;
}

enum_mapper!(zenoh::query::ConsolidationMode: u8 {
    Auto,
    None,
    Monotonic,
    Latest,
});

#[pymethods]
impl ConsolidationMode {
    #[classattr]
    const DEFAULT: Self = Self::Auto;
}

wrapper!(zenoh::query::QueryConsolidation: Clone);
downcast_or_new!(QueryConsolidation => ConsolidationMode);

#[pymethods]
impl QueryConsolidation {
    #[classattr]
    const AUTO: Self = Self(zenoh::query::QueryConsolidation::AUTO);
    #[classattr]
    const DEFAULT: Self = Self(zenoh::query::QueryConsolidation::DEFAULT);

    #[new]
    fn new(mode: Option<ConsolidationMode>) -> Self {
        let Some(mode) = mode else {
            return Self::DEFAULT;
        };
        Self(mode.into_rust().into())
    }

    fn mode(&self) -> ConsolidationMode {
        self.0.mode().into()
    }
}

option_wrapper!(zenoh::query::Query, "Dropped query");

#[pymethods]
impl Query {
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
        self.drop();
        Ok(py.None())
    }

    #[getter]
    fn selector(&self) -> PyResult<Selector> {
        Ok(self.get_ref()?.selector().into_owned().into())
    }

    #[getter]
    fn key_expr(&self) -> PyResult<KeyExpr> {
        Ok(self.get_ref()?.key_expr().clone().into_owned().into())
    }

    #[getter]
    fn parameters(&self) -> PyResult<Parameters> {
        Ok(self.get_ref()?.parameters().clone().into_owned().into())
    }

    #[getter]
    fn payload(&self) -> PyResult<Option<ZBytes>> {
        Ok(self.get_ref()?.payload().cloned().map_into())
    }

    #[getter]
    fn encoding(&self) -> PyResult<Option<Encoding>> {
        Ok(self.get_ref()?.encoding().cloned().map_into())
    }

    #[getter]
    fn attachment(&self) -> PyResult<Option<ZBytes>> {
        Ok(self.get_ref()?.attachment().cloned().map_into())
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (key_expr, payload, *, encoding = None, congestion_control = None, priority = None, express = None, attachment = None, timestamp = None))]
    fn reply(
        &self,
        py: Python,
        #[pyo3(from_py_with = KeyExpr::from_py)] key_expr: KeyExpr,
        #[pyo3(from_py_with = ZBytes::from_py)] payload: ZBytes,
        #[pyo3(from_py_with = Encoding::from_py_opt)] encoding: Option<Encoding>,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
        #[pyo3(from_py_with = ZBytes::from_py_opt)] attachment: Option<ZBytes>,
        timestamp: Option<Timestamp>,
    ) -> PyResult<()> {
        let build = build!(
            self.get_ref()?.reply(key_expr, payload),
            encoding,
            congestion_control,
            priority,
            express,
            attachment,
            timestamp,
        );
        wait(py, build)
    }

    #[pyo3(signature = (payload, *, encoding = None))]
    fn reply_err(
        &self,
        py: Python,
        #[pyo3(from_py_with = ZBytes::from_py)] payload: ZBytes,
        #[pyo3(from_py_with = Encoding::from_py_opt)] encoding: Option<Encoding>,
    ) -> PyResult<()> {
        let build = build!(self.get_ref()?.reply_err(payload), encoding);
        wait(py, build)
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (key_expr, *, congestion_control = None, priority = None, express = None, attachment = None, timestamp = None))]
    fn reply_del(
        &self,
        py: Python,
        #[pyo3(from_py_with = KeyExpr::from_py)] key_expr: KeyExpr,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
        #[pyo3(from_py_with = ZBytes::from_py_opt)] attachment: Option<ZBytes>,
        timestamp: Option<Timestamp>,
    ) -> PyResult<()> {
        let build = build!(
            self.get_ref()?.reply_del(key_expr),
            congestion_control,
            priority,
            express,
            attachment,
            timestamp,
        );
        wait(py, build)
    }

    #[getter]
    fn source_info(&self) -> PyResult<Option<SourceInfo>> {
        Ok(self.get_ref()?.source_info().cloned().map_into())
    }

    fn drop(&mut self) {
        Python::with_gil(|gil| gil.allow_threads(|| drop(self.0.take())));
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.get_ref()?))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.get_ref()?))
    }
}

wrapper!(zenoh::query::Reply);

#[pymethods]
impl Reply {
    #[getter]
    fn result(&self, py: Python) -> PyObject {
        match self.0.result() {
            Ok(sample) => sample.clone().into_pyobject(py),
            Err(value) => value.clone().into_pyobject(py),
        }
    }

    #[getter]
    fn ok(&self, py: Python) -> PyObject {
        match self.0.result() {
            Ok(sample) => sample.clone().into_pyobject(py),
            _ => py.None(),
        }
    }

    #[getter]
    fn err(&self, py: Python) -> PyObject {
        match self.0.result() {
            Err(value) => value.clone().into_pyobject(py),
            _ => py.None(),
        }
    }

    #[getter]
    fn replier_id(&self) -> Option<EntityGlobalId> {
        self.0.replier_id().map_into()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

wrapper!(zenoh::query::ReplyError: Clone);

#[pymethods]
impl ReplyError {
    #[getter]
    fn payload(&self) -> ZBytes {
        self.0.payload().clone().into()
    }

    #[getter]
    fn encoding(&self) -> Encoding {
        self.0.encoding().clone().into()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

option_wrapper!(
    zenoh::query::Queryable<HandlerImpl<Query>>,
    "Undeclared queryable"
);

#[pymethods]
impl Queryable {
    #[classmethod]
    fn __class_getitem__(cls: &Bound<PyType>, args: &Bound<PyAny>) -> PyObject {
        generic(cls, args)
    }

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
        self.undeclare(py)?;
        Ok(py.None())
    }

    #[getter]
    fn id(&self) -> PyResult<EntityGlobalId> {
        Ok(self.get_ref()?.id().into())
    }

    #[getter]
    fn key_expr(&self) -> PyResult<KeyExpr> {
        Ok(self.get_ref()?.key_expr().clone().into())
    }

    #[getter]
    fn handler(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.handler().into_py_any(py)
    }

    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.handler().try_recv(py)
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.handler().recv(py)
    }

    fn undeclare(&mut self, py: Python) -> PyResult<()> {
        wait(py, self.take()?.undeclare())
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        self.handler(py)?.bind(py).try_iter()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.get_ref()?))
    }
}

option_wrapper!(zenoh::query::Querier<'static>, "Undeclared querier");

#[pymethods]
impl Querier {
    #[classmethod]
    fn __class_getitem__(cls: &Bound<PyType>, args: &Bound<PyAny>) -> PyObject {
        generic(cls, args)
    }

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
        self.undeclare(py)?;
        Ok(py.None())
    }

    #[getter]
    fn id(&self) -> PyResult<EntityGlobalId> {
        Ok(self.get_ref()?.id().into())
    }

    #[getter]
    fn key_expr(&self) -> PyResult<KeyExpr> {
        Ok(self.get_ref()?.key_expr().clone().into())
    }

    #[getter]
    fn matching_status(&self, py: Python) -> PyResult<MatchingStatus> {
        Ok(wait(py, self.get_ref()?.matching_status())?.into())
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (handler = None, *, parameters = None, payload = None, encoding = None, attachment = None, source_info = None, cancellation_token = None))]
    fn get(
        &self,
        py: Python,
        handler: Option<&Bound<PyAny>>,
        #[pyo3(from_py_with = Parameters::from_py_opt)] parameters: Option<Parameters>,
        #[pyo3(from_py_with = ZBytes::from_py_opt)] payload: Option<ZBytes>,
        #[pyo3(from_py_with = Encoding::from_py_opt)] encoding: Option<Encoding>,
        #[pyo3(from_py_with = ZBytes::from_py_opt)] attachment: Option<ZBytes>,
        source_info: Option<SourceInfo>,
        cancellation_token: Option<CancellationToken>,
    ) -> PyResult<HandlerImpl<Reply>> {
        let this = self.get_ref()?;
        let (handler, _) = into_handler(py, handler, cancellation_token.as_ref())?;
        let builder = build!(
            this.get(),
            parameters,
            payload,
            encoding,
            attachment,
            source_info,
            cancellation_token
        );
        wait(py, builder.with(handler)).map_into()
    }

    #[pyo3(signature = (handler = None))]
    fn declare_matching_listener(
        &self,
        py: Python,
        handler: Option<&Bound<PyAny>>,
    ) -> PyResult<MatchingListener> {
        let (handler, background) = into_handler(py, handler, None)?;
        let mut listener = wait(py, self.get_ref()?.matching_listener().with(handler))?;
        if background {
            listener.set_background(true);
        }
        Ok(listener.into())
    }

    fn undeclare(&mut self, py: Python) -> PyResult<()> {
        wait(py, self.take()?.undeclare())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.get_ref()?))
    }
}

wrapper!(zenoh::query::Selector<'static>: Clone);
downcast_or_new!(Selector, None);

#[pymethods]
impl Selector {
    #[new]
    #[pyo3(signature = (arg, /, parameters = None))]
    pub(crate) fn new(
        arg: &Bound<PyAny>,
        #[pyo3(from_py_with = Parameters::from_py_opt)] parameters: Option<Parameters>,
    ) -> PyResult<Self> {
        Ok(Self(if let Some(params) = parameters {
            (KeyExpr::from_py(arg)?.0, params.0).into()
        } else if let Ok(s) = arg.extract::<String>() {
            s.parse().into_pyres()?
        } else if let Ok(k) = arg.extract::<KeyExpr>() {
            k.0.into()
        } else {
            return Err(arg.extract::<String>().unwrap_err());
        }))
    }

    #[getter]
    fn get_key_expr(&self) -> KeyExpr {
        self.0.key_expr().clone().into_owned().into()
    }

    #[getter]
    fn get_parameters(&self) -> Parameters {
        self.0.parameters().clone().into_owned().into()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
}

wrapper!(zenoh::query::Parameters<'static>: Clone);
downcast_or_new!(Parameters);

#[pymethods]
impl Parameters {
    #[new]
    pub(crate) fn new(obj: Option<&Bound<PyAny>>) -> PyResult<Self> {
        let Some(obj) = obj else {
            return Ok(Self(zenoh::query::Parameters::empty()));
        };
        if let Ok(map) = obj.extract::<HashMap<String, String>>() {
            return Ok(Self(map.into()));
        }
        Ok(Self(obj.extract::<String>()?.into()))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[pyo3(signature = (key, default = None))]
    fn get(&self, key: &str, default: Option<String>) -> Option<String> {
        self.0.get(key).map_into().or(default)
    }

    fn values(&self, key: &str) -> Vec<&str> {
        self.0.values(key).collect()
    }

    fn insert(&mut self, key: &str, value: &str) -> Option<String> {
        self.0.insert(key, value)
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        self.0.remove(key)
    }

    fn extend(&mut self, #[pyo3(from_py_with = Self::from_py)] parameters: Self) {
        self.0.extend(&parameters.0)
    }

    fn is_ordered(&self) -> bool {
        self.0.is_ordered()
    }

    fn __bool__(&self) -> bool {
        !self.0.is_empty()
    }

    fn __contains__(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    fn __getitem__(&self, key: &str) -> Option<String> {
        self.get(key, None)
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        let list = PyList::empty(py);
        for kv in self.0.iter() {
            list.append(kv)?;
        }
        list.as_any().try_iter()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> &str {
        self.0.as_str()
    }
}
