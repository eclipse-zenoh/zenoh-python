use std::borrow::Cow;

use pyo3::{prelude::*, types::PyBytes};
use zenoh::prelude::Value;
use zenoh_buffers::{SplitBuffer, ZBuf};

use crate::enums::_Encoding;

#[pyclass(subclass)]
#[derive(Clone)]
pub struct _Value {
    inner: Value,
    payload: Option<Cow<'static, [u8]>>,
}
#[pymethods]
impl _Value {
    pub fn payload<'a>(&mut self, py: Python<'a>) -> &'a PyBytes {
        if self.payload.is_none() {
            self.payload = Some(unsafe { std::mem::transmute(self.inner.payload.contiguous()) })
        }
        unsafe { PyBytes::new(py, self.payload.as_ref().unwrap_unchecked()) }
    }
    pub fn encoding(&self) -> _Encoding {
        _Encoding(self.inner.encoding.clone())
    }
}

pub(crate) trait PyAnyToValue {
    fn to_value(self) -> PyResult<zenoh::prelude::Value>;
}
impl PyAnyToValue for &PyAny {
    fn to_value(self) -> PyResult<zenoh::prelude::Value> {
        let encoding: _Encoding = self.call_method0("_encoding")?.extract()?;
        let payload: &PyBytes = self.call_method0("payload")?.extract()?;
        Ok(Value::new(ZBuf::from(payload.as_bytes().to_owned())).encoding(encoding.0))
    }
}
