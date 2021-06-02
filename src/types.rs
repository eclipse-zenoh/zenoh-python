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
use async_std::channel::Sender;
use async_std::task;
use log::warn;
use pyo3::class::basic::CompareOp;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyTuple};
use pyo3::PyObjectProtocol;
use std::collections::HashMap;
use std::convert::TryFrom;
use zenoh::net::ZInt;

pub fn props_to_pydict(py: Python<'_>, props: zenoh::Properties) -> PyObject {
    props.iter().into_py_dict(py).to_object(py)
}

pub fn pydict_to_props(config: &PyDict) -> zenoh::Properties {
    let mut rust_config = zenoh::Properties::default();
    for (k, v) in config.iter() {
        rust_config.insert(k.to_string(), v.to_string());
    }
    rust_config
}

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

/// A zenoh Selector is the conjunction of a path expression identifying a set
/// of paths and some optional parts allowing to refine the set of paths and associated values.
///
/// Structure of a selector::
///
///    /s1/s2/.../sn?x>1&y<2&...&z=4(p1=v1;p2=v2;...;pn=vn)[a;b;x;y;...;z]
///    |           | |             | |                   |  |           |
///    |-- expr ---| |--- filter --| |---- properties ---|  |--fragment-|
///
/// where:
///
/// * **expr**: is a [`PathExpr`].
///
/// * **filter**: a list of predicates separated by ``&`` allowing to perform filtering on the values
///   associated with the matching keys. Each predicate has the form "`field`-`operator`-`value`" value where:
///
///    * *field* is the name of a field in the value (is applicable and is existing. otherwise the predicate is false)
///    * *operator* is one of a comparison operators: ``<`` , ``>`` , ``<=`` , ``>=`` , ``=`` , ``!=``
///    * *value* is the the value to compare the field’s value with
///
/// * **fragment**: a list of fields names allowing to return a sub-part of each value.
///   This feature only applies to structured values using a “self-describing” encoding, such as JSON or XML.
///   It allows to select only some fields within the structure. A new structure with only the selected fields
///   will be used in place of the original value.
///
/// **NOTE**: *the filters and fragments are not yet supported in current zenoh version.*
#[pyclass]
#[derive(Clone)]
pub(crate) struct Selector {
    pub(crate) s: zenoh::Selector,
}

#[pymethods]
impl Selector {
    /// the path expression part of this Selector (before ``?`` character).
    #[getter]
    fn path_expr(&self) -> &str {
        self.s.path_expr.as_str()
    }

    /// the predicate part of this Selector, as used in zenoh-net.
    /// I.e. all characters after ``?`` (or an empty String if no such character).
    #[getter]
    fn predicate(&self) -> &str {
        &self.s.predicate
    }

    /// the filter part of this Selector, if any (all characters after ``?`` and before ``(`` or ``[``)
    #[getter]
    fn filter(&self) -> Option<&str> {
        self.s.filter.as_deref()
    }

    /// the properties part of this Selector (all characters between ``( )`` and after ``?``)
    #[getter]
    fn properties(&self) -> HashMap<String, String> {
        self.s.properties.0.clone()
    }

    /// the fragment part of this Selector, if any (all characters between ``[ ]`` and after `?`)
    #[getter]
    fn fragment(&self) -> Option<&str> {
        self.s.fragment.as_deref()
    }
}

#[pyproto]
impl PyObjectProtocol for Selector {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.s.to_string())
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

/// A user value that is associated with a path in zenoh.
#[pyclass]
#[derive(Clone)]
pub(crate) struct Value {
    pub(crate) v: zenoh::Value,
}

#[allow(non_snake_case)]
#[pymethods]
impl Value {
    /// the encoding flag of the Value.
    ///
    /// :type: int
    #[getter]
    fn encoding(&self) -> ZInt {
        self.v.encoding()
    }

    /// Returns the encoding description of the Value.
    ///
    /// :rtype: str
    fn encoding_descr(&self) -> String {
        self.v.encoding_descr()
    }

    /// Returns the typed content of the value.
    ///
    /// :rtype: depend on the encoding flag (e.g. str for a StringUtf8 Value, int for an Integer Value ...)
    fn get_content(&self, py: Python) -> PyObject {
        use zenoh::Value::*;
        match &self.v {
            Raw(_, buf) => buf.to_vec().into_py(py),
            Custom {
                encoding_descr: _,
                data,
            } => data.to_vec().into_py(py),
            StringUtf8(s) => s.into_py(py),
            Properties(zenoh::Properties(p)) => p.clone().into_py(py),
            Json(s) => s.into_py(py),
            Integer(i) => i.into_py(py),
            Float(f) => f.into_py(py),
        }
    }

