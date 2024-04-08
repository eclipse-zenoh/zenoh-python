use pyo3::prelude::*;

use crate::{
    key_expr::KeyExpr,
    utils::{try_downcast_or_parse, wrapper},
};

wrapper!(zenoh::selector::Selector<'static>: Clone);

#[pymethods]
impl Selector {
    #[new]
    pub(crate) fn new(selector: &Bound<PyAny>) -> PyResult<Self> {
        try_downcast_or_parse!(selector)
    }

    #[getter]
    fn key_expr(&self) -> KeyExpr {
        self.0.key_expr().clone().into_owned().into()
    }

    // TODO parameters

    // TODO time_range

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
}
