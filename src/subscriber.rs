pub(crate) use pyo3::prelude::*;
use pyo3::types::{PyDict, PyIterator, PyTuple, PyType};

use crate::{
    handlers::HandlerImpl,
    key_expr::KeyExpr,
    sample::Sample,
    utils::{allow_threads, generic, opt_wrapper, r#enum, PySyncResolve},
};

r#enum!(zenoh::subscriber::Reliability: u8 {BestEffort, Reliable});

#[pymethods]
impl Reliability {
    #[new]
    fn new() -> Self {
        Self::DEFAULT
    }

    #[classattr]
    const DEFAULT: Self = Self::BestEffort;
}

opt_wrapper!(
    zenoh::subscriber::Subscriber<'static, HandlerImpl<Sample>>,
    "Undeclared subscriber"
);

#[pymethods]
impl Subscriber {
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
    ) -> PyResult<()> {
        self.undeclare(py)
    }

    #[getter]
    fn key_expr(&self) -> PyResult<KeyExpr> {
        Ok(self.get_ref()?.key_expr().clone().into())
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
        allow_threads(py, || self.take()?.undeclare().py_res_sync())
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        self.handler(py)?.bind(py).iter()
    }
}