    /// Creates a Value from a bytes buffer and an encoding flag.
    /// See :class:`zenoh.net.encoding` for available flags.
    ///
    /// :param encoding: the encoding flag
    /// :param buffer: the bytes buffer
    /// :type encoding: int
    /// :type buffer: bytes
    #[staticmethod]
    #[text_signature = "(encoding, buffer)"]
    fn Raw(encoding: ZInt, buffer: &[u8]) -> Value {
        Value {
            v: zenoh::Value::Raw(encoding, buffer.into()),
        }
    }

    /// Creates a Value as a bytes buffer and an encoding description (free string).
    ///
    /// Note: this is equivalent to ``Value.Raw(zenoh.net.encoding.APP_CUSTOM, buffer)`` where buffer contains the encoding description and the data.
    ///
    /// :param encoding_descr: the encoding description
    /// :param buffer: the bytes buffer
    /// :type encoding_descr: str
    /// :type buffer: bytes
    #[staticmethod]
    #[text_signature = "(encoding_descr, buffer)"]
    fn Custom(encoding_descr: String, buffer: &[u8]) -> Value {
        Value {
            v: zenoh::Value::Custom {
                encoding_descr,
                data: buffer.into(),
            },
        }
    }

    /// A String value.
    ///
    /// Note: this is equivalent to ``Value.Raw(zenoh.net.encoding.STRING, buffer)`` where buffer contains the String
    ///
    /// :param s: the string
    /// :type s: str
    #[staticmethod]
    #[text_signature = "(s)"]
    fn StringUTF8(s: String) -> Value {
        Value {
            v: zenoh::Value::StringUtf8(s),
        }
    }

    /// A Properties value.
    ///
    /// Note: this is equivalent to  ``Value.Raw(zenoh.net.encoding.APP_PROPERTIES, buffer)`` where buffer contains the Properties encoded as a String
    ///
    /// :param p: the properties
    /// :type p: dict of str:str
    #[staticmethod]
    #[text_signature = "(p)"]
    fn Properties(p: HashMap<String, String>) -> Value {
        Value {
            v: zenoh::Value::Properties(zenoh::Properties::from(p)),
        }
    }

    /// A Json value (string format).
    ///
    /// Note: this is equivalent to ``Value.Raw(zenoh.net.encoding.APP_JSON, buffer)`` where buffer contains the Json string
    ///
    /// :param s: the Json string
    /// :type s: str
    #[staticmethod]
    #[text_signature = "(s)"]
    fn Json(s: String) -> Value {
        Value {
            v: zenoh::Value::Json(s),
        }
    }

    /// An Integer value.
    ///
    /// Note: this is equivalent to ``Value.Raw(zenoh.net.encoding.APP_INTEGER, buffer)`` where buffer contains the integer encoded as a String
    ///
    /// :param i: the integer
    /// :type i: int
    #[staticmethod]
    #[text_signature = "(i)"]
    fn Integer(i: i64) -> Value {
        Value {
            v: zenoh::Value::Integer(i),
        }
    }

