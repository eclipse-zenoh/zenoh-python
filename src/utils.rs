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
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use pyo3::{prelude::*, sync::GILOnceCell, types::PyType};
use zenoh_core::{AsyncResolve, SyncResolve};

use crate::ZError;

pub(crate) trait ToPyErr {
    fn to_pyerr(self) -> PyErr;
}
impl<E: ToString> ToPyErr for E {
    fn to_pyerr(self) -> PyErr {
        PyErr::new::<ZError, _>(self.to_string())
    }
}
pub(crate) trait ToPyResult<T> {
    fn to_pyres(self) -> Result<T, PyErr>;
}
impl<T, E: ToPyErr> ToPyResult<T> for Result<T, E> {
    fn to_pyres(self) -> Result<T, PyErr> {
        self.map_err(ToPyErr::to_pyerr)
    }
}
pub(crate) trait PySyncResolve {
    type To;
    fn py_res_sync(self) -> Self::To;
}
impl<R: SyncResolve<To = Result<T, E>>, T, E: ToPyErr> PySyncResolve for R {
    type To = PyResult<T>;

    fn py_res_sync(self) -> Self::To {
        self.res_sync().to_pyres()
    }
}
pub(crate) struct PyFuture<F>(F);
impl<F: Future<Output = Result<T, E>>, T, E: ToPyErr> Future for PyFuture<F> {
    type Output = PyResult<T>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // SAFETY: field is pinned
        unsafe { self.map_unchecked_mut(|fut| &mut fut.0) }
            .poll(cx)
            .map(ToPyResult::to_pyres)
    }
}
pub(crate) trait PyAsyncResolve {
    type Future;
    fn py_res_async(self) -> Self::Future;
}
impl<R: AsyncResolve<To = Result<T, E>>, T, E: ToPyErr> PyAsyncResolve for R {
    type Future = PyFuture<R::Future>;

    fn py_res_async(self) -> Self::Future {
        PyFuture(self.res_async())
    }
}

pub(crate) trait IntoRust: Send + Sync + 'static {
    type Into: Send + 'static;
    fn into_rust(self) -> Self::Into;
}

pub(crate) trait IntoPython: Sized + Send + Sync + 'static {
    type Into: IntoPy<PyObject> + Send + 'static;
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

pub(crate) trait MapIntoPy<T> {
    fn map_into_py(self, py: Python) -> PyResult<T>;
}

impl<T: IntoPy<U>, U> MapIntoPy<U> for PyResult<T> {
    fn map_into_py(self, py: Python) -> PyResult<U> {
        Ok(self?.into_py(py))
    }
}

pub(crate) fn allow_threads<T: IntoPython + Send>(
    py: Python,
    f: impl FnOnce() -> PyResult<T> + Send,
) -> PyResult<T::Into> {
    Ok(py.allow_threads(f)?.into_python())
}

pub(crate) struct TryProcessIter<'a, I, E> {
    iter: I,
    error: &'a mut Option<E>,
}

impl<I: Iterator<Item = Result<T, E>>, T, E> Iterator for TryProcessIter<'_, I, E> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(x)) => Some(x),
            Some(Err(err)) => {
                *self.error = Some(err);
                None
            }
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.error.is_some() {
            (0, Some(0))
        } else {
            self.iter.size_hint()
        }
    }
}

pub(crate) fn try_process<I, T, E, R>(
    iter: I,
    process: impl FnOnce(TryProcessIter<'_, I::IntoIter, E>) -> R,
) -> Result<R, E>
where
    I: IntoIterator<Item = Result<T, E>>,
{
    let mut error = None;
    let iter = TryProcessIter {
        iter: iter.into_iter(),
        error: &mut error,
    };
    let res = process(iter);
    if let Some(err) = error {
        return Err(err);
    }
    Ok(res)
}

pub(crate) fn generic(cls: &Bound<PyType>, args: &Bound<PyAny>) -> PyObject {
    let py = cls.py();
    static GENERIC_ALIAS: GILOnceCell<PyObject> = GILOnceCell::new();
    GENERIC_ALIAS
        .get_or_try_init(py, || {
            PyResult::Ok(py.import_bound("types")?.getattr("GenericAlias")?.unbind())
        })
        .unwrap()
        .call1(py, (cls, args))
        .unwrap()
}

macro_rules! into_rust {
    ($($ty:ty),* $(,)?) => {$(
        impl $crate::utils::IntoRust for $ty {
            type Into = $ty;
            fn into_rust(self) -> Self::Into {
                self
            }
        }
    )*};
}
pub(crate) use into_rust;

into_rust!(bool, Duration);

macro_rules! zerror {
    ($($tt:tt)*) => { $crate::utils::ToPyErr::to_pyerr(zenoh_core::zerror!($($tt)*)) };
}
pub(crate) use zerror;

macro_rules! bail {
    ($($tt:tt)*) => {
        return Err($crate::utils::zerror!($($tt)*))
    };
}
pub(crate) use bail;

macro_rules! try_downcast {
    ($obj:expr) => {
        if let Ok(obj) = <Self as pyo3::FromPyObject>::extract_bound($obj) {
            return Ok(obj);
        }
    };
}
pub(crate) use try_downcast;

macro_rules! try_downcast_or_parse {
    (from<$ty:ty> $obj:expr) => {{
        $crate::utils::try_downcast!($obj);
        Ok(Self::from($crate::utils::ToPyResult::to_pyres(
            String::extract_bound($obj)?.parse::<$ty>(),
        )?))
    }};
    ($obj:expr) => {{
        $crate::utils::try_downcast!($obj);
        Ok(Self($crate::utils::ToPyResult::to_pyres(
            String::extract_bound($obj)?.parse(),
        )?))
    }};
}
pub(crate) use try_downcast_or_parse;

macro_rules! r#enum {
    ($($path:ident)::*: $repr:ty { $($variant:ident $(= $discriminator:literal)?),* $(,)? }) => {
        $crate::utils::r#enum!(@ $($path)::*, $($path)::*: $repr { $($variant $(= $discriminator)?,)* });
    };
    (@ $ty:ident::$($tt:ident)::*, $path:path: $repr:ty { $($variant:ident $(= $discriminator:literal)?,)* }) => {
           $crate::utils::r#enum!(@ $($tt)::*, $path: $repr { $($variant $(= $discriminator)?,)* });
    };
    (@ $ty:ident, $path:path: $repr:ty { $($variant:ident $(= $discriminator:literal)?,)* }) => {paste::paste!{
        #[pyo3::pyclass]
        #[repr($repr)]
        #[derive(Copy, Clone)]
        pub(crate) enum $ty {$(
            #[pyo3(name = $variant:snake:upper)]
            $variant $(= $discriminator)?,
        )*}

        impl $ty {
            #[allow(unused)]
            fn enum_to_str(&self) -> &'static str {
                match self {$(
                    Self::$variant => stringify!([<$variant:snake:upper>]),
                )*}
            }
        }

        impl From<$ty> for $path {
            fn from(value: $ty) -> Self {
                match value {$(
                    $ty::$variant => Self::$variant,
                )*}
            }
        }

        impl From<$path> for $ty {
            fn from(value: $path) -> Self {
                match value {$(
                    $path::$variant => Self::$variant,
                )*}
            }
        }

        impl $crate::utils::IntoRust for $ty {
            type Into = $path;
            fn into_rust(self) -> Self::Into { self.into() }
        }

        impl $crate::utils::IntoPython for $path {
            type Into = $ty;
            fn into_python(self) -> Self::Into { self.into() }
        }
    }};
}
pub(crate) use r#enum;

