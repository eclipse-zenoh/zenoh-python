use std::ops::Deref;

use pyo3::{
    prelude::*,
    types::{PyDict, PyIterator, PyList, PyTuple, PyType},
};

#[doc(inline)]
pub(crate) use crate::config::WhatAmI;
use crate::{
    config::{Config, WhatAmIMatcher, ZenohId},
    handlers::{handler_or_default, into_handler, HandlerImpl, IntoHandlerImpl},
    utils::{allow_threads, generic, opt_wrapper, wrapper, PySyncResolve},
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
}

opt_wrapper!(zenoh::scouting::Scout<HandlerImpl<Hello>>, "Stopped scout");

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
}

#[pyfunction]
#[pyo3(signature = (what, config, *, handler = None))]
pub(crate) fn scout(
    py: Python,
    #[pyo3(from_py_with = "WhatAmIMatcher::new")] what: WhatAmIMatcher,
    config: Config,
    #[pyo3(from_py_with = "into_handler::<Hello>")] handler: Option<IntoHandlerImpl<Hello>>,
) -> PyResult<Scout> {
    let handler = handler_or_default(py, handler);
    allow_threads(py, || {
        let builder = zenoh::scout(what, config).with(handler);
        builder.py_res_sync()
    })
}
