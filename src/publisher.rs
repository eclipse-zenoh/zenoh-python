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
    types::{PyDict, PySet, PyTuple},
};
use zenoh::prelude::*;

use crate::{
    bytes::ZBytes,
    encoding::Encoding,
    key_expr::KeyExpr,
    macros::{build, enum_mapper, option_wrapper},
    utils::wait,
};

enum_mapper!(zenoh::publisher::Priority: u8 {
    RealTime = 1,
    InteractiveHigh = 2,
    InteractiveLow = 3,
    DataHigh = 4,
    Data = 5,
    DataLow = 6,
    Background = 7,
});

#[pymethods]
impl Priority {
    #[classattr]
    const DEFAULT: Self = Self::Data;
    #[classattr]
    const MIN: Self = Self::Background;
    #[classattr]
    const MAX: Self = Self::RealTime;
    #[classattr]
    const NUM: usize = 1 + Self::MIN as usize - Self::MAX as usize;
}

enum_mapper!(zenoh::publisher::CongestionControl: u8 {
    Drop = 0,
    Block = 1,
});

#[pymethods]
impl CongestionControl {
    #[classattr]
    const DEFAULT: Self = Self::Drop;
}

#[pyclass]
pub(crate) struct Publisher {
    pub(crate) publisher: Option<zenoh::publisher::Publisher<'static>>,
    pub(crate) session_pool: Py<PySet>,
}

option_wrapper!(
    Publisher.publisher: zenoh::publisher::Publisher<'static>,
    "Undeclared publisher"
);

#[pymethods]
impl Publisher {
    fn _drop(&mut self) {
        self.wait_drop();
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
    fn key_expr(&self) -> PyResult<KeyExpr> {
        Ok(self.get_ref()?.key_expr().clone().into())
    }

    #[setter]
    fn congestion_control(&mut self, congestion_control: CongestionControl) -> PyResult<()> {
        self.get_mut()?
            .set_congestion_control(congestion_control.into());
        Ok(())
    }

    #[setter]
    fn priority(&mut self, priority: Priority) -> PyResult<()> {
        self.get_mut()?.set_priority(priority.into());
        Ok(())
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
        let this = self.get_ref()?;
        wait(py, build!(this.delete(), attachment))
    }

    fn undeclare(this: &Bound<Self>) -> PyResult<()> {
        this.borrow()
            .session_pool
            .bind(this.py())
            .discard(this.into_py(this.py()))?;
        let publisher = this.borrow_mut().take()?;
        wait(this.py(), || publisher.undeclare())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.get_ref()?))
    }
}
