//
// Copyright (c) 2024 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use pyo3::{
    prelude::*,
    types::{PyDict, PyIterator, PySet, PyTuple, PyType},
};

use crate::{
    handlers::HandlerImpl,
    macros::option_wrapper,
    query::Query,
    utils::{generic, wait},
};

#[pyclass]
pub(crate) struct Queryable {
    pub(crate) queryable: Option<zenoh::queryable::Queryable<'static, HandlerImpl<Query>>>,
    pub(crate) session_pool: Py<PySet>,
}

option_wrapper!(
    Queryable.queryable: zenoh::queryable::Queryable<'static, HandlerImpl<Query>>,
    "Undeclared queryable"
);

#[pymethods]
impl Queryable {
    fn _drop(&mut self) {
        self.wait_drop();
    }

    #[classmethod]
    fn __class_getitem__(cls: &Bound<PyType>, args: &Bound<PyAny>) -> PyObject {
        generic(cls, args)
    }

    fn __enter__<'a, 'py>(this: &'a Bound<'py, Self>) -> PyResult<&'a Bound<'py, Self>> {
        Self::check(this)
    }

    #[pyo3(signature = (*_args, **_kwargs))]
    fn __exit__(
        this: &Bound<Self>,
        _args: &Bound<PyTuple>,
        _kwargs: Option<&Bound<PyDict>>,
    ) -> PyResult<PyObject> {
        Self::undeclare(this)?;
        Ok(this.py().None())
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

    fn undeclare(this: &Bound<Self>) -> PyResult<()> {
        this.borrow()
            .session_pool
            .bind(this.py())
            .discard(this.into_py(this.py()))?;
        let queryable = this.borrow_mut().take()?;
        wait(this.py(), || queryable.undeclare())
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        self.handler(py)?.bind(py).iter()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.get_ref()?))
    }
}
