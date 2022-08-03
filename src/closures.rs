use std::convert::TryFrom;

use pyo3::{prelude::*, types::PyTuple};

pub(crate) struct PyClosure<I> {
    pub(crate) pycall: Py<PyAny>,
    pub(crate) drop: Option<Py<PyAny>>,
    _marker: std::marker::PhantomData<I>,
}
impl<I> TryFrom<&PyAny> for PyClosure<I> {
    type Error = PyErr;
    fn try_from(value: &PyAny) -> Result<Self, Self::Error> {
        Python::with_gil(|py| {
            let pycall = match value.getattr("call") {
                Ok(value) => value.into_py(py),
                Err(e) => return Err(e),
            };
            let drop = match value.getattr("drop") {
                Ok(value) => {
                    if value.is_none() {
                        None
                    } else {
                        Some(value.into_py(py))
                    }
                }
                Err(e) => return Err(e),
            };
            Ok(PyClosure {
                pycall,
                drop,
                _marker: std::marker::PhantomData,
            })
        })
    }
}
impl<I: IntoPy<Py<PyTuple>>> PyClosure<I> {
    pub fn call(&self, args: I) -> PyResult<PyObject> {
        Python::with_gil(|py| self.pycall.call1(py, args))
    }
}
impl<I> Drop for PyClosure<I> {
    fn drop(&mut self) {
        if let Some(drop) = self.drop.take() {
            Python::with_gil(|py| drop.call0(py)).unwrap();
        }
    }
}
