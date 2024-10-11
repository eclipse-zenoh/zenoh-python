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
    types::{PyDict, PyIterator, PyTuple, PyType},
};

use crate::{
    bytes::{Encoding, ZBytes},
    handlers::HandlerImpl,
    key_expr::KeyExpr,
    macros::{build, option_wrapper},
    qos::{CongestionControl, Priority, Reliability},
    sample::Sample,
    utils::{generic, wait},
};

option_wrapper!(zenoh::pubsub::Publisher<'static>, "Undeclared publisher");

#[pymethods]
impl Publisher {
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
        self.undeclare(py)?;
        Ok(py.None())
    }

    #[getter]
    fn key_expr(&self) -> PyResult<KeyExpr> {
        Ok(self.get_ref()?.key_expr().clone().into())
    }

    #[getter]
    fn encoding(&self) -> PyResult<Encoding> {
        Ok(self.get_ref()?.encoding().clone().into())
    }

    #[getter]
    fn congestion_control(&self) -> PyResult<CongestionControl> {
        Ok(self.get_ref()?.congestion_control().into())
    }

    #[getter]
    fn priority(&self) -> PyResult<Priority> {
        Ok(self.get_ref()?.priority().into())
    }

    #[getter]
    fn reliability(&self) -> PyResult<Reliability> {
        Ok(self.get_ref()?.reliability().into())
    }

    // TODO add timestamp
    #[pyo3(signature = (payload, *, encoding = None, attachment = None))]
    fn put(
        &self,
        py: Python,
        #[pyo3(from_py_with = "ZBytes::from_py")] payload: ZBytes,
        #[pyo3(from_py_with = "Encoding::from_py_opt")] encoding: Option<Encoding>,
        #[pyo3(from_py_with = "ZBytes::from_py_opt")] attachment: Option<ZBytes>,
    ) -> PyResult<()> {
        let this = self.get_ref()?;
        wait(py, build!(this.put(payload), encoding, attachment))
    }

    #[pyo3(signature = (*, attachment = None))]
    fn delete(
        &self,
        py: Python,
        #[pyo3(from_py_with = "ZBytes::from_py_opt")] attachment: Option<ZBytes>,
    ) -> PyResult<()> {
        wait(py, build!(self.get_ref()?.delete(), attachment))
    }

    fn undeclare(&mut self, py: Python) -> PyResult<()> {
        wait(py, self.take()?.undeclare())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.get_ref()?))
    }
}

option_wrapper!(
    zenoh::pubsub::Subscriber<HandlerImpl<Sample>>,
    "Undeclared subscriber"
);

#[pymethods]
impl Subscriber {
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
        wait(py, self.take()?.undeclare())
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        self.handler(py)?.bind(py).iter()
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.get_ref()?))
    }
}
