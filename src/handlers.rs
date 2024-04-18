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
use std::marker::PhantomData;

use pyo3::{exceptions::PyAttributeError, prelude::*};
use zenoh::handlers::{Callback, Dyn, IntoHandler};

use crate::utils::{allow_threads, bail, IntoPython, IntoRust, MapIntoPy, ToPyResult};

#[pyclass]
#[derive(Clone)]
pub(crate) struct DefaultHandler;

impl IntoRust for DefaultHandler {
    type Into = zenoh::handlers::DefaultHandler;

    fn into_rust(self) -> Self::Into {
        Self::Into::default()
    }
}

#[pymethods]
impl DefaultHandler {
    #[new]
    fn new() -> Self {
        Self
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct FifoChannel(usize);

impl IntoRust for FifoChannel {
    type Into = zenoh::handlers::FifoChannel;

    fn into_rust(self) -> Self::Into {
        Self::Into::new(self.0)
    }
}

#[pymethods]
impl FifoChannel {
    #[new]
    fn new(capacity: usize) -> Self {
        Self(capacity)
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct RingChannel(usize);

impl IntoRust for RingChannel {
    type Into = zenoh::handlers::RingChannel;

    fn into_rust(self) -> Self::Into {
        Self::Into::new(self.0)
    }
}

#[pymethods]
impl RingChannel {
    #[new]
    fn new(capacity: usize) -> Self {
        Self(capacity)
    }
}

pub(crate) trait Receiver {
    fn try_recv(&self, py: Python) -> PyResult<PyObject>;
    fn recv(&self, py: Python) -> PyResult<PyObject>;
}

#[pyclass]
pub(crate) struct Handler(Box<dyn Receiver + Send + Sync>);

#[pymethods]
impl Handler {
    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        self.0.try_recv(py)
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        dbg!(self.0.recv(py))
    }

    fn __iter__(this: Py<Self>) -> Py<Self> {
        this
    }

    fn __next__(&self, py: Python) -> Option<PyObject> {
        self.0.recv(py).ok()
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct CallbackDrop {
    callback: PyObject,
    drop: PyObject,
}

#[pymethods]
impl CallbackDrop {
    #[new]
    fn new(callback: PyObject, drop: PyObject) -> Self {
        Self { callback, drop }
    }

    fn __call__(&self, arg: &Bound<PyAny>) -> PyResult<PyObject> {
        self.callback.call1(arg.py(), (arg,))
    }

    fn drop(&self, py: Python) -> PyResult<PyObject> {
        self.drop.call0(py)
    }
}

pub(crate) enum PythonCallback {
    Simple(PyObject),
    WithDrop { callback: PyObject, drop: PyObject },
}

impl PythonCallback {
    fn new(obj: &Bound<PyAny>) -> PyResult<Self> {
        if let Ok(CallbackDrop { callback, drop }) = CallbackDrop::extract_bound(obj) {
            return Ok(Self::WithDrop { callback, drop });
        }
        let callback = obj.clone().unbind();
        Ok(if obj.hasattr("drop").unwrap_or(false) {
            Self::WithDrop {
                callback,
                drop: obj.getattr("drop")?.unbind(),
            }
        } else {
            PythonCallback::Simple(callback)
        })
    }
    fn call<T: IntoPython>(&self, t: T) {
        let callback = match self {
            Self::Simple(cb) => cb,
            Self::WithDrop { callback, .. } => callback,
        };
        let _ = Python::with_gil(|gil| PyResult::Ok(callback.call1(gil, (t.into_pyobject(gil),))));
    }
}

impl Drop for PythonCallback {
    fn drop(&mut self) {
        if let Self::WithDrop { drop, .. } = self {
            let _ = Python::with_gil(|gil| PyResult::Ok(drop.call0(gil)));
        }
    }
}

pub(crate) enum IntoHandlerImpl<T: IntoRust> {
    Rust {
        callback: Callback<'static, T::Into>,
        handler: Py<Handler>,
    },
    Python {
        callback: PythonCallback,
        handler: PyObject,
    },
}

impl<T: IntoPython> IntoHandler<'static, T> for IntoHandlerImpl<T::Into>
where
    T::Into: IntoRust<Into = T>,
{
    type Handler = HandlerImpl<T::Into>;

    fn into_handler(self) -> (Callback<'static, T>, Self::Handler) {
        match self {
            Self::Rust { callback, handler } => (callback, HandlerImpl::Rust(handler, PhantomData)),
            Self::Python { callback, handler } => (
                Dyn::new(move |t| callback.call(t)),
                HandlerImpl::Python(handler),
            ),
        }
    }
}

pub(crate) enum HandlerImpl<T> {
    // PhantomData just for documentation, until pyo3 accepts generic classes
    Rust(Py<Handler>, PhantomData<T>),
    Python(PyObject),
}

impl<T: IntoRust> IntoPy<PyObject> for HandlerImpl<T>
where
    T::Into: IntoPython<Into = T>,
{
    fn into_py(self, _: Python<'_>) -> PyObject {
        match self {
            Self::Rust(obj, _) => obj.into_any(),
            Self::Python(obj) => obj,
        }
    }
}

impl<T: IntoRust> ToPyObject for HandlerImpl<T>
where
    T::Into: IntoPython,
{
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            Self::Rust(obj, _) => obj.clone_ref(py).into_any(),
            Self::Python(obj) => obj.clone_ref(py),
        }
    }
}

impl<T: IntoRust> HandlerImpl<T>
where
    T::Into: IntoPython,
{
    pub(crate) fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        match self {
            Self::Rust(handler, _) => handler.borrow(py).try_recv(py),
            _ => Err(PyAttributeError::new_err(
                "No method 'try_recv'. For Python receiver, use 'receiver' attribute",
            )),
        }
    }

    pub(crate) fn recv(&self, py: Python) -> PyResult<PyObject> {
        match self {
            Self::Rust(handler, _) => handler.borrow(py).recv(py),
            _ => Err(PyAttributeError::new_err(
                "No method 'recv'. For Python receiver, use 'receiver' attribute",
            )),
        }
    }
}

fn rust_handler<H: IntoRust, T: IntoRust>(py: Python, into_handler: H) -> IntoHandlerImpl<T>
where
    H::Into: IntoHandler<'static, T::Into>,
    <H::Into as IntoHandler<'static, T::Into>>::Handler: Send + Sync,
    T::Into: IntoPython,
    RustHandler<H, T>: Receiver,
{
    let (callback, handler) = into_handler.into_rust().into_handler();
    let handler = RustHandler::<H, T> {
        handler,
        _phantom: PhantomData,
    };
    IntoHandlerImpl::Rust {
        callback,
        handler: Py::new(py, Handler(Box::new(handler))).unwrap(),
    }
}

struct RustHandler<H: IntoRust, T: IntoRust>
where
    H::Into: IntoHandler<'static, T::Into>,
{
    handler: <H::Into as IntoHandler<'static, T::Into>>::Handler,
    _phantom: PhantomData<T>,
}

impl<T: IntoRust> Receiver for RustHandler<DefaultHandler, T>
where
    T::Into: IntoPython,
{
    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        let res = py.allow_threads(|| self.handler.try_recv());
        Ok(res.ok().into_pyobject(py))
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        allow_threads(py, || self.handler.recv().to_pyres()).map_into_py(py)
    }
}

impl<T: IntoRust> Receiver for RustHandler<FifoChannel, T>
where
    T::Into: IntoPython,
{
    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        let res = py.allow_threads(|| self.handler.try_recv());
        Ok(res.ok().into_pyobject(py))
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        allow_threads(py, || self.handler.recv().to_pyres()).map_into_py(py)
    }
}

