use std::convert::TryFrom;

use pyo3::{prelude::*, types::PyTuple};

#[pyclass(subclass)]
pub struct _Closure {
    pub(crate) pycall: Py<PyAny>,
    pub(crate) drop: Option<Py<PyAny>>,
}
impl TryFrom<&PyAny> for _Closure {
    type Error = PyErr;
    fn try_from(value: &PyAny) -> Result<Self, Self::Error> {
        Python::with_gil(|py| {
            let pycall = match value.call_method0("call") {
                Ok(value) => value.into_py(py),
                Err(e) => return Err(e),
            };
            let drop = match value.call_method0("drop") {
                Ok(value) => {
                    if value.is_none() {
                        None
                    } else {
                        Some(value.into_py(py))
                    }
                }
                Err(e) => return Err(e),
            };
            Ok(_Closure { pycall, drop })
        })
    }
}
impl _Closure {
    pub fn call<I: IntoPy<Py<PyTuple>>>(&self, args: I) -> PyResult<PyObject> {
        Python::with_gil(|py| self.pycall.call1(py, args))
    }
}
impl Drop for _Closure {
    fn drop(&mut self) {
        if let Some(drop) = self.drop.take() {
            Python::with_gil(|py| drop.call0(py)).unwrap();
        }
    }
}
