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
use std::{fmt, marker::PhantomData, sync::Arc, time::Duration};

use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyCFunction, PyDict, PyType},
};
use zenoh::handlers::IntoHandler;

use crate::{
    macros::{import, py_static},
    utils::{generic, short_type_name, IntoPyErr, IntoPyResult, IntoPython, IntoRust},
    ZError,
};

type RustCallback<T> = zenoh::handlers::Callback<T>;

const CHECK_SIGNALS_INTERVAL: Duration = Duration::from_millis(100);
const DROP_CALLBACK_WARNING: &str = "Passing drop-callback using a tuple \
`(callback, drop-callback)` no longer works in 1.0;\n\
`zenoh.handlers.Callback(callback, drop_callback)` must be used instead.\n\
The tuple form is reserved for passing a handler with `(callback, handler)`.\n\
If you are already passing a handler and this warning is still incorrectly displayed, \
you can silence it with:\n\
warnings.filterwarnings(\"ignore\", message=\"Passing drop-callback\")";

fn log_error(py: Python, result: PyResult<PyObject>) {
    if let Err(err) = result {
        let kwargs = PyDict::new_bound(py);
        kwargs.set_item("exc_info", err.into_value(py)).unwrap();
        py_static!(py, PyAny, || Ok(import!(py, logging.getLogger)
            .call1(("zenoh.handlers",))?
            .getattr("error")?
            .unbind()))
        .unwrap()
        .call(("callback error",), Some(&kwargs))
        .ok();
    }
}

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
    fn type_name(&self) -> &'static str;
    fn try_recv(&self, py: Python) -> PyResult<PyObject>;
    fn recv(&self, py: Python) -> PyResult<PyObject>;
}

#[pyclass]
pub(crate) struct Handler(Box<dyn Receiver + Send + Sync>);

#[pymethods]
impl Handler {
    #[classmethod]
    fn __class_getitem__(cls: &Bound<PyType>, args: &Bound<PyAny>) -> PyObject {
        generic(cls, args)
    }

    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        self.0.try_recv(py)
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        self.0.recv(py)
    }

    fn __iter__(this: Py<Self>) -> Py<Self> {
        this
    }

    fn __next__(&self, py: Python) -> PyResult<Option<PyObject>> {
        match self.0.recv(py) {
            Ok(obj) => Ok(Some(obj)),
            Err(err) if err.is_instance_of::<ZError>(py) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn __repr__(&self) -> String {
        format!("Handler[{}]", self.0.type_name())
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub(crate) struct Callback {
    #[pyo3(get)]
    callback: PyObject,
    #[pyo3(get)]
    drop: Option<PyObject>,
    #[pyo3(get)]
    indirect: bool,
}

#[pymethods]
impl Callback {
    #[new]
    #[pyo3(signature = (callback, drop, *, indirect = true))]
    fn new(callback: PyObject, drop: Option<PyObject>, indirect: bool) -> Self {
        Self {
            callback,
            drop,
            indirect,
        }
    }

    fn __call__(&self, arg: &Bound<PyAny>) -> PyResult<PyObject> {
        self.callback.call1(arg.py(), (arg,))
    }

    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

pub(crate) struct PythonCallback(Callback);

impl PythonCallback {
    fn new(obj: &Bound<PyAny>) -> Self {
        if let Ok(cb) = Callback::extract_bound(obj) {
            return Self(cb);
        }
        Self(Callback::new(obj.clone().unbind(), None, true))
    }

    fn call<T: IntoPython>(&self, py: Python, t: T) {
        log_error(py, self.0.callback.call1(py, (t.into_pyobject(py),)));
    }
}

impl Drop for PythonCallback {
    fn drop(&mut self) {
        if let Some(drop) = &self.0.drop {
            Python::with_gil(|gil| log_error(gil, drop.call0(gil)));
        }
    }
}

pub(crate) enum HandlerImpl<T> {
    Rust(Py<Handler>, PhantomData<T>),
    Python(PyObject),
}

impl<T> fmt::Debug for HandlerImpl<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rust(..) => write!(f, "Handler[{}]", short_type_name::<T>()),
            Self::Python(obj) => write!(f, "{obj:?}"),
        }
    }
}

impl<T> IntoPy<PyObject> for HandlerImpl<T> {
    fn into_py(self, _: Python<'_>) -> PyObject {
        match self {
            Self::Rust(obj, _) => obj.into_any(),
            Self::Python(obj) => obj,
        }
    }
}

impl<T> ToPyObject for HandlerImpl<T> {
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
            Self::Python(handler) => handler.call_method0(py, "try_recv"),
        }
    }

    pub(crate) fn recv(&self, py: Python) -> PyResult<PyObject> {
        match self {
            Self::Rust(handler, _) => handler.borrow(py).recv(py),
            Self::Python(handler) => handler.call_method0(py, "recv"),
        }
    }
}

struct RustHandler<H: IntoRust, T: IntoRust>
where
    H::Into: IntoHandler<T::Into>,
{
    handler: <H::Into as IntoHandler<T::Into>>::Handler,
    _phantom: PhantomData<T>,
}

fn try_recv<T: IntoPython, E: IntoPyErr + Send>(
    py: Python,
    f: impl FnOnce() -> Result<T, E> + Send,
) -> PyResult<PyObject> {
    Ok(py.allow_threads(f).into_pyres()?.into_pyobject(py))
}