macro_rules! wrapper {
    ($($path:ident)::* $(<$($args:tt),*>)? $(:$($derive:ty),*)?) => {
        $crate::utils::wrapper!(@ $($path)::*, $($path)::* $(<$($args),*>)? $(:$($derive),*)?);
    };
    (@ $ty:ident::$($tt:ident)::*, $path:path $(:$($derive:ty),*)?) => {
        $crate::utils::wrapper!(@ $($tt)::*, $path $(:$($derive),*)?);
    };
    (@ $ty:ident, $path:path $(:$($derive:ty),*)?) => {
        #[pyo3::pyclass]
        #[derive($($($derive),*)?)]
        pub(crate) struct $ty(pub(crate) $path);

        impl From<$ty> for $path {
            fn from(value: $ty) -> Self {
                value.0
            }
        }

        impl From<$path> for $ty {
            fn from(value: $path) -> Self {
                Self(value)
            }
        }

        impl $crate::utils::IntoRust for $ty {
            type Into = $path;
            fn into_rust(self) -> Self::Into { self.into() }
        }

        impl $crate::utils::IntoPython for $path {
            type Into = $ty;
            fn into_python(self) -> Self::Into { self.into() }
        }
    };
}
pub(crate) use wrapper;

macro_rules! opt_wrapper {
    ($($path:ident)::* $(<$lf:lifetime, $arg:ty>)?, $error:literal) => {
        $crate::utils::opt_wrapper!(@ $($path)::*, $($path)::* $(<$lf, $arg>)?, $error);
    };
    ($($path:ident)::* $(<$lf:lifetime>)?, $error:literal) => {
        $crate::utils::opt_wrapper!(@ $($path)::*, $($path)::* $(<$lf>)?, $error);
    };
    ($($path:ident)::* $(<$arg:ty>)?, $error:literal) => {
        $crate::utils::opt_wrapper!(@ $($path)::*, $($path)::* $(<$arg>)?, $error);
    };
    ($ty:ident, $path:ty, $error:literal) => {
        $crate::utils::opt_wrapper!(@ $ty, $path, $error);
    };
    (@ $ty:ident::$($tt:ident)::*, $path:path, $error:literal) => {
        $crate::utils::opt_wrapper!(@ $($tt)::*, $path, $error);
    };
    (@ $ty:ident, $path:ty, $error:literal) => {
        #[pyclass]
        pub(crate) struct $ty(pub(crate) Option<$path>);

        #[allow(unused)]
        impl $ty {
            fn none() -> PyErr {
                $crate::utils::zerror!($error)
            }
            fn check<'a, 'py>(this: &'a Bound<'py, Self>) -> PyResult<&'a Bound<'py, Self>> {
                this.borrow().get_ref()?;
                Ok(this)
            }
            fn get_ref(&self) -> PyResult<&$path> {
                self.0.as_ref().ok_or_else(Self::none)
            }
            fn get_mut(&mut self) -> PyResult<&mut $path> {
                self.0.as_mut().ok_or_else(Self::none)
            }
            fn take(&mut self) -> PyResult<$path> {
                self.0.take().ok_or_else(Self::none)
            }
        }

        impl From<$path> for $ty {
            fn from(value: $path) -> Self {
                Self(Some(value))
            }
        }

        impl $crate::utils::IntoPython for $path {
            type Into = $ty;
            fn into_python(self) -> Self::Into { self.into() }
        }

        impl Drop for $ty {
            fn drop(&mut self) {
                Python::with_gil(|gil| gil.allow_threads(|| drop(self.0.take())))
            }
        }
    };
}
pub(crate) use opt_wrapper;

macro_rules! build {
    ($builder:ident, $($value:ident),* $(,)?) => {$(
        if let Some(value) = $value.map($crate::utils::IntoRust::into_rust) {
            $builder = $builder.$value(value);
        }
    )*};
}
pub(crate) use build;
