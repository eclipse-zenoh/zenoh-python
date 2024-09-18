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

use pyo3::{prelude::*, types::PyType};

use crate::{
    macros::{import, into_rust},
    ZError,
};

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

pub(crate) trait IntoRust: Send + Sync + 'static {
    type Into;
    fn into_rust(self) -> Self::Into;
}

into_rust!(bool, Duration);

pub(crate) trait IntoPython: Sized + Send + 'static {
    type Into: IntoPy<PyObject>;
    fn into_python(self) -> Self::Into;
    fn into_pyobject(self, py: Python) -> PyObject {
        self.into_python().into_py(py)
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

pub(crate) fn wait<T: Send>(
    py: Python,
    resolve: impl zenoh::Resolve<zenoh::Result<T>>,
) -> PyResult<T> {
    py.allow_threads(|| resolve.wait()).into_pyres()
}
