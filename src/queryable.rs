use std::sync::Arc;

use pyo3::prelude::*;
use zenoh::queryable::{CallbackQueryable, Query};

#[pyclass(subclass)]
#[derive(Clone)]
pub struct _Query(pub(crate) Arc<Query>);
#[pymethods]
impl _Query {
    #[new]
    pub fn pynew(this: Self) -> Self {
        this
    }
}

#[pyclass(subclass)]
pub struct _Queryable(pub(crate) CallbackQueryable<'static>);