    /// An Float value.
    ///
    /// Note: this is equivalent to ``Value.Raw(zenoh.net.encoding.APP_FLOAT, buffer)`` where buffer contains the float encoded as a String
    ///
    /// :param f: the float
    /// :type f: float
    #[staticmethod]
    #[text_signature = "(f)"]
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

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

pub(crate) fn zvalue_of_pyany(obj: &PyAny) -> PyResult<zenoh::Value> {
    match obj.get_type().name()? {
        "Value" => {
            let v: Value = obj.extract()?;
            Ok(v.v)
        }
        "bytes" => {
            let buf: &[u8] = obj.extract()?;
            Ok(zenoh::Value::Raw(
                zenoh::net::encoding::APP_OCTET_STREAM,
                buf.into(),
            ))
        }
        "str" => {
            let s: String = obj.extract()?;
            Ok(zenoh::Value::StringUtf8(s))
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
                && tuple.get_item(0).get_type().name()? == "str"
                && tuple.get_item(1).get_type().name()? == "bytes"
            {
                let encoding_descr: String = tuple.get_item(0).extract()?;
                let buf: &[u8] = tuple.get_item(1).extract()?;
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

/// A Data returned as a result of a :meth:`zenoh.Workspace.get` operation.
///
/// It contains the :attr:`path`, its associated :attr:`value` and a :attr:`timestamp` which corresponds to the time
/// at which the path/value has been put into zenoh.
#[pyclass]
pub(crate) struct Data {
    pub(crate) d: zenoh::Data,
}

#[pymethods]
impl Data {
    /// :type: str
    #[getter]
    fn path(&self) -> String {
        self.d.path.to_string()
    }

    /// :type: Value
    #[getter]
    fn value(&self) -> Value {
        Value {
            v: self.d.value.clone(),
        }
    }

    /// :type: Timestamp
    #[getter]
    fn timestamp(&self) -> Timestamp {
        Timestamp {
            t: self.d.timestamp.clone(),
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Data {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.d))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

// zenoh.ChangeKind (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/834 to be fixed)
//
/// The kind of a :class:`Change`.
#[pyclass]
#[derive(Clone)]
pub(crate) struct ChangeKind {
    pub(crate) k: ZInt,
}

#[allow(non_snake_case)]
#[pymethods]
impl ChangeKind {
    /// if the :class:`Change` was caused by a ``put`` operation.
    #[classattr]
    fn PUT() -> ChangeKind {
        ChangeKind {
            k: zenoh::net::data_kind::PUT,
        }
    }

    /// if the :class:`Change` was caused by a ``patch`` operation.
    #[classattr]
    fn PATCH() -> ChangeKind {
        ChangeKind {
            k: zenoh::net::data_kind::PATCH,
        }
    }

    /// if the :class:`Change` was caused by a ``delete`` operation.
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

    fn __repr__(&self) -> PyResult<&str> {
        self.__str__()
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

/// The notification of a change occured on a path/value and reported to a subscription.
///
/// See :meth:`zenoh.Workspace.subscribe`.
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
    /// the path related to this change.
    ///
    /// :type: str
    #[getter]
    fn path(&self) -> String {
        self.c.path.to_string()
    }

    /// the new Value if the kind is :attr:`ChangeKind.DELETE`. ``None`` if the kind is :attr:`ChangeKind.DELETE`.
    ///
    /// :type: :class:`Value` or ``None``
    #[getter]
    fn value(&self) -> Option<Value> {
        self.c.value.as_ref().map(|v| Value { v: v.clone() })
    }

    /// the Timestamp of the change
    ///
    /// :type: Timestamp
    #[getter]
    fn timestamp(&self) -> Timestamp {
        Timestamp {
            t: self.c.timestamp.clone(),
        }
    }

    /// the kind of change (:attr:`ChangeKind.PUT` or :attr:`ChangeKind.DELETE`).
    ///
    /// :type: ChangeKind
    #[getter]
    fn kind(&self) -> ChangeKind {
        ChangeKind {
            k: self.c.kind.clone() as ZInt,
        }
    }
}

/// A handle returned as a result of :meth:`Workspace.subscribe` method.
#[pyclass]
pub(crate) struct Subscriber {
    pub(crate) close_tx: Sender<bool>,
    pub(crate) loop_handle: Option<async_std::task::JoinHandle<()>>,
}

#[pymethods]
impl Subscriber {
    /// Closes the subscription.
    fn close(&mut self) {
        if let Some(handle) = self.loop_handle.take() {
            task::block_on(async {
                if let Err(e) = self.close_tx.send(true).await {
                    warn!("Error in Subscriber::close() : {}", e);
                }
                handle.await;
            });
        }
    }
}

/// A *GET* request received by an evaluation function (see :meth:`Workspace::register_eval`).
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
    /// The selector used by this GetRequest
    ///
    /// :type: Selector
    #[getter]
    fn selector(&self) -> Selector {
        Selector {
            s: self.r.selector.clone(),
        }
    }

    /// Send a path/value as a reply to the requester.
    ///
    /// Note that the *value* parameter also accepts the following types that can be converted to a :class:`Value`:
    ///
    /// * **bytes** for a ``Value.Raw(APP_OCTET_STREAM, bytes)``
    /// * **str** for a ``Value.StringUtf8(str)``
    /// * **int** for a ``Value.Integer(int)``
    /// * **float** for a ``Value.Float(int)``
    /// * **dict of str:str** for a ``Value.Properties(dict)``
    /// * **(str, bytes)** for a ``Value.Custom(str, bytes)``
    ///
    /// :param path: the path
    /// :type path: str
    /// :param value: the value as a :class:`Value`
    /// :type value: Value
    #[text_signature = "(self, path, value)"]
    fn reply(&self, path: String, value: &PyAny) -> PyResult<()> {
        let p = path_of_string(path)?;
        let v = zvalue_of_pyany(value)?;
        self.r.reply(p, v);
        Ok(())
    }
}

/// A handle returned as a result of :meth:`Workspace.register_eval` method.
#[pyclass]
pub(crate) struct Eval {
    pub(crate) close_tx: Sender<bool>,
    pub(crate) loop_handle: Option<async_std::task::JoinHandle<()>>,
}

#[pymethods]
impl Eval {
    /// Closes the eval.
    fn close(&mut self) {
        if let Some(handle) = self.loop_handle.take() {
            task::block_on(async {
                if let Err(e) = self.close_tx.send(true).await {
                    warn!("Error in Eval::close() : {}", e);
                }
                handle.await;
            });
        }
    }
}
