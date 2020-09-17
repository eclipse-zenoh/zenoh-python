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
use crate::zenoh_net::Timestamp;
use async_std::sync::Sender;
use async_std::task;
use pyo3::class::basic::CompareOp;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::PyObjectProtocol;
use std::collections::HashMap;
use std::convert::TryFrom;
use zenoh::net::ZInt;

pub(crate) fn path_of_string(s: String) -> PyResult<zenoh::Path> {
    zenoh::Path::try_from(s).map_err(|e| PyErr::new::<exceptions::PyValueError, _>(e.to_string()))
}

pub(crate) fn pathexpr_of_string(s: String) -> PyResult<zenoh::PathExpr> {
    zenoh::PathExpr::try_from(s)
        .map_err(|e| PyErr::new::<exceptions::PyValueError, _>(e.to_string()))
}

pub(crate) fn selector_of_string(s: String) -> PyResult<zenoh::Selector> {
    zenoh::Selector::try_from(s)
        .map_err(|e| PyErr::new::<exceptions::PyValueError, _>(e.to_string()))
}

// zenoh.Selector
#[pyclass]
#[derive(Clone)]
pub(crate) struct Selector {
    pub(crate) s: zenoh::Selector,
}

#[pymethods]
impl Selector {
    #[getter]
    fn path_expr(&self) -> &str {
        self.s.path_expr.as_str()
    }

    #[getter]
    fn predicate(&self) -> &str {
        &self.s.predicate
    }

    #[getter]
    fn filter(&self) -> Option<&str> {
        self.s.filter.as_ref().map(|s| s.as_str())
    }

    #[getter]
    fn properties(&self) -> HashMap<String, String> {
        self.s.properties.0.clone()
    }

    #[getter]
    fn fragment(&self) -> Option<&str> {
        self.s.fragment.as_ref().map(|s| s.as_str())
    }
}

#[pyproto]
impl PyObjectProtocol for Selector {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.s.to_string())
    }
}

// zenoh.Value
#[pyclass]
#[derive(Clone)]
pub(crate) struct Value {
    pub(crate) v: zenoh::Value,
}

#[allow(non_snake_case)]
#[pymethods]
impl Value {
    #[getter]
    fn encoding(&self) -> ZInt {
        self.v.encoding()
    }

    fn encoding_descr(&self) -> String {
        self.v.encoding_descr()
    }

    fn content(&self, py: Python) -> PyObject {
        use zenoh::Value::*;
        match &self.v {
            Raw(_, buf) => buf.to_vec().into_py(py),
            Custom {
                encoding_descr: _,
                data,
            } => data.to_vec().into_py(py),
            StringUTF8(s) => s.into_py(py),
            Properties(zenoh::Properties(p)) => p.clone().into_py(py),
            Json(s) => s.into_py(py),
            Integer(i) => i.into_py(py),
            Float(f) => f.into_py(py),
        }
    }

    #[staticmethod]
    fn Raw(encoding: ZInt, buffer: Vec<u8>) -> Value {
        Value {
            v: zenoh::Value::Raw(encoding, buffer.into()),
        }
    }

    #[staticmethod]
    fn Custom(encoding_descr: String, buffer: Vec<u8>) -> Value {
        Value {
            v: zenoh::Value::Custom {
                encoding_descr,
                data: buffer.into(),
            },
        }
    }

    #[staticmethod]
    fn StringUTF8(s: String) -> Value {
        Value {
            v: zenoh::Value::StringUTF8(s),
        }
    }

    #[staticmethod]
    fn Properties(p: HashMap<String, String>) -> Value {
        Value {
            v: zenoh::Value::Properties(zenoh::Properties::from(p)),
        }
    }

    #[staticmethod]
    fn Json(s: String) -> Value {
        Value {
            v: zenoh::Value::Json(s),
        }
    }

    #[staticmethod]
    fn Integer(i: i64) -> Value {
        Value {
            v: zenoh::Value::Integer(i),
        }
    }

    #[staticmethod]
    fn Float(f: f64) -> Value {
        Value {
            v: zenoh::Value::Float(f),
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Value {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.v))
    }
}

pub(crate) fn zvalue_of_pyany(obj: &PyAny) -> PyResult<zenoh::Value> {
    match obj.get_type().name().as_ref() {
        "Value" => {
            let v: Value = obj.extract()?;
            Ok(v.v)
        }
        "bytes" => {
            let buf: Vec<u8> = obj.extract()?;
            Ok(zenoh::Value::Raw(
                zenoh::net::encoding::APP_OCTET_STREAM,
                buf.into(),
            ))
        }
        "str" => {
            let s: String = obj.extract()?;
            Ok(zenoh::Value::StringUTF8(s))
        }
        "dict" => {
            let props: HashMap<String, String> = obj.extract()?;
            Ok(zenoh::Value::Properties(zenoh::Properties::from(props)))
        }
        "int" => {
            let i: i64 = obj.extract()?;
            Ok(zenoh::Value::Integer(i))
        }
        "float" => {
            let f: f64 = obj.extract()?;
            Ok(zenoh::Value::Float(f))
        }
        "tuple" => {
            let tuple: &PyTuple = obj.downcast()?;
            if tuple.len() == 2
                && tuple.get_item(0).get_type().name() == "str"
                && tuple.get_item(1).get_type().name() == "bytes"
            {
                let encoding_descr: String = tuple.get_item(0).extract()?;
                let buf: Vec<u8> = tuple.get_item(1).extract()?;
                if let Ok(encoding) = zenoh::net::encoding::from_str(&encoding_descr) {
                    Ok(zenoh::Value::Raw(encoding, buf.into()))
                } else {
                    Ok(zenoh::Value::Custom {
                        encoding_descr,
                        data: buf.into(),
                    })
                }
            } else {
                Err(PyErr::new::<exceptions::PyValueError, _>(format!(
                    "Cannot convert type '{:?}' to a zenoh Value",
                    tuple
                )))
            }
        }
        x => Err(PyErr::new::<exceptions::PyValueError, _>(format!(
            "Cannot convert type '{}' to a zenoh Value",
            x
        ))),
    }
}