impl<T: IntoRust> Receiver for RustHandler<RingChannel, T>
where
    T::Into: IntoPython,
{
    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        allow_threads(py, || self.handler.try_recv().to_pyres()).map_into_py(py)
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        allow_threads(py, || self.handler.recv().to_pyres()).map_into_py(py)
    }
}

pub(crate) fn into_handler<T: IntoRust>(obj: &Bound<PyAny>) -> PyResult<Option<IntoHandlerImpl<T>>>
where
    T::Into: IntoPython,
{
    if obj.is_none() {
        return Ok(None);
    }
    if let Ok(handler) = obj.extract::<DefaultHandler>() {
        return Ok(Some(rust_handler(obj.py(), handler)));
    }
    if let Ok(handler) = obj.extract::<FifoChannel>() {
        return Ok(Some(rust_handler(obj.py(), handler)));
    }
    if let Ok(handler) = obj.extract::<RingChannel>() {
        return Ok(Some(rust_handler(obj.py(), handler)));
    }
    if obj.is_callable() {
        return Ok(Some(IntoHandlerImpl::Python {
            callback: PythonCallback::new(obj)?,
            handler: obj.py().None(),
        }));
    }
    if let Ok((cb, handler)) = obj.extract::<(Bound<PyAny>, PyObject)>() {
        if cb.is_callable() {
            return Ok(Some(IntoHandlerImpl::Python {
                callback: PythonCallback::new(&cb)?,
                handler,
            }));
        }
    }
    bail!("Invalid handler {}", obj.get_type().name()?);
}

pub(crate) fn handler_or_default<T: IntoRust>(
    py: Python,
    into_handler: Option<IntoHandlerImpl<T>>,
) -> IntoHandlerImpl<T>
where
    T::Into: IntoPython,
{
    into_handler.unwrap_or_else(|| rust_handler(py, DefaultHandler))
}
