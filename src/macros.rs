macro_rules! try_import {
    ($py:expr, $module:ident.$attr:ident) => {{
        static IMPORTED: GILOnceCell<PyObject> = GILOnceCell::new();
        let import = || {
            PyResult::Ok(
                $py.import_bound(stringify!($module))?
                    .getattr(stringify!($attr))?
                    .unbind(),
            )
        };
        IMPORTED
            .get_or_try_init($py, import)
            .map(|obj| obj.bind($py))
    }};
}
pub(crate) use try_import;

macro_rules! import {
    ($py:expr, $module:ident.$attr:ident) => {{
        $crate::macros::try_import!($py, $module.$attr).unwrap()
    }};
}
pub(crate) use import;

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

macro_rules! zerror {
    ($($tt:tt)*) => { $crate::utils::IntoPyErr::into_pyerr(zenoh_core::zerror!($($tt)*)) };
}
pub(crate) use zerror;

macro_rules! bail {
    ($($tt:tt)*) => {
        return Err($crate::macros::zerror!($($tt)*))
    };
}
pub(crate) use bail;

macro_rules! downcast_or_new {
    ($ty:ty $(=> $new:ty)? $(, $other:expr)?) => {
        #[allow(unused)]
        impl $ty {
            pub(crate) fn from_py(obj: &Bound<PyAny>) -> PyResult<Self> {
                if let Ok(obj) = <Self as pyo3::FromPyObject>::extract_bound(obj) {
                    return Ok(obj);
                }
                Self::new(PyResult::Ok(obj)$(.and_then(<$new>::extract_bound))??.into(), $($other)?)
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
pub(crate) use downcast_or_new;

macro_rules! enum_mapper {
    ($($path:ident)::*: $repr:ty { $($variant:ident $(= $discriminator:literal)?),* $(,)? }) => {
        $crate::macros::enum_mapper!(@ $($path)::*, $($path)::*: $repr { $($variant $(= $discriminator)?,)* });
    };
    (@ $ty:ident::$($tt:ident)::*, $path:path: $repr:ty { $($variant:ident $(= $discriminator:literal)?,)* }) => {
           $crate::macros::enum_mapper!(@ $($tt)::*, $path: $repr { $($variant $(= $discriminator)?,)* });
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
pub(crate) use enum_mapper;

macro_rules! wrapper {
    ($($path:ident)::* $(<$($args:tt),*>)? $(:$($derive:ty),*)?) => {
        $crate::macros::wrapper!(@ $($path)::*, $($path)::* $(<$($args),*>)? $(:$($derive),*)?);
    };
    (@ $ty:ident::$($tt:ident)::*, $path:path $(:$($derive:ty),*)?) => {
        $crate::macros::wrapper!(@ $($tt)::*, $path $(:$($derive),*)?);
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
    };
}
pub(crate) use wrapper;

macro_rules! droppable_wrapper {
    ($($path:ident)::* $(<$lf:lifetime, $arg:ty>)?, $error:literal) => {
        $crate::macros::droppable_wrapper!(@ $($path)::*, $($path)::* $(<$lf, $arg>)?, $error);
    };
    ($($path:ident)::* $(<$lf:lifetime>)?, $error:literal) => {
        $crate::macros::droppable_wrapper!(@ $($path)::*, $($path)::* $(<$lf>)?, $error);
    };
    ($($path:ident)::* $(<$arg:ty>)?, $error:literal) => {
        $crate::macros::droppable_wrapper!(@ $($path)::*, $($path)::* $(<$arg>)?, $error);
    };
    ($ty:ident, $path:ty, $error:literal) => {
        $crate::macros::droppable_wrapper!(@ $ty, $path, $error);
    };
    (@ $ty:ident::$($tt:ident)::*, $path:path, $error:literal) => {
        $crate::macros::droppable_wrapper!(@ $($tt)::*, $path, $error);
    };
    (@ $ty:ident, $path:ty, $error:literal) => {
        #[pyclass]
        pub struct $ty(pub(crate) Option<$path>);

        #[allow(unused)]
        impl $ty {
            fn none() -> PyErr {
                $crate::macros::zerror!($error)
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
pub(crate) use droppable_wrapper;

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
        || $crate::macros::build!($builder, $($value),*)().with(handler)
    }};
}
pub(crate) use build_with;
