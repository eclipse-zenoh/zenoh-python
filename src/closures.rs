//
// Copyright (c) 2017, 2022 ZettaScale Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh team, <zenoh@zettascale.tech>
//
use std::{
    convert::TryFrom,
    sync::{Arc, Mutex},
};

use pyo3::{
    prelude::*,
    types::{PyList, PyTuple},
};
use zenoh::prelude::IntoCallbackReceiverPair;

trait CallbackUnwrap {
    type Output;
    fn cb_unwrap(self) -> Self::Output;
}
impl<T> CallbackUnwrap for PyResult<T> {
    type Output = T;
    fn cb_unwrap(self) -> Self::Output {
        match self {
            Ok(o) => o,
            Err(e) => Python::with_gil(|py| {
                if let Some(trace) = e.traceback(py).and_then(|trace| trace.format().ok()) {
                    panic!("Exception thrown in callback: {}.\n{}", e, trace)
                } else {
                    panic!("Exception thrown in callback: {}.", e,)
                }
            }),
        }
    }
}

pub(crate) struct PyClosure<I> {
    pub(crate) pycall: Py<PyAny>,
    pub(crate) drop: Option<Py<PyAny>>,
    _marker: std::marker::PhantomData<I>,
}
impl<I> TryFrom<&PyAny> for PyClosure<I> {
    type Error = PyErr;
    fn try_from(value: &PyAny) -> Result<Self, Self::Error> {
        Python::with_gil(|py| {
            let pycall = match value.getattr("call") {
                Ok(value) => value.into_py(py),
                Err(e) => return Err(e),
            };
            let drop = match value.getattr("drop") {
                Ok(value) => {
                    if value.is_none() {
                        None
                    } else {
                        Some(value.into_py(py))
                    }
                }
                Err(e) => return Err(e),
            };
            Ok(PyClosure {
                pycall,
                drop,
                _marker: std::marker::PhantomData,
            })
        })
    }
}
impl<I: IntoPy<Py<PyTuple>>> PyClosure<I> {
    pub fn call(&self, args: I) -> PyResult<PyObject> {
        Python::with_gil(|py| self.pycall.call1(py, args))
    }
}
impl<I> Drop for PyClosure<I> {
    fn drop(&mut self) {
        if let Some(drop) = self.drop.take() {
            Python::with_gil(|py| drop.call0(py)).unwrap();
        }
    }
}
impl<T, I> IntoCallbackReceiverPair<'static, T> for PyClosure<(I,)>
where
    T: Into<I>,
    I: Send + Sync + 'static,
    (I,): IntoPy<Py<PyTuple>>,
{
    type Receiver = ();

    fn into_cb_receiver_pair(self) -> (zenoh::handlers::Callback<'static, T>, Self::Receiver) {
        (
            Arc::new(move |reply| {
                self.call((reply.into(),)).cb_unwrap();
            }),
            (),
        )
    }
}

#[pyclass(subclass)]
pub struct _Queue {
    send: Mutex<Option<flume::Sender<PyObject>>>,
    recv: flume::Receiver<PyObject>,
}
#[pymethods]
impl _Queue {
    #[new]
    pub fn pynew(bound: Option<usize>) -> Self {
        let (send, recv) = match bound {
            None => flume::unbounded(),
            Some(bound) => flume::bounded(bound),
        };
        Self {
            send: Mutex::new(Some(send)),
            recv,
        }
    }
    pub fn close(&self) {
        *self.send.lock().unwrap() = None;
    }
    pub fn put(&self, value: PyObject, py: Python<'_>) -> PyResult<()> {
        match self.send.lock().unwrap().as_ref() {
            None => Err(pyo3::exceptions::PyBrokenPipeError::new_err(
                "Attempted to put on closed Queue",
            )),
            Some(send) => Python::allow_threads(py, || {
                send.send(value).unwrap();
                Ok(())
            }),
        }
    }
    pub fn get(&self, timeout: Option<f32>, py: Python<'_>) -> PyResult<PyObject> {
        Python::allow_threads(py, || match timeout {
            None => match self.recv.recv() {
                Ok(value) => Ok(value),
                Err(_) => Err(pyo3::exceptions::PyStopIteration::new_err(())),
            },
            Some(secs) => match self
                .recv
                .recv_timeout(std::time::Duration::from_secs_f32(secs))
            {
                Ok(value) => Ok(value),
                Err(flume::RecvTimeoutError::Timeout) => {
                    Err(pyo3::exceptions::PyTimeoutError::new_err(()))
                }
                Err(flume::RecvTimeoutError::Disconnected) => {
                    Err(pyo3::exceptions::PyStopIteration::new_err(()))
                }
            },
        })
    }
    pub fn get_remaining(&self, timeout: Option<f32>, py: Python<'_>) -> PyResult<Py<PyList>> {
        Python::allow_threads(py, || {
            let vec = match timeout {
                None => self.recv.iter().collect::<Vec<_>>(),
                Some(secs) => {
                    let deadline =
                        std::time::Instant::now() + std::time::Duration::from_secs_f32(secs);
                    let mut vec = Vec::new();
                    loop {
                        match self.recv.recv_deadline(deadline) {
                            Ok(v) => vec.push(v),
                            Err(flume::RecvTimeoutError::Disconnected) => break,
                            Err(flume::RecvTimeoutError::Timeout) => {
                                let list: Py<PyList> =
                                    Python::with_gil(|py| PyList::new(py, vec).into_py(py));
                                return Err(pyo3::exceptions::PyTimeoutError::new_err((list,)));
                            }
                        }
                    }
                    vec
                }
            };
            Ok(Python::with_gil(|py| PyList::new(py, vec).into_py(py)))
        })
    }
    pub fn is_closed(&self) -> bool {
        self.send.lock().unwrap().is_none()
    }
}
