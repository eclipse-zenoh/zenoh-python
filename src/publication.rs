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
    types::{PyDict, PyTuple},
};
use zenoh::{payload::Payload, prelude::ValueBuilderTrait};

use crate::{
    encoding::Encoding,
    key_expr::KeyExpr,
    payload::into_payload,
    resolve::{resolve, Resolve},
    utils::{build, opt_wrapper, r#enum},
};

r#enum!(zenoh::publication::Priority: u8 {
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

r#enum!(zenoh::publication::CongestionControl: u8 {
    Drop = 0,
    Block = 1,
});

#[pymethods]
impl CongestionControl {
    #[classattr]
    const DEFAULT: Self = Self::Drop;
}

opt_wrapper!(
    zenoh::publication::Publisher<'static>,
    "Undeclared publisher"
);

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
        self.undeclare(py)?.wait(py)
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
    #[pyo3(signature = (payload, *, encoding = None))]
    fn put(
        &self,
        py: Python,
        #[pyo3(from_py_with = "into_payload")] payload: Payload,
        #[pyo3(from_py_with = "Encoding::opt")] encoding: Option<Encoding>,
    ) -> PyResult<Resolve> {
        let this = self.get_ref()?;
        resolve(py, build!(this.put(payload), encoding))
    }

    fn delete(&self, py: Python) -> PyResult<Resolve> {
        let this = self.get_ref()?;
        resolve(py, || this.delete())
    }

    fn undeclare(&mut self, py: Python) -> PyResult<Resolve> {
        let this = self.take()?;
        resolve(py, || this.undeclare())
    }
}
