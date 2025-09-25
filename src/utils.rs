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
use std::time::Duration;

use pyo3::{exceptions::PyValueError, prelude::*, types::PyType, IntoPyObjectExt};

use crate::{
    macros::{import, into_rust},
    ZError,
};

pub(crate) trait IntoResult<T, E> {
    fn into_result(self) -> Result<T, E>;
}

impl<T, E> IntoResult<T, E> for T {
    fn into_result(self) -> Result<T, E> {
        Ok(self)
    }
}

impl<T, E> IntoResult<T, E> for Result<T, E> {
    fn into_result(self) -> Result<T, E> {
        self
    }
}

pub(crate) trait IntoPyErr {
    fn into_pyerr(self) -> PyErr;
}
impl<E: ToString> IntoPyErr for E {
    fn into_pyerr(self) -> PyErr {
        ZError::new_err(self.to_string())
    }
}
pub(crate) trait IntoPyResult<T> {
    fn into_pyres(self) -> Result<T, PyErr>;
}
impl<T, E: IntoPyErr> IntoPyResult<T> for Result<T, E> {
    fn into_pyres(self) -> Result<T, PyErr> {
        self.map_err(IntoPyErr::into_pyerr)
    }
}

pub(crate) trait IntoRust: 'static {
    type Into;
    fn into_rust(self) -> Self::Into;
}

into_rust!(bool, usize, f64, Duration);

pub(crate) trait IntoPython: Sized + Send + Sync + 'static {
    type Into: for<'py> IntoPyObject<'py>;
    fn into_python(self) -> Self::Into;
    fn into_pyobject(self, py: Python) -> PyObject {
        self.into_python().into_py_any(py).unwrap()
    }
}

impl IntoPython for () {
    type Into = ();
    fn into_python(self) -> Self::Into {
        self
    }
}

impl<T> IntoPython for Option<T>
where
    T: IntoPython,
{
    type Into = Option<T::Into>;

    fn into_python(self) -> Self::Into {
        Some(self?.into_python())
    }
}

pub(crate) trait MapInto<T> {
    fn map_into(self) -> T;
}

impl<T: Into<U>, U> MapInto<Option<U>> for Option<T> {
    fn map_into(self) -> Option<U> {
        self.map(Into::into)
    }
}

impl<T: Into<U>, U, E> MapInto<Result<U, E>> for Result<T, E> {
    fn map_into(self) -> Result<U, E> {
        self.map(Into::into)
    }
}

pub(crate) fn generic(cls: &Bound<PyType>, args: &Bound<PyAny>) -> PyObject {
    import!(cls.py(), types.GenericAlias)
        .call1((cls, args))
        .unwrap()
        .unbind()
}

pub(crate) fn short_type_name<T: ?Sized>() -> &'static str {
    let name = std::any::type_name::<T>();
    name.rsplit_once("::").map_or(name, |(_, name)| name)
}

pub(crate) fn wait<T: Send, E: IntoPyErr + Send>(
    py: Python,
    resolve: impl zenoh::Wait<To = Result<T, E>> + Send,
) -> PyResult<T> {
    py.allow_threads(|| resolve.wait()).into_pyres()
}

pub(crate) fn duration(obj: &Bound<PyAny>) -> PyResult<Option<Duration>> {
    if obj.is_none() {
        return Ok(None);
    }
    Duration::try_from_secs_f64(obj.extract::<f64>()?)
        .map(Some)
        .map_err(|_| PyValueError::new_err("negative timeout"))
}

#[cfg(Py_3_11)]
pub(crate) unsafe fn init_buffer(
    view: *mut pyo3::ffi::Py_buffer,
    flags: std::ffi::c_int,
    buf: *mut u8,
    len: usize,
    readonly: bool,
    owner: *mut pyo3::ffi::PyObject,
) {
    unsafe {
        (*view).obj = owner;
        (*view).buf = buf as *mut std::ffi::c_void;
        (*view).len = len as isize;
        (*view).readonly = readonly as std::ffi::c_int;
        (*view).itemsize = 1;
        (*view).format = if (flags & pyo3::ffi::PyBUF_FORMAT) != 0 {
            static B: &std::ffi::CStr = pyo3::ffi::c_str!("B");
            B.as_ptr().cast_mut()
        } else {
            std::ptr::null_mut()
        };
        (*view).ndim = 1;
        (*view).shape = if (flags & pyo3::ffi::PyBUF_ND) != 0 {
            &mut (*view).len
        } else {
            std::ptr::null_mut()
        };
        (*view).strides = if (flags & pyo3::ffi::PyBUF_STRIDES) != 0 {
            &mut (*view).itemsize
        } else {
            std::ptr::null_mut()
        };
        (*view).suboffsets = std::ptr::null_mut();
        (*view).internal = std::ptr::null_mut();
    }
}
