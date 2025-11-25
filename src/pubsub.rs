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
    IntoPyObjectExt,
};

use crate::{
    bytes::{Encoding, ZBytes},
    handlers::{into_handler, HandlerImpl},
    key_expr::KeyExpr,
    macros::{build, option_wrapper},
    matching::{MatchingListener, MatchingStatus},
    qos::{CongestionControl, Priority, Reliability},
    sample::{Sample, SourceInfo},
    session::EntityGlobalId,
    time::Timestamp,
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
    fn id(&self) -> PyResult<EntityGlobalId> {
        Ok(self.get_ref()?.id().into())
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

    #[getter]
    fn matching_status(&self, py: Python) -> PyResult<MatchingStatus> {
        Ok(wait(py, self.get_ref()?.matching_status())?.into())
    }

    #[pyo3(signature = (payload, *, encoding = None, attachment = None, timestamp = None, source_info = None))]
    fn put(
        &self,
        py: Python,
        #[pyo3(from_py_with = ZBytes::from_py)] payload: ZBytes,
        #[pyo3(from_py_with = Encoding::from_py_opt)] encoding: Option<Encoding>,
        #[pyo3(from_py_with = ZBytes::from_py_opt)] attachment: Option<ZBytes>,
        timestamp: Option<Timestamp>,
        source_info: Option<SourceInfo>,
    ) -> PyResult<()> {
        let this = self.get_ref()?;
        let builder = build!(
            this.put(payload),
            encoding,
            attachment,
            timestamp,
            source_info
        );
        wait(py, builder)
    }

    #[pyo3(signature = (*, attachment = None, timestamp = None, source_info = None))]
    fn delete(
        &self,
        py: Python,
        #[pyo3(from_py_with = ZBytes::from_py_opt)] attachment: Option<ZBytes>,
        timestamp: Option<Timestamp>,
        source_info: Option<SourceInfo>,
    ) -> PyResult<()> {
        let builder = build!(self.get_ref()?.delete(), attachment, timestamp, source_info);
        wait(py, builder)
    }

    #[pyo3(signature = (handler = None))]
    fn declare_matching_listener(
        &self,
        py: Python,
        handler: Option<&Bound<PyAny>>,
    ) -> PyResult<MatchingListener> {
        let (handler, background) = into_handler(py, handler, None)?;
        let mut listener = wait(py, self.get_ref()?.matching_listener().with(handler))?;
        if background {
            listener.set_background(true);
        }
        Ok(listener.into())
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
    fn id(&self) -> PyResult<EntityGlobalId> {
        Ok(self.get_ref()?.id().into())
    }

    #[getter]
    fn key_expr(&self) -> PyResult<KeyExpr> {
        Ok(self.get_ref()?.key_expr().clone().into())
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
