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

use pyo3::{prelude::*, sync::GILOnceCell, types::PyType};

use crate::ZError;

pub(crate) trait IntoPyErr {
    fn into_pyerr(self) -> PyErr;
}
impl<E: ToString> IntoPyErr for E {
    fn into_pyerr(self) -> PyErr {
        PyErr::new::<ZError, _>(self.to_string())
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

pub(crate) trait Named {
    const NAME: &'static str; 
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
    ($($tt:tt)*) => { $crate::utils::IntoPyErr::into_pyerr(zenoh_core::zerror!($($tt)*)) };
}
pub(crate) use zerror;

macro_rules! bail {
    ($($tt:tt)*) => {
        return Err($crate::utils::zerror!($($tt)*))
    };
}
pub(crate) use bail;

macro_rules! downcast_or_parse {
    ($ty:ty) => {
        #[allow(unused)]
        impl $ty {
            pub(crate) fn from_py(obj: &Bound<PyAny>) -> PyResult<Self> {
                if let Ok(obj) = <Self as pyo3::FromPyObject>::extract_bound(obj) {
                    return Ok(obj);
                }
                Self::new(String::extract_bound(obj)?)
            }
            pub(crate) fn from_py_opt(obj: &Bound<PyAny>) -> PyResult<Option<Self>> {
                if obj.is_none() {
                    return Ok(None);
                }
                Self::from_py(obj).map(Some)
            }
        }
    };
}
pub(crate) use downcast_or_parse;

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
        pub enum $ty {$(
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
        pub struct $ty(pub(crate) $path);

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
        
        impl $crate::utils::Named for $ty {
            const NAME: &'static str = stringify!($ty);
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
        pub struct $ty(pub(crate) Option<$path>);

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
    ($builder:expr, $($value:ident),* $(,)?) => {|| {
        let mut builder = $builder;
        $(
            if let Some(value) = $value.map($crate::utils::IntoRust::into_rust) {
                builder = builder.$value(value);
            }
        )*
        builder
    }};
}
pub(crate) use build;

macro_rules! build_with {
    ($handler:expr, $builder:expr, $($value:ident),* $(,)?) => {{
        let handler = $handler;
        #[allow(clippy::redundant_closure_call)]
        || $crate::utils::build!($builder, $($value),*)().with(handler)
    }};
}
pub(crate) use build_with;
