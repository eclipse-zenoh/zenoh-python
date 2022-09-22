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
use std::{convert::TryFrom, sync::Arc};

use pyo3::{prelude::*, types::PyTuple};
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
