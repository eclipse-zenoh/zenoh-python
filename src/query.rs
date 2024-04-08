use pyo3::prelude::*;

use crate::{
    config::ZenohId,
    utils::{r#enum, wrapper, IntoPython},
};

r#enum!(zenoh::query::QueryTarget: u8 {
    BestMatching,
    All,
    AllComplete,
});

#[pymethods]
impl QueryTarget {
    #[classattr]
    const DEFAULT: Self = Self::BestMatching;
}

r#enum!(zenoh::query::ConsolidationMode: u8 {
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
    fn replier_id(&self) -> ZenohId {
        self.0.replier_id().into()
    }
}