// zenoh.Data
#[pyclass]
pub(crate) struct Data {
    pub(crate) d: zenoh::Data,
}

#[pymethods]
impl Data {
    #[getter]
    fn path(&self) -> String {
        self.d.path.to_string()
    }

    #[getter]
    fn value(&self) -> Value {
        Value {
            v: self.d.value.clone(),
        }
    }

    #[getter]
    fn timestamp(&self) -> Timestamp {
        Timestamp {
            t: self.d.timestamp.clone(),
        }
    }
}

// zenoh.ChangeKind (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/834 to be fixed)
#[pyclass]
#[derive(Clone)]
pub(crate) struct ChangeKind {
    pub(crate) k: ZInt,
}

#[allow(non_snake_case)]
#[pymethods]
impl ChangeKind {
    #[classattr]
    fn PUT() -> ChangeKind {
        ChangeKind {
            k: zenoh::net::data_kind::PUT,
        }
    }

    #[classattr]
    fn PATCH() -> ChangeKind {
        ChangeKind {
            k: zenoh::net::data_kind::PATCH,
        }
    }

    #[classattr]
    fn DELETE() -> ChangeKind {
        ChangeKind {
            k: zenoh::net::data_kind::DELETE,
        }
    }
}

#[pyproto]
impl PyObjectProtocol for ChangeKind {
    fn __str__(&self) -> PyResult<&str> {
        match self.k {
            zenoh::net::data_kind::PUT => Ok("PUT"),
            zenoh::net::data_kind::PATCH => Ok("PATCH"),
            zenoh::net::data_kind::DELETE => Ok("DELETE"),
            _ => Ok("PUT"),
        }
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<&str> {
        self.__str__()
    }

    fn __richcmp__(&self, other: Self, op: CompareOp) -> PyResult<bool> {
        match op {
            CompareOp::Eq => Ok(self.k == other.k),
            CompareOp::Ne => Ok(self.k != other.k),
            CompareOp::Lt => Ok(self.k < other.k),
            CompareOp::Le => Ok(self.k <= other.k),
            CompareOp::Gt => Ok(self.k > other.k),
            CompareOp::Ge => Ok(self.k >= other.k),
        }
    }
}

// zenoh.Change
#[pyclass]
#[derive(Clone)]
pub(crate) struct Change {
    pub(crate) c: zenoh::Change,
}

impl pyo3::conversion::ToPyObject for Change {
    fn to_object(&self, py: Python) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(pyo3::Py::new(py, self.clone()).unwrap(), py)
    }
}

#[pymethods]
impl Change {
    #[getter]
    fn path(&self) -> String {
        self.c.path.to_string()
    }

    #[getter]
    fn value(&self) -> Option<Value> {
        self.c.value.as_ref().map(|v| Value { v: v.clone() })
    }

    #[getter]
    fn timestamp(&self) -> Timestamp {
        Timestamp {
            t: self.c.timestamp.clone(),
        }
    }

    #[getter]
    fn kind(&self) -> ChangeKind {
        ChangeKind {
            k: self.c.kind.clone() as ZInt,
        }
    }
}

// zenoh.Subscriber
#[pyclass]
pub(crate) struct Subscriber {
    pub(crate) close_tx: Sender<bool>,
    pub(crate) loop_handle: Option<async_std::task::JoinHandle<()>>,
}

#[pymethods]
impl Subscriber {
    fn close(&mut self) {
        if let Some(handle) = self.loop_handle.take() {
            task::block_on(async {
                self.close_tx.send(true).await;
                handle.await;
            });
        }
    }
}

// zenoh.GetRequest
#[pyclass]
#[derive(Clone)]
pub(crate) struct GetRequest {
    pub(crate) r: zenoh::GetRequest,
}

impl pyo3::conversion::ToPyObject for GetRequest {
    fn to_object(&self, py: Python) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(pyo3::Py::new(py, self.clone()).unwrap(), py)
    }
}

#[pymethods]
impl GetRequest {
    #[getter]
    fn selector(&self) -> Selector {
        Selector {
            s: self.r.selector.clone(),
        }
    }

    fn reply(&self, path: String, value: &PyAny) -> PyResult<()> {
        let p = path_of_string(path)?;
        let v = zvalue_of_pyany(value)?;
        task::block_on(async { self.r.reply(p, v).await });
        Ok(())
    }
}

// zenoh.Eval
#[pyclass]
pub(crate) struct Eval {
    pub(crate) close_tx: Sender<bool>,
    pub(crate) loop_handle: Option<async_std::task::JoinHandle<()>>,
}

#[pymethods]
impl Eval {
    fn close(&mut self) {
        if let Some(handle) = self.loop_handle.take() {
            task::block_on(async {
                self.close_tx.send(true).await;
                handle.await;
            });
        }
    }
}