fn recv<T: IntoPython, E: IntoPyErr + Send>(
    py: Python,
    f: impl Fn() -> Result<T, E> + Sync,
    is_timeout: impl Fn(&E) -> bool,
) -> PyResult<PyObject> {
    loop {
        match py.allow_threads(&f) {
            Ok(obj) => return Ok(obj.into_pyobject(py)),
            Err(err) if is_timeout(&err) => py.check_signals()?,
            Err(err) => return Err(err.into_pyerr()),
        }
    }
}

enum DeadlineError<E> {
    Timeout,
    Error(E),
}
impl<E: fmt::Display> fmt::Display for DeadlineError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error(err) => write!(f, "{err}"),
            Self::Timeout => unreachable!(),
        }
    }
}

impl<T: IntoRust> Receiver for RustHandler<DefaultHandler, T>
where
    T::Into: IntoPython,
{
    fn type_name(&self) -> &'static str {
        short_type_name::<T>()
    }

    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        try_recv(py, || PyResult::Ok(self.handler.try_recv().ok()))
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        recv(
            py,
            || match self.handler.recv_timeout(CHECK_SIGNALS_INTERVAL) {
                Ok(Some(x)) => Ok(x),
                Ok(None) => Err(DeadlineError::Timeout),
                Err(err) => Err(DeadlineError::Error(err)),
            },
            |err| matches!(err, DeadlineError::Timeout),
        )
    }
}

impl<T: IntoRust> Receiver for RustHandler<FifoChannel, T>
where
    T::Into: IntoPython,
{
    fn type_name(&self) -> &'static str {
        short_type_name::<T>()
    }

    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        try_recv(py, || PyResult::Ok(self.handler.try_recv().ok()))
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        recv(
            py,
            || match self.handler.recv_timeout(CHECK_SIGNALS_INTERVAL) {
                Ok(Some(x)) => Ok(x),
                Ok(None) => Err(DeadlineError::Timeout),
                Err(err) => Err(DeadlineError::Error(err)),
            },
            |err| matches!(err, DeadlineError::Timeout),
        )
    }
}

impl<T: IntoRust> Receiver for RustHandler<RingChannel, T>
where
    T::Into: IntoPython,
{
    fn type_name(&self) -> &'static str {
        short_type_name::<T>()
    }

    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        try_recv(py, || self.handler.try_recv())
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        recv(
            py,
            || match self.handler.recv_timeout(CHECK_SIGNALS_INTERVAL) {
                Ok(Some(x)) => Ok(x),
                Ok(None) => Err(DeadlineError::Timeout),
                Err(err) => Err(DeadlineError::Error(err)),
            },
            |err| matches!(err, DeadlineError::Timeout),
        )
    }
}

fn rust_handler<H: IntoRust, T: IntoRust>(
    py: Python,
    into_handler: H,
) -> (RustCallback<T::Into>, HandlerImpl<T>)
where
    H::Into: IntoHandler<T::Into>,
    <H::Into as IntoHandler<T::Into>>::Handler: Send + Sync,
    T::Into: IntoPython,
    RustHandler<H, T>: Receiver,
{
    let (callback, handler) = into_handler.into_rust().into_handler();
    let rust_handler = RustHandler::<H, T> {
        handler,
        _phantom: PhantomData,
    };
    let handler = Py::new(py, Handler(Box::new(rust_handler))).unwrap();
    (callback, HandlerImpl::Rust(handler, PhantomData))
}

fn python_callback<T: IntoPython>(callback: &Bound<PyAny>) -> PyResult<RustCallback<T>> {
    let py = callback.py();
    let callback = PythonCallback::new(callback);
    Ok(if callback.0.indirect {
        let (rust_callback, receiver) = DefaultHandler.into_rust().into_handler();
        let kwargs = PyDict::new_bound(py);
        let target = PyCFunction::new_closure_bound(py, None, None, move |args, _| {
            let py = args.py();
            // No need to call `Python::check_signals` because it's not the main thread.
            while let Ok(x) = py.allow_threads(|| receiver.recv()) {
                callback.call(py, x);
            }
        })?;
        kwargs.set_item("target", target)?;
        let thread = import!(py, threading.Thread).call((), Some(&kwargs))?;
        thread.call_method0("start")?;
        rust_callback
    } else {
        RustCallback::new(Arc::new(move |t| {
            Python::with_gil(|gil| callback.call(gil, t))
        }))
    })
}

pub(crate) fn into_handler<T: IntoRust>(
    py: Python,
    obj: Option<&Bound<PyAny>>,
) -> PyResult<(impl IntoHandler<T::Into, Handler = HandlerImpl<T>>, bool)>
where
    T::Into: IntoPython,
{
    let mut background = false;
    let Some(obj) = obj else {
        return Ok((rust_handler(py, DefaultHandler), background));
    };
    let into_handler = if let Ok(handler) = obj.extract::<DefaultHandler>() {
        rust_handler(py, handler)
    } else if let Ok(handler) = obj.extract::<FifoChannel>() {
        rust_handler(py, handler)
    } else if let Ok(handler) = obj.extract::<RingChannel>() {
        rust_handler(py, handler)
    } else if obj.is_callable() {
        background = true;
        (python_callback(obj)?, HandlerImpl::Python(py.None()))
    } else if let Some((cb, handler)) = obj
        .extract::<(Bound<PyAny>, PyObject)>()
        .ok()
        .filter(|(cb, _)| cb.is_callable())
    {
        if handler.bind(py).is_callable() {
            import!(py, warnings.warn).call1((DROP_CALLBACK_WARNING,))?;
        }
        (python_callback(&cb)?, HandlerImpl::Python(handler))
    } else {
        return Err(PyValueError::new_err(format!(
            "Invalid handler type {}",
            obj.get_type().name()?
        )));
    };
    Ok((into_handler, background))
}
