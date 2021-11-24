use std::collections::HashMap;
use std::ops::BitOr;

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
use crate::sample_kind::SampleKind;
use async_std::channel::Sender;
use async_std::task;
use log::warn;
use pyo3::exceptions;
use pyo3::number::PyNumberOrProtocol;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyTuple};
use pyo3::PyObjectProtocol;
use zenoh::config::whatami::WhatAmIMatcher;
use zenoh::config::WhatAmI as ZWhatAmI;
use zenoh::prelude::{Encoding, KeyExpr as ZKeyExpr, ZInt};

// zenoh.config (simulate the package as a class, and consts as class attributes)
/// Constants and helpers to build the configuration to pass to :func:`zenoh.open`.
#[allow(non_camel_case_types)]
#[pyclass]
pub(crate) struct config {}

#[allow(non_snake_case)]
#[pymethods]
impl config {
    #[classattr]
    pub fn ZN_MODE_KEY() -> ZInt {
        zenoh::config::ZN_MODE_KEY
    }

    #[classattr]
    pub fn ZN_PEER_KEY() -> ZInt {
        zenoh::config::ZN_PEER_KEY
    }

    #[classattr]
    pub fn ZN_LISTENER_KEY() -> ZInt {
        zenoh::config::ZN_LISTENER_KEY
    }

    #[classattr]
    pub fn ZN_USER_KEY() -> ZInt {
        zenoh::config::ZN_USER_KEY
    }

    #[classattr]
    fn ZN_PASSWORD_KEY() -> ZInt {
        zenoh::config::ZN_PASSWORD_KEY
    }

    #[classattr]
    pub fn ZN_MULTICAST_SCOUTING_KEY() -> ZInt {
        zenoh::config::ZN_MULTICAST_SCOUTING_KEY
    }

    #[classattr]
    pub fn ZN_MULTICAST_INTERFACE_KEY() -> ZInt {
        zenoh::config::ZN_MULTICAST_INTERFACE_KEY
    }

    #[classattr]
    pub fn ZN_MULTICAST_IPV4_ADDRESS_KEY() -> ZInt {
        zenoh::config::ZN_MULTICAST_IPV4_ADDRESS_KEY
    }

    #[classattr]
    pub fn ZN_SCOUTING_TIMEOUT_KEY() -> ZInt {
        zenoh::config::ZN_SCOUTING_TIMEOUT_KEY
    }

    #[classattr]
    pub fn ZN_SCOUTING_DELAY_KEY() -> ZInt {
        zenoh::config::ZN_SCOUTING_DELAY_KEY
    }

    #[classattr]
    pub fn ZN_ADD_TIMESTAMP_KEY() -> ZInt {
        zenoh::config::ZN_ADD_TIMESTAMP_KEY
    }

    #[classattr]
    pub fn ZN_LOCAL_ROUTING_KEY() -> ZInt {
        zenoh::config::ZN_LOCAL_ROUTING_KEY
    }

    // #[staticmethod]
    // pub fn empty<'p>(py: Python<'p>) -> Vec<(ZInt, &'p PyBytes)> {
    //     props_to_pylist(py, zenoh::config::empty())
    // }

    // #[staticmethod]
    // pub fn default<'p>(py: Python<'p>) -> Vec<(ZInt, &'p PyBytes)> {
    //     props_to_pylist(py, zenoh::config::default())
    // }

    // #[staticmethod]
    // pub fn peer<'p>(py: Python<'p>) -> Vec<(ZInt, &'p PyBytes)> {
    //     props_to_pylist(py, zenoh::config::peer())
    // }

    // #[staticmethod]
    // pub fn client<'p>(py: Python<'p>, peer: Option<String>) -> Vec<(ZInt, &'p PyBytes)> {
    //     props_to_pylist(py, zenoh::config::client(peer))
    // }
}

// zenoh.info (simulate the package as a class, and consts as class attributes)
/// Constants and helpers to interpret the properties returned by :func:`zenoh.Session.info`.
#[allow(non_camel_case_types)]
#[pyclass]
pub(crate) struct info {}

#[allow(non_snake_case)]
#[pymethods]
impl info {
    #[classattr]
    fn ZN_INFO_PID_KEY() -> ZInt {
        zenoh::info::ZN_INFO_PID_KEY
    }

    #[classattr]
    fn ZN_INFO_PEER_PID_KEY() -> ZInt {
        zenoh::info::ZN_INFO_PEER_PID_KEY
    }

    #[classattr]
    fn ZN_INFO_ROUTER_PID_KEY() -> ZInt {
        zenoh::info::ZN_INFO_ROUTER_PID_KEY
    }
}

