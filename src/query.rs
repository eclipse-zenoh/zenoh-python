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
    types::{PyDict, PyIterator, PyTuple, PyType},
};

use crate::{
    bytes::{Encoding, ZBytes},
    config::ZenohId,
    handlers::HandlerImpl,
    key_expr::KeyExpr,
    macros::{build, downcast_or_new, enum_mapper, option_wrapper, wrapper},
    qos::{CongestionControl, Priority},
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
    fn new(mode: Option<ConsolidationMode>) -> PyResult<Self> {
        let Some(mode) = mode else {
            return Ok(Self::DEFAULT);
        };
        Ok(Self(mode.into_rust().into()))
    }

    fn mode(&self) -> ConsolidationMode {
        self.0.mode().into()
    }
}

wrapper!(zenoh::query::Query: Clone);

#[pymethods]
impl Query {
    #[getter]
    fn selector(&self) -> Selector {
        self.0.selector().into_owned().into()
    }

    #[getter]
    fn key_expr(&self) -> KeyExpr {
        self.0.key_expr().clone().into_owned().into()
    }

    #[getter]
    fn parameters(&self) -> Parameters {
        self.0.parameters().clone().into_owned().into()
    }

    #[getter]
    fn payload(&self) -> Option<ZBytes> {
        self.0.payload().cloned().map_into()
    }

    #[getter]
    fn encoding(&self) -> Option<Encoding> {
        self.0.encoding().cloned().map_into()
    }

    #[getter]
    fn attachment(&self) -> Option<ZBytes> {
        self.0.attachment().cloned().map_into()
    }

    // TODO timestamp
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (key_expr, payload, *, encoding = None, congestion_control = None, priority = None, express = None, attachment = None))]
    fn reply(
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
            self.0.reply(key_expr, payload),
            encoding,
            congestion_control,
            priority,
            express,
            attachment,
        );
        wait(py, build)
    }
    #[pyo3(signature = (payload, *, encoding = None))]
    fn reply_err(
        &self,
        py: Python,
        #[pyo3(from_py_with = "ZBytes::from_py")] payload: ZBytes,
        #[pyo3(from_py_with = "Encoding::from_py_opt")] encoding: Option<Encoding>,
    ) -> PyResult<()> {
        let build = build!(self.0.reply_err(payload), encoding);
        wait(py, build)
    }

    #[pyo3(signature = (key_expr, *, congestion_control = None, priority = None, express = None, attachment = None))]
    fn reply_del(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
        #[pyo3(from_py_with = "ZBytes::from_py_opt")] attachment: Option<ZBytes>,
    ) -> PyResult<()> {
        let build = build!(
            self.0.reply_del(key_expr),
            congestion_control,
            priority,
            express,
            attachment,
        );
        wait(py, build)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
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
    fn replier_id(&self) -> Option<ZenohId> {
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
    fn handler(&self, py: Python) -> PyResult<PyObject> {
        Ok(self.get_ref()?.handler().to_object(py))
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
        self.handler(py)?.bind(py).iter()
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
        #[pyo3(from_py_with = "Parameters::from_py_opt")] parameters: Option<Parameters>,
    ) -> PyResult<Self> {
        Ok(Self(if let Some(params) = parameters {
            (KeyExpr::from_py(arg)?.0, params.0).into()
        } else if let Ok(s) = String::extract_bound(arg) {
            s.parse().into_pyres()?
        } else if let Ok(k) = KeyExpr::extract_bound(arg) {
            k.0.into()
        } else {
            return Err(String::extract_bound(arg).unwrap_err());
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
        if let Ok(map) = <HashMap<String, String>>::extract_bound(obj) {
            return Ok(Self(map.into()));
        }
        Ok(Self(String::extract_bound(obj)?.into()))
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

    fn extend(&mut self, #[pyo3(from_py_with = "Parameters::from_py")] parameters: Parameters) {
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

    fn __iter__(&self) -> Vec<(&str, &str)> {
        self.0.iter().collect()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> &str {
        self.0.as_str()
    }
}
