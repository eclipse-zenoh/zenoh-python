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
use std::ops::Deref;

use pyo3::{
    prelude::*,
    types::{PyDict, PyIterator, PyList, PyTuple, PyType},
};

use crate::{
    config::{Config, WhatAmI, WhatAmIMatcher, ZenohId},
    handlers::{handler_or_default, into_handler, HandlerImpl, IntoHandlerImpl},
    resolve::{resolve, Resolve},
    utils::{droppable_wrapper, generic, wrapper},
};

wrapper!(zenoh::scouting::Hello);

#[pymethods]
impl Hello {
    #[getter]
    fn version(&self) -> u8 {
        self.0.version
    }

    #[getter]
    fn whatami(&self) -> WhatAmI {
        self.0.whatami.into()
    }

    #[getter]
    fn zid(&self) -> ZenohId {
        ZenohId(self.0.zid)
    }

    #[getter]
    fn locators<'py>(&self, py: Python<'py>) -> Bound<'py, PyList> {
        let locators = self.0.locators.iter().map(|loc| loc.as_str().to_object(py));
        PyList::new_bound(py, locators)
    }

    fn __eq__(&self, other: &Hello) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
}

droppable_wrapper!(zenoh::scouting::Scout<HandlerImpl<Hello>>, "Stopped scout");

#[pymethods]
impl Scout {
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
        _args: &Bound<PyTuple>,
        _kwargs: Option<&Bound<PyDict>>,
    ) -> PyResult<()> {
        self.stop()
    }

    #[getter]
    fn handler(&self, py: Python) -> PyResult<PyObject> {
        Ok(self.get_ref()?.deref().to_object(py))
    }

    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.deref().try_recv(py)
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.deref().recv(py)
    }

    fn stop(&mut self) -> PyResult<()> {
        self.take()?.stop();
        Ok(())
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        self.handler(py)?.bind(py).iter()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

#[pyfunction]
#[pyo3(signature = (handler = None, what = None, config = None))]
pub(crate) fn scout(
    py: Python,
    #[pyo3(from_py_with = "into_handler::<Hello>")] handler: Option<IntoHandlerImpl<Hello>>,
    #[pyo3(from_py_with = "WhatAmIMatcher::from_py_opt")] what: Option<WhatAmIMatcher>,
    config: Option<Config>,
) -> PyResult<Resolve<Scout>> {
    let what = what.unwrap_or_default();
    let config = config.unwrap_or_default();
    let handler = handler_or_default(py, handler);
    resolve(py, || zenoh::scout(what, config).with(handler))
}