// zenoh.whatami (simulate the package as a class, and consts as class attributes)
/// Constants defining the different zenoh process to look for with :func:`zenoh.scout`.
#[allow(non_camel_case_types)]
#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WhatAmI {
    inner: WhatAmIMatcher,
}
impl From<WhatAmI> for WhatAmIMatcher {
    fn from(w: WhatAmI) -> Self {
        w.inner
    }
}
impl BitOr for WhatAmI {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        WhatAmI {
            inner: self.inner | rhs.inner,
        }
    }
}
#[pyproto]
impl pyo3::PyNumberProtocol for WhatAmI
where
    <Self as PyNumberOrProtocol<'p>>::Left: BitOr<
        <Self as PyNumberOrProtocol<'p>>::Right,
        Output = <Self as PyNumberOrProtocol<'p>>::Result,
    >,
{
    fn __or__(lhs: Self, rhs: Self) -> Self
    where
        Self: PyNumberOrProtocol<'p>,
    {
        lhs | rhs
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl WhatAmI {
    #[classattr]
    fn Router() -> Self {
        WhatAmI {
            inner: ZWhatAmI::Router.into(),
        }
    }

    #[classattr]
    fn Peer() -> Self {
        WhatAmI {
            inner: ZWhatAmI::Peer.into(),
        }
    }

    #[classattr]
    fn Client() -> Self {
        WhatAmI {
            inner: ZWhatAmI::Client.into(),
        }
    }
}

impl std::fmt::Display for WhatAmI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.to_str())
    }
}

/// A Hello message received as a response to a :meth:`scout`
#[pyclass]
#[derive(Clone)]
pub(crate) struct Hello {
    pub(crate) h: zenoh::scouting::Hello,
}

#[pymethods]
impl Hello {
    /// The PeerId of the Hello message sender
    ///
    /// :type: :class:`PeerId` or ``None``
    #[getter]
    fn pid(&self) -> Option<PeerId> {
        self.h.pid.as_ref().map(|p| PeerId { p: *p })
    }

    /// The mode of the Hello message sender (bitmask of constants from :class:`whatami`)
    ///
    /// :type: :class:`whatami` or ``None``
    #[getter]
    fn whatami(&self) -> Option<WhatAmI> {
        self.h.whatami.map(|w| WhatAmI { inner: w.into() })
    }

    /// The locators list of the Hello message sender
    ///
    /// :type: list of str or ``None``
    #[getter]
    fn locators(&self) -> Option<Vec<String>> {
        self.h
            .locators
            .as_ref()
            .map(|v| v.iter().map(|l| l.to_string()).collect())
    }
}

#[pyproto]
impl PyObjectProtocol for Hello {
    fn __str__(&self) -> String {
        self.h.to_string()
    }
}

// zenoh.resource_name (simulate the package as a class with static methodss)
#[allow(non_camel_case_types)]
#[pyclass]
pub struct KeyExpr {
    pub(crate) inner: ZKeyExpr<'static>,
}

#[allow(non_snake_case)]
#[pymethods]
impl KeyExpr {
    /// Return true if both resource names intersect.
    ///
    /// :param s1: the 1st resource name
    /// :type s1: str
    /// :param s2: the 2nd resource name
    /// :type s2: str
    #[staticmethod]
    #[pyo3(text_signature = "(s1, s2)")]
    fn intersect(s1: &PyAny, s2: &PyAny) -> bool {
        let s1 = zkey_expr_of_pyany(s1).unwrap();
        let s2 = zkey_expr_of_pyany(s2).unwrap();
        match (s1.as_id_and_suffix(), s2.as_id_and_suffix()) {
            ((s1, _), (s2, _)) if s1 != s2 => false,
            ((_, s1), (_, s2)) => zenoh::utils::key_expr::intersect(s1, s2),
        }
    }
}

#[pyproto]
impl PyObjectProtocol for KeyExpr {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }
}

impl From<KeyExpr> for ZKeyExpr<'static> {
    fn from(r: KeyExpr) -> ZKeyExpr<'static> {
        r.inner
    }
}

impl From<ZKeyExpr<'static>> for KeyExpr {
    fn from(inner: ZKeyExpr<'static>) -> KeyExpr {
        KeyExpr { inner }
    }
}

pub(crate) fn zkey_expr_of_pyany(obj: &PyAny) -> PyResult<ZKeyExpr> {
    match obj.get_type().name()? {
        "KeyExpr" => {
            let rk: PyRef<KeyExpr> = obj.extract()?;
            Ok(rk.inner.clone())
        }
        "int" => {
            let id: u64 = obj.extract()?;
            Ok(id.into())
        }
        "str" => {
            let name: String = obj.extract()?;
            Ok(name.into())
        }
        "tuple" => {
            let tuple: &PyTuple = obj.downcast()?;
            if tuple.len() == 2
                && tuple.get_item(0)?.get_type().name()? == "int"
                && tuple.get_item(1)?.get_type().name()? == "str"
            {
                let id: u64 = tuple.get_item(0)?.extract()?;
                let suffix: String = tuple.get_item(1)?.extract()?;
                Ok(ZKeyExpr::from(id).with_suffix(&suffix).to_owned())
            } else {
                Err(PyErr::new::<exceptions::PyValueError, _>(format!(
                    "Cannot convert type '{:?}' to a zenoh-net KeyExpr",
                    tuple
                )))
            }
        }
        x => Err(PyErr::new::<exceptions::PyValueError, _>(format!(
            "Cannot convert type '{}' to a zenoh-net KeyExpr",
            x
        ))),
    }
}

