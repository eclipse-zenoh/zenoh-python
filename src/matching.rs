//
// Copyright (c) 2025 ZettaScale Technology
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
    types::{PyDict, PyIterator, PyTuple, PyType},
    IntoPyObjectExt,
};

use crate::{
    handlers::HandlerImpl,
    macros::{option_wrapper, wrapper},
    utils::{generic, wait},
};

wrapper!(zenoh::matching::MatchingStatus);

#[pymethods]
impl MatchingStatus {
    #[getter]
    fn matching(&self) -> bool {
        self.0.matching()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

option_wrapper!(
    zenoh::matching::MatchingListener<HandlerImpl<MatchingStatus>>,
    "Undeclared matching listener"
);

#[pymethods]
impl MatchingListener {
    #[classmethod]
    fn __class_getitem__(cls: &Bound<PyType>, args: &Bound<PyAny>) -> PyObject {
        generic(cls, args)
    }

    fn __enter__<'a, 'py>(this: &'a Bound<'py, Self>) -> &'a Bound<'py, Self> {
        this
    }

    #[pyo3(signature = (*_args, **_kwargs))]
    fn __exit__(
        &mut self,
        py: Python,
        _args: &Bound<PyTuple>,
        _kwargs: Option<&Bound<PyDict>>,
    ) -> PyResult<PyObject> {
        self.undeclare(py)?;
        Ok(py.None())
    }

    #[getter]
    fn handler(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.handler().into_py_any(py)
    }

    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.handler().try_recv(py)
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.handler().recv(py)
    }

    fn undeclare(&mut self, py: Python) -> PyResult<()> {
        wait(py, self.take()?.undeclare())
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        self.handler(py)?.bind(py).try_iter()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.get_ref()?))
    }
}
