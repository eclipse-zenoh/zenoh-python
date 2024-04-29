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
pub(crate) use pyo3::prelude::*;
use pyo3::types::{PyDict, PyIterator, PyTuple, PyType};

use crate::{
    handlers::HandlerImpl,
    key_expr::KeyExpr,
    resolve::{resolve, Resolve},
    sample::Sample,
    utils::{droppable_wrapper, enum_mapper, generic},
};

enum_mapper!(zenoh::subscriber::Reliability: u8 {BestEffort, Reliable});

#[pymethods]
impl Reliability {
    #[classattr]
    const DEFAULT: Self = Self::BestEffort;
}

droppable_wrapper!(
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
    ) -> PyResult<PyObject> {
        self.undeclare(py)?.wait(py)
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

    fn undeclare(&mut self, py: Python) -> PyResult<Resolve> {
        let this = self.take()?;
        resolve(py, || this.undeclare())
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        self.handler(py)?.bind(py).iter()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
