use pyo3::prelude::*;

use crate::utils::{IntoPyResult, IntoPython};

pub struct Resolve<T = ()>(T);

impl<T: IntoPy<PyObject>> Resolve<T> {
    pub(crate) fn wait(self, py: Python) -> PyResult<PyObject> {
        Ok(self.0.into_py(py))
    }
}

impl<T: IntoPy<PyObject>> IntoPy<PyObject> for Resolve<T> {
    fn into_py(self, py: Python<'_>) -> PyObject {
        self.0.into_py(py)
    }
}

pub(crate) fn resolve<T: IntoPython, R: zenoh_core::Resolve<zenoh::Result<T>>>(
    py: Python,
    resolve: impl FnOnce() -> R + Send,
) -> PyResult<Resolve<T::Into>> {
    let res_sync = || resolve().res_sync();
    Ok(Resolve(
        py.allow_threads(res_sync).into_pyres()?.into_python(),
    ))
}