/// A Peer id
#[pyclass]
pub(crate) struct PeerId {
    pub(crate) p: zenoh::prelude::PeerId,
}

#[pyproto]
impl PyObjectProtocol for PeerId {
    fn __str__(&self) -> String {
        self.p.to_string()
    }
}

/// A user value that is associated with a path in zenoh.
#[pyclass]
#[derive(Clone)]
pub struct Value {
    pub(crate) v: zenoh::prelude::Value,
}
impl From<Value> for zenoh::prelude::Value {
    fn from(v: Value) -> Self {
        v.v
    }
}
impl From<zenoh::prelude::Value> for Value {
    fn from(v: zenoh::prelude::Value) -> Self {
        Value { v }
    }
}

macro_rules! const_prefixes {
    ($id: ident $p: path) => {
        pub const $id: ZInt = $p.prefix;
    };
    ($($id: ident: $p: path),*) => {
        $(const_prefixes!($id $p);)*
    };
}

trait IntoPyAlt<U> {
    fn into_py_alt(self, py: Python) -> U;
}

impl IntoPyAlt<PyObject> for serde_json::Value {
    fn into_py_alt(self, py: Python) -> PyObject {
        match self {
            serde_json::Value::Null => py.None(),
            serde_json::Value::Bool(v) => v.into_py(py),
            serde_json::Value::Number(v) => v.into_py_alt(py),
            serde_json::Value::String(v) => v.into_py(py),
            serde_json::Value::Array(a) => a
                .into_iter()
                .map(|v| v.into_py_alt(py))
                .collect::<Vec<_>>()
                .into_py(py),
            serde_json::Value::Object(m) => m
                .into_iter()
                .map(|(k, v)| (k, v.into_py_alt(py)))
                .collect::<std::collections::HashMap<_, _>>()
                .into_py(py),
        }
    }
}
impl IntoPyAlt<PyObject> for serde_json::Number {
    fn into_py_alt(self, py: Python) -> PyObject {
        if let Some(v) = self.as_u64() {
            return v.into_py(py);
        }
        if let Some(v) = self.as_i64() {
            return v.into_py(py);
        }
        if let Some(v) = self.as_f64() {
            return v.into_py(py);
        }
        unreachable!()
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl Value {
    const_prefixes!(
        EMPTY: Encoding::EMPTY,
        APP_OCTET_STREAM: Encoding::APP_OCTET_STREAM,
        APP_CUSTOM: Encoding::APP_CUSTOM,
        TEXT_PLAIN: Encoding::TEXT_PLAIN,
        STRING: Encoding::STRING,
        APP_PROPERTIES: Encoding::APP_PROPERTIES,
        APP_JSON: Encoding::APP_JSON,
        APP_SQL: Encoding::APP_SQL,
        APP_INTEGER: Encoding::APP_INTEGER,
        APP_FLOAT: Encoding::APP_FLOAT,
        APP_XML: Encoding::APP_XML,
        APP_XHTML_XML: Encoding::APP_XHTML_XML,
        APP_X_WWW_FORM_URLENCODED: Encoding::APP_X_WWW_FORM_URLENCODED,
        TEXT_JSON: Encoding::TEXT_JSON,
        TEXT_HTML: Encoding::TEXT_HTML,
        TEXT_XML: Encoding::TEXT_XML,
        TEXT_CSS: Encoding::TEXT_CSS,
        TEXT_CSV: Encoding::TEXT_CSV,
        TEXT_JAVASCRIPT: Encoding::TEXT_JAVASCRIPT,
        IMG_JPG: Encoding::IMG_JPG,
        IMG_PNG: Encoding::IMG_PNG,
        IMG_GIF: Encoding::IMG_GIF
    );
    /// the encoding flag of the Value.
    ///
    /// :type: int
    #[getter]
    fn encoding(&self) -> ZInt {
        self.v.encoding.prefix
    }

    /// Returns the encoding description of the Value.
    ///
    /// :rtype: str
    fn encoding_descr(&self) -> String {
        self.v.encoding.to_string()
    }

    /// Returns the typed content of the value.
    ///
    /// :rtype: depend on the encoding flag (e.g. str for a StringUtf8 Value, int for an Integer Value ...)
    fn get_content(&self, py: Python) -> PyObject {
        let payload = self.v.payload.contiguous();
        if !self.v.encoding.suffix.is_empty() {
            return self.v.payload.to_vec().into_py(py);
        }
        let vec_payload = || payload.to_vec().into_py(py);
        match self.v.encoding.prefix {
            Self::STRING => payload.to_string().into_py(py),
            Self::APP_PROPERTIES => self
                .v
                .as_properties()
                .map(|v| v.0.into_py(py))
                .unwrap_or_else(vec_payload),
            Self::APP_JSON | Self::TEXT_JSON => self
                .v
                .as_json()
                .map(|v: serde_json::Value| v.into_py_alt(py))
                .unwrap_or_else(vec_payload),
            Self::APP_INTEGER => self
                .v
                .as_integer()
                .map(|v: i64| v.into_py(py))
                .unwrap_or_else(vec_payload),
            Self::APP_FLOAT => self
                .v
                .as_float()
                .map(|v: f64| v.into_py(py))
                .unwrap_or_else(vec_payload),
            _ => vec_payload(),
        }
    }

    /// Creates a Value from a bytes buffer and an encoding flag.
    /// See :class:`zenoh.encoding` for available flags.
    ///
    /// :param encoding: the encoding flag
    /// :param buffer: the bytes buffer
    /// :type encoding: int
    /// :type buffer: bytes
    #[staticmethod]
    #[pyo3(text_signature = "(encoding, buffer)")]
    fn Raw(encoding: ZInt, buffer: &[u8]) -> Value {
        Value {
            v: zenoh::prelude::Value {
                payload: Vec::from(buffer).into(),
                encoding: encoding.into(),
            },
        }
    }

    /// Creates a Value as a bytes buffer and an encoding description (free string).
    ///
    /// Note: this is equivalent to ``Value.Raw(zenoh.encoding.APP_CUSTOM, buffer)`` where buffer contains the encoding description and the data.
    ///
    /// :param encoding_descr: the encoding description
    /// :param buffer: the bytes buffer
    /// :type encoding_descr: str
    /// :type buffer: bytes
    #[staticmethod]
    #[pyo3(text_signature = "(encoding_descr, buffer)")]
    fn Custom(encoding_descr: String, buffer: &[u8]) -> Value {
        Value {
            v: zenoh::prelude::Value::new(Vec::from(buffer).into())
                .encoding(Encoding::APP_CUSTOM.with_suffix(encoding_descr)),
        }
    }

    /// A String value.
    ///
    /// Note: this is equivalent to ``Value.Raw(zenoh.encoding.STRING, buffer)`` where buffer contains the String
    ///
    /// :param s: the string
    /// :type s: str
    #[staticmethod]
    #[pyo3(text_signature = "(s)")]
    fn StringUTF8(s: String) -> Value {
        Value { v: s.into() }
    }

    /// A Properties value.
    ///
    /// Note: this is equivalent to  ``Value.Raw(zenoh.encoding.APP_PROPERTIES, buffer)`` where buffer contains the Properties encoded as a String
    ///
    /// :param p: the properties
    /// :type p: dict of str:str
    #[staticmethod]
    #[pyo3(text_signature = "(p)")]
    fn Properties(p: HashMap<String, String>) -> Value {
        Value {
            v: zenoh::prelude::Properties::from(p).into(),
        }
    }

    /// A Json value (string format).
    ///
    /// Note: this is equivalent to ``Value.Raw(zenoh.encoding.APP_JSON, buffer)`` where buffer contains the Json string
    ///
    /// :param s: the Json string
    /// :type s: str
    #[staticmethod]
    #[pyo3(text_signature = "(s)")]
    fn Json(s: String) -> Value {
        Value {
            v: zenoh::prelude::Value::from(s).encoding(Encoding::APP_JSON),
        }
    }

    /// An Integer value.
    ///
    /// Note: this is equivalent to ``Value.Raw(zenoh.encoding.APP_INTEGER, buffer)`` where buffer contains the integer encoded as a String
    ///
    /// :param i: the integer
    /// :type i: int
    #[staticmethod]
    #[pyo3(text_signature = "(i)")]
    fn Integer(i: i64) -> Value {
        Value { v: i.into() }
    }

    /// An Float value.
    ///
    /// Note: this is equivalent to ``Value.Raw(zenoh.encoding.APP_FLOAT, buffer)`` where buffer contains the float encoded as a String
    ///
    /// :param f: the float
    /// :type f: float
    #[staticmethod]
    #[pyo3(text_signature = "(f)")]
    fn Float(f: f64) -> Value {
        Value { v: f.into() }
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
}

pub(crate) fn zvalue_of_pyany(obj: &PyAny) -> PyResult<zenoh::prelude::Value> {
    use zenoh::prelude::Value as ZValue;
    match obj.get_type().name()? {
        "Value" => {
            let v: Value = obj.extract()?;
            Ok(v.v)
        }
        "bytes" => {
            let buf: &[u8] = obj.extract()?;
            Ok(ZValue::new(Vec::from(buf).into()).encoding(Encoding::APP_OCTET_STREAM))
        }
        "str" => {
            let s: String = obj.extract()?;
            Ok(s.into())
        }
        "dict" => {
            let props: HashMap<String, String> = obj.extract()?;
            Ok(zenoh::prelude::Properties::from(props).into())
        }
        "int" => {
            let i: i64 = obj.extract()?;
            Ok(i.into())
        }
        "float" => {
            let f: f64 = obj.extract()?;
            Ok(f.into())
        }
        "tuple" => {
            let tuple: &PyTuple = obj.downcast()?;
            if tuple.len() == 2
                && tuple.get_item(0)?.get_type().name()? == "bytes"
                && tuple.get_item(1)?.get_type().name()? == "str"
            {
                let buf: &[u8] = tuple.get_item(0)?.extract()?;
                let encoding_descr: String = tuple.get_item(1)?.extract()?;
                Ok(ZValue::new(Vec::from(buf).into()).encoding(encoding_descr.into()))
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

/// A Timestamp composed of a time and the identifier of the timestamp source.
#[pyclass]
#[derive(Debug, Clone, Copy)]
pub(crate) struct Timestamp {
    pub(crate) t: zenoh::time::Timestamp,
}

#[pymethods]
impl Timestamp {
    /// The time in seconds since the UNIX EPOCH (January 1, 1970, 00:00:00 (UTC))
    /// as a floating point number.
    ///
    /// :type: float
    #[getter]
    fn time(&self) -> f64 {
        self.t.get_time().to_duration().as_secs_f64()
    }

    /// The identifier of the timestamp source
    ///
    /// :type: bytes
    #[getter]
    fn id(&self) -> &[u8] {
        self.t.get_id().as_slice()
    }
}

#[pyproto]
impl PyObjectProtocol for Timestamp {
    fn __str__(&self) -> String {
        self.t.to_string()
    }
}

/// Some informations about the associated data
#[pyclass]
#[derive(Clone)]
pub(crate) struct SourceInfo {
    pub(crate) i: zenoh::prelude::SourceInfo,
}

#[pymethods]
impl SourceInfo {
    /// The :class:`PeerId` of the data source.
    ///
    /// :type: :class:`PeerId` or ``None``
    #[getter]
    fn source_id(&self) -> Option<PeerId> {
        self.i.source_id.as_ref().map(|p| PeerId { p: *p })
    }

    /// The source sequence number of the data.
    ///
    /// :type: int or ``None``
    #[getter]
    fn source_sn(&self) -> Option<ZInt> {
        self.i.source_sn
    }

    /// The :class:`PeerId` of the 1st router that routed the data.
    ///
    /// :type: :class:`PeerId` or ``None``
    #[getter]
    fn first_router_id(&self) -> Option<PeerId> {
        self.i.first_router_id.as_ref().map(|p| PeerId { p: *p })
    }

    /// The first router sequence number of the data.
    ///
    /// :type: int or ``None``
    #[getter]
    fn first_router_sn(&self) -> Option<ZInt> {
        self.i.first_router_sn
    }
}

/// A zenoh sample.
///
/// :param key_expr: the resource name
/// :type key_expr: str
/// :param payload: the data payload
/// :type payload: bytes
/// :param source_info: some information about the data
/// :type source_info: SourceInfo, optional
#[pyclass]
#[pyo3(text_signature = "(key_expr, payload, source_info=None)")]
#[derive(Clone)]
pub(crate) struct Sample {
    pub(crate) s: zenoh::prelude::Sample,
}

impl pyo3::conversion::ToPyObject for Sample {
    fn to_object(&self, py: Python) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(pyo3::Py::new(py, self.clone()).unwrap(), py)
    }
}

#[pymethods]
impl Sample {
    #[new]
    fn new(key_expr: &PyAny, payload: &PyAny) -> Self {
        let key_expr = zkey_expr_of_pyany(key_expr).unwrap();
        let payload = zvalue_of_pyany(payload).unwrap();
        Sample {
            s: zenoh::prelude::Sample::new(key_expr.to_owned(), payload),
        }
    }
    pub fn with_timestamp(&mut self, timestamp: Timestamp) {
        unsafe {
            let s = std::ptr::read(self);
            let s = s.s.with_timestamp(timestamp.t);
            std::ptr::write(self, Sample { s });
        }
    }
    pub fn with_source_info(&mut self, info: SourceInfo) {
        unsafe {
            let s = std::ptr::read(self);
            let s = s.s.with_source_info(info.i);
            std::ptr::write(self, Sample { s });
        }
    }

    /// The resource name
    ///
    /// :type: str
    #[getter]
    fn key_expr(&self) -> KeyExpr {
        self.s.key_expr.to_owned().into()
    }

    /// The data payload
    ///
    /// :type: bytes
    #[getter]
    fn payload<'a>(&self, py: Python<'a>) -> &'a PyBytes {
        PyBytes::new(py, self.s.value.payload.contiguous().as_slice())
    }

    /// The data payload
    ///
    /// :type: bytes
    #[getter]
    fn value(&self) -> Value {
        Value {
            v: self.s.value.clone(),
        }
    }

    /// The data payload
    ///
    /// :type: bytes
    #[getter]
    fn kind(&self) -> SampleKind {
        self.s.kind.into()
    }

    /// Some information about the data
    ///
    /// :type: :class:`SourceInfo` or ``None``
    #[getter]
    fn source_info(&self) -> Option<SourceInfo> {
        Some(SourceInfo {
            i: self.s.source_info.clone(),
        })
    }

    /// The timestamp
    ///
    /// :type: :class:`Timestamp` or ``None``
    #[getter]
    fn timestamp(&self) -> Option<Timestamp> {
        self.s.timestamp.map(|t| Timestamp { t })
    }
}

#[pyproto]
impl PyObjectProtocol for Sample {
    fn __str__(&self) -> String {
        format!("{:?}", self.s)
    }

    fn __repr__(&self) -> String {
        self.__str__()
    }
}

// zenoh.Reliability (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/834 to be fixed)
//
/// The kind of reliability
#[pyclass]
#[derive(Clone, Copy, PartialEq, Default)]
pub(crate) struct Reliability {
    pub(crate) r: zenoh::subscriber::Reliability,
}

#[allow(non_snake_case)]
#[pymethods]
impl Reliability {
    #[classattr]
    fn BestEffort() -> Reliability {
        Reliability {
            r: zenoh::subscriber::Reliability::BestEffort,
        }
    }

    #[classattr]
    fn Reliable() -> Reliability {
        Reliability {
            r: zenoh::subscriber::Reliability::Reliable,
        }
    }
}

// zenoh.SubMode (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/834 to be fixed)
//
/// The subscription mode.
#[pyclass]
#[derive(Clone, Default)]
pub(crate) struct SubMode {
    pub(crate) m: zenoh::subscriber::SubMode,
}

#[allow(non_snake_case)]
#[pymethods]
impl SubMode {
    #[classattr]
    fn Push() -> SubMode {
        SubMode {
            m: zenoh::subscriber::SubMode::Push,
        }
    }

    #[classattr]
    fn Pull() -> SubMode {
        SubMode {
            m: zenoh::subscriber::SubMode::Pull,
        }
    }
}

/// A time period.
#[pyclass]
#[pyo3(text_signature = "(origin, period, duration)")]
#[derive(Clone)]
pub(crate) struct Period {
    pub(crate) p: zenoh::time::Period,
}

#[pymethods]
impl Period {
    #[new]
    fn new(origin: ZInt, period: ZInt, duration: ZInt) -> Period {
        Period {
            p: zenoh::time::Period {
                origin,
                period,
                duration,
            },
        }
    }
}

pub(crate) enum ZnSubOps {
    Pull,
    Unregister,
}

/// A subscriber
#[pyclass]
pub(crate) struct Subscriber {
    pub(crate) unregister_tx: Sender<ZnSubOps>,
    pub(crate) loop_handle: Option<async_std::task::JoinHandle<()>>,
}

#[pymethods]
impl Subscriber {
    /// Pull available data for a pull-mode subscriber.
    fn pull(&self) {
        task::block_on(async {
            if let Err(e) = self.unregister_tx.send(ZnSubOps::Pull).await {
                warn!("Error in Subscriber::pull() : {}", e);
            }
        });
    }

    /// Close the subscriber.
    fn close(&mut self) {
        if let Some(handle) = self.loop_handle.take() {
            task::block_on(async {
                if let Err(e) = self.unregister_tx.send(ZnSubOps::Unregister).await {
                    warn!("Error in Subscriber::close() : {}", e);
                }
                handle.await;
            });
        }
    }
}

// zenoh.queryable (simulate the package as a class, and consts as class attributes)
//
/// Constants defining the different modes of a zenoh :class:`Queryable`.
#[allow(non_camel_case_types)]
#[pyclass]
pub(crate) struct queryable {}

#[allow(non_snake_case)]
#[pymethods]
impl queryable {
    #[classattr]
    fn ALL_KINDS() -> ZInt {
        zenoh::queryable::ALL_KINDS
    }

    #[classattr]
    fn STORAGE() -> ZInt {
        zenoh::queryable::STORAGE
    }

    #[classattr]
    fn EVAL() -> ZInt {
        zenoh::queryable::EVAL
    }
}

/// Type received by a queryable callback. See :meth:`Session.register_queryable`.
#[pyclass]
#[derive(Clone)]
pub(crate) struct Query {
    pub(crate) q: async_std::sync::Arc<zenoh::queryable::Query>,
}

impl pyo3::conversion::ToPyObject for Query {
    fn to_object(&self, py: Python) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(pyo3::Py::new(py, self.clone()).unwrap(), py)
    }
}

#[pymethods]
impl Query {
    /// The key_selector of the query
    ///
    /// :type: String
    #[getter]
    fn selector(&self) -> String {
        self.q.selector().to_string()
    }

    /// The key_selector of the query
    ///
    /// :type: KeyExpr
    #[getter]
    fn key_selector(&self) -> KeyExpr {
        self.q.selector().key_selector.to_owned().into()
    }

    /// The value_selector of the query
    ///
    /// :type: str
    #[getter]
    fn value_selector(&self) -> &str {
        self.q.selector().value_selector
    }

    /// Send a reply to the query
    ///
    /// :param sample: the reply sample
    /// :type: Sample
    #[pyo3(text_signature = "(self, sample)")]
    fn reply(&self, sample: Sample) {
        self.q.reply(sample.s);
    }
}

/// An entity able to reply to queries.
#[pyclass]
pub(crate) struct Queryable {
    pub(crate) unregister_tx: Sender<bool>,
    pub(crate) loop_handle: Option<async_std::task::JoinHandle<()>>,
}

#[pymethods]
impl Queryable {
    /// Close the queryable.
    fn close(&mut self) {
        if let Some(handle) = self.loop_handle.take() {
            task::block_on(async {
                if let Err(e) = self.unregister_tx.send(true).await {
                    warn!("Error in Queryable::close() : {}", e);
                }
                handle.await;
            });
        }
    }
}

// zenoh.Target (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/834 to be fixed)
//
/// The queryables that should be target of a :class:`Query`
#[pyclass]
#[derive(Clone)]
pub(crate) struct Target {
    pub(crate) t: zenoh::query::Target,
}

#[allow(non_snake_case)]
#[pymethods]
impl Target {
    #[staticmethod]
    fn BestMatching() -> Target {
        Target {
            t: zenoh::query::Target::BestMatching,
        }
    }

    #[cfg(features = "complete_n")]
    #[staticmethod]
    #[pyo3(text_signature = "(n)")]
    fn Complete(n: ZInt) -> Target {
        Target {
            t: zenoh::query::Target::Complete { n },
        }
    }

    #[staticmethod]
    fn All() -> Target {
        Target {
            t: zenoh::query::Target::All,
        }
    }

    #[staticmethod]
    fn AllComplete() -> Target {
        Target {
            t: zenoh::query::Target::AllComplete,
        }
    }

    #[staticmethod]
    fn No() -> Target {
        Target {
            t: zenoh::query::Target::None,
        }
    }
}

/// The queryables that should be target of a :class:`Query`.
///
/// :param kind: the kind of queryable (one constant from :class:`queryable`)
/// :type kind: int, optional
/// :param target: a characteristic of the queryable.
/// :type target: Target, optional
#[pyclass]
#[pyo3(text_signature = "(kind=None, target=None)")]
#[derive(Clone)]
pub(crate) struct QueryTarget {
    pub(crate) t: zenoh::query::QueryTarget,
}

#[pymethods]
impl QueryTarget {
    #[new]
    fn new(kind: Option<ZInt>, target: Option<Target>) -> QueryTarget {
        let mut t = zenoh::query::QueryTarget::default();
        if let Some(k) = kind {
            t.kind = k;
        }
        if let Some(target) = target {
            t.target = target.t;
        }
        QueryTarget { t }
    }
}

impl Default for QueryTarget {
    fn default() -> Self {
        QueryTarget {
            t: zenoh::query::QueryTarget::default(),
        }
    }
}

// zenoh.QueryConsolidation (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/834 to be fixed)
//
/// The kind of consolidation that should be applied on replies to a :meth:`Session.get`.
#[pyclass]
#[derive(Clone)]
pub(crate) struct ConsolidationMode {
    pub(crate) c: zenoh::query::ConsolidationMode,
}

#[allow(non_snake_case)]
#[pymethods]
impl ConsolidationMode {
    #[classattr]
    fn No() -> ConsolidationMode {
        ConsolidationMode {
            c: zenoh::query::ConsolidationMode::None,
        }
    }

    #[classattr]
    fn Lazy() -> ConsolidationMode {
        ConsolidationMode {
            c: zenoh::query::ConsolidationMode::Lazy,
        }
    }

    #[classattr]
    fn Full() -> ConsolidationMode {
        ConsolidationMode {
            c: zenoh::query::ConsolidationMode::Full,
        }
    }
}

// zenoh.QueryConsolidation (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/834 to be fixed)
//
/// The kind of consolidation that should be applied on replies to a :meth:`Session.get`
/// at the different stages of the reply process.
///
/// :param first_routers: the consolidation mode to apply on first routers of the replies routing path (default: :attr:`ConsolidationMode.Lazy`)
/// :type first_routers: ConsolidationMode, optional
/// :param last_router: the consolidation mode to apply on last router of the replies routing path (default: :attr:`ConsolidationMode.Lazy`)
/// :type last_router: ConsolidationMode, optional
/// :param reception: the consolidation mode to apply at reception of the replies (default: :attr:`ConsolidationMode.Full`)
/// :type reception: ConsolidationMode, optional
#[pyclass]
#[pyo3(text_signature = "(first_routers=None, last_router=None, reception=None)")]
#[derive(Clone)]
pub(crate) struct QueryConsolidation {
    pub(crate) c: zenoh::query::QueryConsolidation,
}

#[pymethods]
impl QueryConsolidation {
    #[new]
    fn new(
        first_routers: Option<ConsolidationMode>,
        last_router: Option<ConsolidationMode>,
        reception: Option<ConsolidationMode>,
    ) -> QueryConsolidation {
        let mut c = zenoh::query::QueryConsolidation::default();
        if let Some(f) = first_routers {
            c.first_routers = f.c;
        }
        if let Some(l) = last_router {
            c.last_router = l.c;
        }
        if let Some(r) = reception {
            c.reception = r.c;
        }
        QueryConsolidation { c }
    }
}

impl Default for QueryConsolidation {
    fn default() -> Self {
        QueryConsolidation {
            c: zenoh::query::QueryConsolidation::default(),
        }
    }
}

/// Type received by a get callback. See :meth:`Session.get`.
#[pyclass]
#[derive(Clone)]
pub(crate) struct Reply {
    pub(crate) r: zenoh::query::Reply,
}

impl pyo3::conversion::ToPyObject for Reply {
    fn to_object(&self, py: Python) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(pyo3::Py::new(py, self.clone()).unwrap(), py)
    }
}

#[pymethods]
impl Reply {
    /// The data
    ///
    /// :type: Sample
    #[getter]
    fn data(&self) -> Sample {
        Sample {
            s: self.r.data.clone(),
        }
    }

    /// The kind of reply source
    ///
    /// :type: int
    #[getter]
    fn replier_kind(&self) -> ZInt {
        self.r.replier_kind
    }

    /// The identifier of reply source
    ///
    /// :type: PeerId
    fn replier_id(&self) -> PeerId {
        PeerId {
            p: self.r.replier_id,
        }
    }
}

// zenoh.CongestionControl (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/834 to be fixed)
//
/// The kind of congestion control.
#[pyclass]
#[derive(Clone)]
pub struct CongestionControl {
    pub(crate) cc: zenoh::publication::CongestionControl,
}

#[allow(non_snake_case)]
#[pymethods]
impl CongestionControl {
    #[classattr]
    fn Drop() -> CongestionControl {
        CongestionControl {
            cc: zenoh::publication::CongestionControl::Drop,
        }
    }
    #[classattr]
    fn Block() -> CongestionControl {
        CongestionControl {
            cc: zenoh::publication::CongestionControl::Block,
        }
    }
}

impl Default for CongestionControl {
    fn default() -> Self {
        CongestionControl {
            cc: zenoh::publication::CongestionControl::default(),
        }
    }
}

#[pyclass]
#[derive(Clone, Default)]
pub struct Priority {
    pub(crate) p: zenoh::prelude::Priority,
}

#[allow(non_snake_case)]
#[pymethods]
impl Priority {
    #[classattr]
    fn Background() -> Self {
        Priority {
            p: zenoh::prelude::Priority::Background,
        }
    }
    #[classattr]
    fn Data() -> Self {
        Priority {
            p: zenoh::prelude::Priority::Data,
        }
    }
    #[classattr]
    fn DataHigh() -> Self {
        Priority {
            p: zenoh::prelude::Priority::DataHigh,
        }
    }
    #[classattr]
    fn DataLow() -> Self {
        Priority {
            p: zenoh::prelude::Priority::DataLow,
        }
    }
    #[classattr]
    fn InteractiveHigh() -> Self {
        Priority {
            p: zenoh::prelude::Priority::InteractiveHigh,
        }
    }
    #[classattr]
    fn InteractiveLow() -> Self {
        Priority {
            p: zenoh::prelude::Priority::InteractiveLow,
        }
    }
    #[classattr]
    fn RealTime() -> Self {
        Priority {
            p: zenoh::prelude::Priority::RealTime,
        }
    }
}
