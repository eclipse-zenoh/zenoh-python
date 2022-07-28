use pyo3::{
    prelude::*,
    types::{PyBytes, PyString},
};
use zenoh::{
    prelude::{Encoding, KeyExpr, Sample, Value},
    query::Reply,
};
use zenoh_buffers::{SplitBuffer, ZBuf};

use crate::{
    enums::{_Encoding, _SampleKind},
    keyexpr::_KeyExpr,
};

#[derive(Clone)]
pub(crate) enum Payload {
    Zenoh(ZBuf),
    Python(Py<PyBytes>),
}
impl Payload {
    pub(crate) fn into_zbuf(self) -> ZBuf {
        match self {
            Payload::Zenoh(buf) => buf,
            Payload::Python(buf) => Python::with_gil(|py| ZBuf::from(buf.as_bytes(py).to_owned())),
        }
    }
    pub(crate) fn into_pybytes(self) -> Py<PyBytes> {
        match self {
            Payload::Zenoh(buf) => {
                Python::with_gil(|py| Py::from(PyBytes::new(py, buf.contiguous().as_ref())))
            }
            Payload::Python(buf) => buf,
        }
    }
}
impl From<ZBuf> for Payload {
    fn from(buf: ZBuf) -> Self {
        Payload::Zenoh(buf)
    }
}
impl From<Py<PyBytes>> for Payload {
    fn from(buf: Py<PyBytes>) -> Self {
        Payload::Python(buf)
    }
}
#[pyclass(subclass)]
#[derive(Clone)]
pub struct _Value {
    pub(crate) payload: Payload,
    pub(crate) encoding: Encoding,
}
#[pymethods]
impl _Value {
    #[new]
    pub fn pynew(this: Self) -> Self {
        this
    }
    #[staticmethod]
    pub fn new(payload: Py<PyBytes>) -> Self {
        Self {
            payload: payload.into(),
            encoding: Encoding::EMPTY,
        }
    }
    #[getter]
    pub fn payload(&mut self) -> Py<PyBytes> {
        if let Payload::Python(buf) = &self.payload {
            return buf.clone();
        }
        let payload = unsafe { std::ptr::read(&self.payload) };
        let buf = payload.into_pybytes();
        unsafe { std::ptr::write(&mut self.payload, Payload::Python(buf.clone())) };
        buf
    }
    pub fn with_payload(&mut self, payload: Py<PyBytes>) {
        self.payload = Payload::Python(payload)
    }
    #[getter]
    pub fn encoding(&self) -> _Encoding {
        _Encoding(self.encoding.clone())
    }
    pub fn with_encoding(&mut self, encoding: _Encoding) {
        self.encoding = encoding.0;
    }
}
impl From<Value> for _Value {
    fn from(value: Value) -> Self {
        _Value {
            payload: value.payload.into(),
            encoding: value.encoding,
        }
    }
}
impl From<_Value> for Value {
    fn from(value: _Value) -> Self {
        Value::new(value.payload.into_zbuf()).encoding(value.encoding)
    }
}

pub(crate) trait PyAnyToValue {
    fn to_value(self) -> PyResult<Value>;
}
impl PyAnyToValue for &PyAny {
    fn to_value(self) -> PyResult<Value> {
        let encoding: _Encoding = self.getattr("encoding")?.extract()?;
        let payload: &PyBytes = self.getattr("payload")?.extract()?;
        Ok(Value::new(ZBuf::from(payload.as_bytes().to_owned())).encoding(encoding.0))
    }
}

#[pyclass(subclass)]
#[derive(Clone)]
pub struct _Sample {
    key_expr: KeyExpr<'static>,
    value: _Value,
    kind: _SampleKind,
}
impl From<Sample> for _Sample {
    fn from(sample: Sample) -> Self {
        let Sample {
            key_expr,
            value,
            kind,
            ..
        } = sample;
        _Sample {
            key_expr,
            value: value.into(),
            kind: _SampleKind(kind),
        }
    }
}
#[pymethods]
impl _Sample {
    #[new]
    pub fn pynew(this: Self) -> Self {
        this
    }
    #[getter]
    pub fn value(&self) -> _Value {
        self.value.clone()
    }
    #[getter]
    pub fn key_expr(&self) -> _KeyExpr {
        _KeyExpr(self.key_expr.clone())
    }
    #[getter]
    pub fn payload(&mut self) -> Py<PyBytes> {
        if let Payload::Python(buf) = &self.value.payload {
            return buf.clone();
        }
        let payload = unsafe { std::ptr::read(&self.value.payload) };
        let buf = payload.into_pybytes();
        unsafe { std::ptr::write(&mut self.value.payload, Payload::Python(buf.clone())) };
        buf
    }
    #[getter]
    pub fn encoding(&self) -> _Encoding {
        _Encoding(self.value.encoding.clone())
    }
    #[getter]
    pub fn kind(&self) -> _SampleKind {
        self.kind.clone()
    }
}

impl From<_Sample> for Sample {
    fn from(sample: _Sample) -> Self {
        let mut value = Sample::new(sample.key_expr, sample.value);
        value.kind = sample.kind.0;
        value
    }
}

#[pyclass(subclass)]
#[derive(Clone)]
pub struct _Reply {
    #[pyo3(get)]
    pub replier_id: Py<PyString>,
    pub reply: Result<_Sample, _Value>,
}
#[pymethods]
impl _Reply {
    #[new]
    pub fn pynew(this: Self) -> Self {
        this
    }
    #[getter]
    pub fn ok(&self) -> Option<_Sample> {
        self.reply.as_ref().ok().map(Clone::clone)
    }
    #[getter]
    pub fn err(&self) -> Option<_Sample> {
        self.reply.as_ref().ok().map(Clone::clone)
    }
}
impl From<Reply> for _Reply {
    fn from(reply: Reply) -> Self {
        let replier_id = reply.replier_id.to_string();
        let replier_id = Python::with_gil(|py| Py::from(PyString::new(py, &replier_id)));
        _Reply {
            replier_id,
            reply: match reply.sample {
                Ok(o) => Ok(o.into()),
                Err(e) => Err(e.into()),
            },
        }
    }
}
