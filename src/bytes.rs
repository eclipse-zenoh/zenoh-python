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
use std::{borrow::Cow, io::Read};

use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    sync::GILOnceCell,
    types::{
        PyBool, PyByteArray, PyBytes, PyCFunction, PyDict, PyFloat, PyFrozenSet, PyInt, PyList,
        PySet, PyString, PyTuple, PyType,
    },
    PyTypeInfo,
};

use crate::{
    macros::{downcast_or_new, import, try_import, wrapper},
    utils::{IntoPyResult, MapInto},
};

#[derive(Clone, Copy)]
#[repr(u8)]
enum SupportedType {
    ZBytes,
    Bytes,
    ByteArray,
    Str,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Float,
    Float32,
    Float64,
    Bool,
    List,
    Tuple,
    Dict,
    Set,
    FrozenSet,
}

impl SupportedType {
    fn init_dict(py: Python) -> Py<PyDict> {
        let dict = PyDict::new_bound(py);
        fn add_type<T: PyTypeInfo>(py: Python, dict: &Bound<PyDict>, tp: SupportedType) {
            dict.set_item(T::type_object_bound(py), tp as u8).unwrap()
        }
        let zenoh = py.import_bound("zenoh").unwrap();
        let add_wrapper_type = |name, tp| {
            let wrapper = zenoh.getattr(name).unwrap();
            dict.set_item(wrapper, tp as u8).unwrap();
        };
        add_type::<ZBytes>(py, &dict, SupportedType::ZBytes);
        add_type::<PyBytes>(py, &dict, SupportedType::Bytes);
        add_type::<PyByteArray>(py, &dict, SupportedType::ByteArray);
        add_type::<PyString>(py, &dict, SupportedType::Str);
        add_type::<PyInt>(py, &dict, SupportedType::Int);
        add_wrapper_type("Int8", SupportedType::Int8);
        add_wrapper_type("Int16", SupportedType::Int16);
        add_wrapper_type("Int32", SupportedType::Int32);
        add_wrapper_type("Int64", SupportedType::Int64);
        add_wrapper_type("Int128", SupportedType::Int128);
        add_wrapper_type("UInt8", SupportedType::UInt8);
        add_wrapper_type("UInt16", SupportedType::UInt16);
        add_wrapper_type("UInt32", SupportedType::UInt32);
        add_wrapper_type("UInt64", SupportedType::UInt64);
        add_wrapper_type("UInt128", SupportedType::UInt128);
        add_type::<PyFloat>(py, &dict, SupportedType::Float);
        add_wrapper_type("Float32", SupportedType::Float32);
        add_wrapper_type("Float64", SupportedType::Float64);
        add_type::<PyBool>(py, &dict, SupportedType::Bool);
        add_type::<PyList>(py, &dict, SupportedType::List);
        add_type::<PyTuple>(py, &dict, SupportedType::Tuple);
        add_type::<PyDict>(py, &dict, SupportedType::Dict);
        add_type::<PySet>(py, &dict, SupportedType::Set);
        add_type::<PyFrozenSet>(py, &dict, SupportedType::FrozenSet);
        dict.unbind()
    }

    fn try_from_py(obj: &Bound<PyAny>) -> Option<Self> {
        match u8::extract_bound(obj).ok()? {
            n if n == Self::ZBytes as u8 => Some(Self::ZBytes),
            n if n == Self::Bytes as u8 => Some(Self::Bytes),
            n if n == Self::ByteArray as u8 => Some(Self::ByteArray),
            n if n == Self::Str as u8 => Some(Self::Str),
            n if n == Self::Int as u8 => Some(Self::Int),
            n if n == Self::Int8 as u8 => Some(Self::Int8),
            n if n == Self::Int16 as u8 => Some(Self::Int16),
            n if n == Self::Int32 as u8 => Some(Self::Int32),
            n if n == Self::Int64 as u8 => Some(Self::Int64),
            n if n == Self::Int128 as u8 => Some(Self::Int128),
            n if n == Self::UInt8 as u8 => Some(Self::UInt8),
            n if n == Self::UInt16 as u8 => Some(Self::UInt16),
            n if n == Self::UInt32 as u8 => Some(Self::UInt32),
            n if n == Self::UInt64 as u8 => Some(Self::UInt64),
            n if n == Self::UInt128 as u8 => Some(Self::UInt128),
            n if n == Self::Float as u8 => Some(Self::Float),
            n if n == Self::Float32 as u8 => Some(Self::Float32),
            n if n == Self::Float64 as u8 => Some(Self::Float64),
            n if n == Self::Bool as u8 => Some(Self::Bool),
            n if n == Self::List as u8 => Some(Self::List),
            n if n == Self::Tuple as u8 => Some(Self::Tuple),
            n if n == Self::Dict as u8 => Some(Self::Dict),
            n if n == Self::Set as u8 => Some(Self::Set),
            n if n == Self::FrozenSet as u8 => Some(Self::FrozenSet),
            _ => unreachable!(),
        }
    }
}

fn serializers(py: Python) -> &'static Py<PyDict> {
    static SERIALIZERS: GILOnceCell<Py<PyDict>> = GILOnceCell::new();
    SERIALIZERS.get_or_init(py, || SupportedType::init_dict(py))
}
fn deserializers(py: Python) -> &'static Py<PyDict> {
    static DESERIALIZERS: GILOnceCell<Py<PyDict>> = GILOnceCell::new();
    DESERIALIZERS.get_or_init(py, || SupportedType::init_dict(py))
}

fn get_type<'py>(func: &Bound<'py, PyAny>, name: impl ToPyObject) -> PyResult<Bound<'py, PyType>> {
    let py = func.py();
    Ok(import!(py, typing.get_type_hints)
        .call1((func,))?
        .downcast::<PyDict>()?
        .get_item(name)?
        .unwrap_or_else(|| py.None().into_bound(py))
        .downcast_into::<PyType>()?)
}

fn get_first_parameter<'py>(func: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyString>> {
    let py = func.py();
    Ok(import!(py, inspect.signature)
        .call1((func,))?
        .getattr("parameters")?
        .iter()?
        .next()
        .unwrap_or_else(|| Ok(py.None().into_bound(py)))?
        .downcast_into::<PyString>()?)
}

#[pyfunction]
#[pyo3(signature = (func = None, /, *, target = None))]
pub(crate) fn serializer(
    py: Python,
    func: Option<&Bound<PyAny>>,
    target: Option<&Bound<PyAny>>,
) -> PyResult<PyObject> {
    match (func, target) {
        (Some(func), Some(target)) => {
            serializers(py).bind(py).set_item(target, func)?;
            Ok(py.None())
        }
        (Some(func), None) => {
            match get_first_parameter(func).and_then(|param| get_type(func, param)) {
                Ok(target) => serializer(py, Some(func), Some(&target)),
                _ => Err(PyValueError::new_err(
                    "Cannot extract target from serializer signature",
                )),
            }
        }
        (None, Some(target)) => {
            let target = target.clone().unbind();
            let closure = PyCFunction::new_closure_bound(py, None, None, move |args, _| {
                let Ok(func) = args.get_item(0) else {
                    return Err(PyTypeError::new_err("expected one positional argument"));
                };
                serializer(args.py(), Some(&func), Some(target.bind(args.py())))
            })?;
            Ok(closure.into_any().unbind())
        }
        (None, None) => Err(PyTypeError::new_err("missing 'func' or 'target' parameter")),
    }
}

#[pyfunction]
#[pyo3(signature = (func = None, /, *, target = None))]
pub(crate) fn deserializer(
    py: Python,
    func: Option<&Bound<PyAny>>,
    target: Option<&Bound<PyAny>>,
) -> PyResult<PyObject> {
    match (func, target) {
        (Some(func), Some(target)) => {
            deserializers(py).bind(py).set_item(target, func)?;
            Ok(py.None())
        }
        (Some(func), None) => match get_type(func, "return") {
            Ok(target) => deserializer(py, Some(func), Some(&target)),
            _ => Err(PyValueError::new_err(
                "Cannot extract target from deserializer signature",
            )),
        },
        (None, Some(target)) => {
            let target = target.clone().unbind();
            let closure = PyCFunction::new_closure_bound(py, None, None, move |args, _| {
                let Ok(func) = args.get_item(0) else {
                    return Err(PyTypeError::new_err("expected one positional argument"));
                };
                deserializer(args.py(), Some(&func), Some(target.bind(args.py())))
            })?;
            Ok(closure.into_any().unbind())
        }
        (None, None) => Err(PyTypeError::new_err("missing 'func' or 'target' parameter")),
    }
}

wrapper!(zenoh::bytes::ZBytes: Clone, Default);
downcast_or_new!(serialize_impl: ZBytes);

impl ZBytes {
    fn serialize_impl(obj: &Bound<PyAny>) -> PyResult<Self> {
        if let Ok(obj) = Self::extract_bound(obj) {
            return Ok(obj);
        }
        let py = obj.py();
        let Ok(Some(serializer)) = serializers(py).bind(py).get_item(obj.get_type()) else {
            return Err(PyValueError::new_err(
                format!("no serializer registered for type {type}", type = obj.get_type().name()?),
            ));
        };
        let Some(tp) = SupportedType::try_from_py(&serializer) else {
            return match ZBytes::extract_bound(&serializer.call1((obj,))?) {
                Ok(b) => Ok(b),
                _ => Err(PyTypeError::new_err(format!(
                    "serializer {} didn't return ZBytes",
                    serializer.repr()?
                ))),
            };
        };
        let serialize_item = |elt| PyResult::Ok(Self::serialize_impl(&elt)?.0);
        let serialize_pair =
            |(k, v)| PyResult::Ok((Self::serialize_impl(&k)?.0, Self::serialize_impl(&v)?.0));
        Ok(Self(match tp {
            SupportedType::ZBytes => ZBytes::extract_bound(obj)?.0,
            SupportedType::Bytes | SupportedType::ByteArray => {
                <Vec<u8>>::extract_bound(obj)?.into()
            }
            SupportedType::Str => String::extract_bound(obj)?.into(),
            SupportedType::Int | SupportedType::Int64 => i64::extract_bound(obj)?.into(),
            SupportedType::Int8 => i8::extract_bound(obj)?.into(),
            SupportedType::Int16 => i16::extract_bound(obj)?.into(),
            SupportedType::Int32 => i32::extract_bound(obj)?.into(),
            SupportedType::Int128 => i128::extract_bound(obj)?.into(),
            SupportedType::UInt8 => u8::extract_bound(obj)?.into(),
            SupportedType::UInt16 => u16::extract_bound(obj)?.into(),
            SupportedType::UInt32 => u32::extract_bound(obj)?.into(),
            SupportedType::UInt64 => u64::extract_bound(obj)?.into(),
            SupportedType::UInt128 => u128::extract_bound(obj)?.into(),
            SupportedType::Float | SupportedType::Float64 => f64::extract_bound(obj)?.into(),
            SupportedType::Float32 => (f64::extract_bound(obj)? as f32).into(),
            SupportedType::Bool => bool::extract_bound(obj)?.into(),
            SupportedType::List => obj
                .downcast::<PyList>()?
                .into_iter()
                .map(serialize_item)
                .collect::<Result<_, _>>()?,
            SupportedType::Tuple => obj
                .downcast::<PyTuple>()?
                .into_iter()
                .map(serialize_item)
                .collect::<Result<_, _>>()?,
            SupportedType::Dict => obj
                .downcast::<PyDict>()?
                .into_iter()
                .map(serialize_pair)
                .collect::<Result<_, _>>()?,
            SupportedType::Set => obj
                .downcast::<PySet>()?
                .into_iter()
                .map(serialize_item)
                .collect::<Result<_, _>>()?,
            SupportedType::FrozenSet => obj
                .downcast::<PyFrozenSet>()?
                .into_iter()
                .map(serialize_item)
                .collect::<Result<_, _>>()?,
        }))
    }

    fn deserialize_impl(this: PyRef<Self>, tp: &Bound<PyAny>) -> PyResult<PyObject> {
        let py = tp.py();
        let Ok(Some(deserializer)) = deserializers(py).bind(py).get_item(tp) else {
            if try_import!(py, types.GenericAlias)
                .is_ok_and(|alias| tp.is_instance(alias).unwrap_or(false))
            {
                return this.deserialize_generic(tp);
            }
            return Err(PyValueError::new_err(format!(
                "no deserializer registered for {tp:?}"
            )));
        };
        let Some(tp) = SupportedType::try_from_py(&deserializer) else {
            return Ok(deserializer.call1((this,))?.unbind());
        };
        let into_py = |zbytes| Self(zbytes).into_py(py);
        let to_vec = || Vec::from_iter(this.0.iter().map(Result::unwrap).map(into_py));
        Ok(match tp {
            SupportedType::ZBytes => this.into_py(py),
            SupportedType::Bytes => this.__bytes__(py)?.into_py(py),
            SupportedType::ByteArray => PyByteArray::new_bound_with(py, this.0.len(), |bytes| {
                this.0.reader().read_exact(bytes).into_pyres()
            })?
            .into_py(py),
            SupportedType::Str => this.0.deserialize::<Cow<str>>().into_pyres()?.into_py(py),
            SupportedType::Int => this.0.deserialize::<i64>().into_pyres()?.into_py(py),
            SupportedType::Int8 => import!(py, zenoh.Int8)
                .call1((this.0.deserialize::<i8>().into_pyres()?,))?
                .into_py(py),
            SupportedType::Int16 => import!(py, zenoh.Int16)
                .call1((this.0.deserialize::<i16>().into_pyres()?,))?
                .into_py(py),
            SupportedType::Int32 => import!(py, zenoh.Int32)
                .call1((this.0.deserialize::<i32>().into_pyres()?,))?
                .into_py(py),
            SupportedType::Int64 => import!(py, zenoh.Int64)
                .call1((this.0.deserialize::<i64>().into_pyres()?,))?
                .into_py(py),
            SupportedType::Int128 => import!(py, zenoh.Int128)
                .call1((this.0.deserialize::<i128>().into_pyres()?,))?
                .into_py(py),
            SupportedType::UInt8 => import!(py, zenoh.UInt8)
                .call1((this.0.deserialize::<u8>().into_pyres()?,))?
                .into_py(py),
            SupportedType::UInt16 => import!(py, zenoh.UInt16)
                .call1((this.0.deserialize::<u16>().into_pyres()?,))?
                .into_py(py),
            SupportedType::UInt32 => import!(py, zenoh.UInt32)
                .call1((this.0.deserialize::<u32>().into_pyres()?,))?
                .into_py(py),
            SupportedType::UInt64 => import!(py, zenoh.UInt64)
                .call1((this.0.deserialize::<u64>().into_pyres()?,))?
                .into_py(py),
            SupportedType::UInt128 => import!(py, zenoh.UInt128)
                .call1((this.0.deserialize::<u128>().into_pyres()?,))?
                .into_py(py),
            SupportedType::Float => this.0.deserialize::<f64>().into_pyres()?.into_py(py),
            SupportedType::Float32 => import!(py, zenoh.Float32)
                .call1((this.0.deserialize::<f32>().into_pyres()?,))?
                .into_py(py),
            SupportedType::Float64 => import!(py, zenoh.Float64)
                .call1((this.0.deserialize::<f64>().into_pyres()?,))?
                .into_py(py),
            SupportedType::Bool => this.0.deserialize::<bool>().into_pyres()?.into_py(py),
            SupportedType::List => PyList::new_bound(py, to_vec()).into_py(py),
            SupportedType::Tuple => PyTuple::new_bound(py, to_vec()).into_py(py),
            SupportedType::Dict => {
                let dict = PyDict::new_bound(py);
                for kv in this.0.iter() {
                    let (k, v) = kv.into_pyres()?;
                    dict.set_item(Self(k).into_py(py), Self(v).into_py(py))?;
                }
                dict.into_py(py)
            }
            SupportedType::Set => PySet::new_bound(py, &to_vec())?.into_py(py),
            SupportedType::FrozenSet => PyFrozenSet::new_bound(py, &to_vec())?.into_py(py),
        })
    }

    fn deserialize_generic(&self, tp: &Bound<PyAny>) -> PyResult<PyObject> {
        let py = tp.py();
        let origin = import!(py, typing.get_origin).call1((tp,))?;
        let args = import!(py, typing.get_args)
            .call1((tp,))?
            .downcast_into::<PyTuple>()?;
        let deserialize = |tp| {
            move |zbytes: Result<_, _>| {
                Self::deserialize_impl(Py::new(py, Self(zbytes.unwrap())).unwrap().borrow(py), &tp)
            }
        };
        Ok(if origin.eq(PyList::type_object_bound(py))? {
            let vec: Vec<_> = Result::from_iter(self.0.iter().map(deserialize(args.get_item(0)?)))?;
            PyList::new_bound(py, vec).into_py(py)
        } else if origin.eq(PyTuple::type_object_bound(py))?
            && args.len() == 2
            && args.get_item(1).is_ok_and(|item| item.is(&py.Ellipsis()))
        {
            let vec: Vec<_> = Result::from_iter(self.0.iter().map(deserialize(args.get_item(0)?)))?;
            PyTuple::new_bound(py, vec).into_py(py)
        } else if origin.eq(PyTuple::type_object_bound(py))? {
            let mut zbytes_iter = self.0.iter();
            let mut tp_iter = args.iter();
            let vec = zbytes_iter
                .by_ref()
                .zip(tp_iter.by_ref())
                .map(|(zbytes, tp)| deserialize(tp)(zbytes))
                .collect::<Result<Vec<_>, _>>()?;
            let remaining = zbytes_iter.count();
            if remaining > 0 || tp_iter.next().is_some() {
                return Err(PyTypeError::new_err(format!(
                    "tuple length doesn't match, found {}",
                    vec.len() + remaining
                )));
            }
            PyTuple::new_bound(py, vec).into_py(py)
        } else if origin.eq(PyDict::type_object_bound(py))? {
            let deserialize_key = deserialize(args.get_item(0)?);
            let deserialize_value = deserialize(args.get_item(1)?);
            let dict = PyDict::new_bound(py);
            for kv in self.0.iter() {
                let (k, v) = kv.into_pyres()?;
                dict.set_item(deserialize_key(Ok(k))?, deserialize_value(Ok(v))?)?;
            }
            dict.into_py(py)
        } else if origin.eq(PySet::type_object_bound(py))? {
            let vec: Vec<_> = Result::from_iter(self.0.iter().map(deserialize(args.get_item(0)?)))?;
            PySet::new_bound(py, &vec)?.into_py(py)
        } else if origin.eq(PyFrozenSet::type_object_bound(py))? {
            let vec: Vec<_> = Result::from_iter(self.0.iter().map(deserialize(args.get_item(0)?)))?;
            PyFrozenSet::new_bound(py, &vec)?.into_py(py)
        } else {
            return Err(PyValueError::new_err(
                "only `list`/`tuple`/`dict`/`set`/`frozenset` are supported as generic type",
            ));
        })
    }
}

#[pymethods]
impl ZBytes {
    #[new]
    fn new(obj: Option<&Bound<PyAny>>) -> PyResult<Self> {
        let Some(obj) = obj else {
            return Ok(Self::default());
        };
        if let Ok(bytes) = obj.downcast::<PyByteArray>() {
            // SAFETY: bytes is immediately copied
            Ok(Self(unsafe { bytes.as_bytes() }.into()))
        } else if let Ok(bytes) = obj.downcast::<PyBytes>() {
            Ok(Self(bytes.as_bytes().into()))
        } else {
            Err(PyTypeError::new_err(format!(
                "expected buffer type, found '{}'",
                obj.get_type().name().unwrap()
            )))
        }
    }

    #[classmethod]
    fn serialize(_cls: &Bound<PyType>, obj: &Bound<PyAny>) -> PyResult<Self> {
        Self::serialize_impl(obj)
    }

    fn deserialize(this: PyRef<Self>, tp: &Bound<PyAny>) -> PyResult<PyObject> {
        Self::deserialize_impl(this, tp)
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn __bool__(&self) -> bool {
        !self.0.is_empty()
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        PyBytes::new_bound_with(py, self.0.len(), |bytes| {
            self.0.reader().read_exact(bytes).into_pyres()
        })
    }

    fn __eq__(&self, other: &Bound<PyAny>) -> PyResult<bool> {
        Ok(self.0 == Self::from_py(other)?.0)
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        self.__bytes__(py)?.hash()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

wrapper!(zenoh::bytes::Encoding: Clone, Default);
downcast_or_new!(Encoding => Option<String>);

#[pymethods]
impl Encoding {
    #[new]
    fn new(s: Option<String>) -> PyResult<Self> {
        Ok(s.map_into().map(Self).unwrap_or_default())
    }

    fn with_schema(&self, schema: String) -> Self {
        Self(self.0.clone().with_schema(schema))
    }

    // Cannot use `#[pyo3(from_py_with = "...")]`, see https://github.com/PyO3/pyo3/issues/4113
    fn __eq__(&self, other: &Bound<PyAny>) -> PyResult<bool> {
        Ok(self.0 == Self::from_py(other)?.0)
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        PyString::new_bound(py, &self.__str__()).hash()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    #[classattr]
    const ZENOH_BYTES: Self = Self(zenoh::bytes::Encoding::ZENOH_BYTES);
    #[classattr]
    const ZENOH_INT8: Self = Self(zenoh::bytes::Encoding::ZENOH_INT8);
    #[classattr]
    const ZENOH_INT16: Self = Self(zenoh::bytes::Encoding::ZENOH_INT16);
    #[classattr]
    const ZENOH_INT32: Self = Self(zenoh::bytes::Encoding::ZENOH_INT32);
    #[classattr]
    const ZENOH_INT64: Self = Self(zenoh::bytes::Encoding::ZENOH_INT64);
    #[classattr]
    const ZENOH_INT128: Self = Self(zenoh::bytes::Encoding::ZENOH_INT128);
    #[classattr]
    const ZENOH_UINT8: Self = Self(zenoh::bytes::Encoding::ZENOH_UINT8);
    #[classattr]
    const ZENOH_UINT16: Self = Self(zenoh::bytes::Encoding::ZENOH_UINT16);
    #[classattr]
    const ZENOH_UINT32: Self = Self(zenoh::bytes::Encoding::ZENOH_UINT32);
    #[classattr]
    const ZENOH_UINT64: Self = Self(zenoh::bytes::Encoding::ZENOH_UINT64);
    #[classattr]
    const ZENOH_UINT128: Self = Self(zenoh::bytes::Encoding::ZENOH_UINT128);
    #[classattr]
    const ZENOH_FLOAT32: Self = Self(zenoh::bytes::Encoding::ZENOH_FLOAT32);
    #[classattr]
    const ZENOH_FLOAT64: Self = Self(zenoh::bytes::Encoding::ZENOH_FLOAT64);
    #[classattr]
    const ZENOH_BOOL: Self = Self(zenoh::bytes::Encoding::ZENOH_BOOL);
    #[classattr]
    const ZENOH_STRING: Self = Self(zenoh::bytes::Encoding::ZENOH_STRING);
    #[classattr]
    const ZENOH_ERROR: Self = Self(zenoh::bytes::Encoding::ZENOH_ERROR);
    #[classattr]
    const APPLICATION_OCTET_STREAM: Self = Self(zenoh::bytes::Encoding::APPLICATION_OCTET_STREAM);
    #[classattr]
    const TEXT_PLAIN: Self = Self(zenoh::bytes::Encoding::TEXT_PLAIN);
    #[classattr]
    const APPLICATION_JSON: Self = Self(zenoh::bytes::Encoding::APPLICATION_JSON);
    #[classattr]
    const TEXT_JSON: Self = Self(zenoh::bytes::Encoding::TEXT_JSON);
    #[classattr]
    const APPLICATION_CDR: Self = Self(zenoh::bytes::Encoding::APPLICATION_CDR);
    #[classattr]
    const APPLICATION_CBOR: Self = Self(zenoh::bytes::Encoding::APPLICATION_CBOR);
    #[classattr]
    const APPLICATION_YAML: Self = Self(zenoh::bytes::Encoding::APPLICATION_YAML);
    #[classattr]
    const TEXT_YAML: Self = Self(zenoh::bytes::Encoding::TEXT_YAML);
    #[classattr]
    const TEXT_JSON5: Self = Self(zenoh::bytes::Encoding::TEXT_JSON5);
    #[classattr]
    const APPLICATION_PYTHON_SERIALIZED_OBJECT: Self =
        Self(zenoh::bytes::Encoding::APPLICATION_PYTHON_SERIALIZED_OBJECT);
    #[classattr]
    const APPLICATION_PROTOBUF: Self = Self(zenoh::bytes::Encoding::APPLICATION_PROTOBUF);
    #[classattr]
    const APPLICATION_JAVA_SERIALIZED_OBJECT: Self =
        Self(zenoh::bytes::Encoding::APPLICATION_JAVA_SERIALIZED_OBJECT);
    #[classattr]
    const APPLICATION_OPENMETRICS_TEXT: Self =
        Self(zenoh::bytes::Encoding::APPLICATION_OPENMETRICS_TEXT);
    #[classattr]
    const IMAGE_PNG: Self = Self(zenoh::bytes::Encoding::IMAGE_PNG);
    #[classattr]
    const IMAGE_JPEG: Self = Self(zenoh::bytes::Encoding::IMAGE_JPEG);
    #[classattr]
    const IMAGE_GIF: Self = Self(zenoh::bytes::Encoding::IMAGE_GIF);
    #[classattr]
    const IMAGE_BMP: Self = Self(zenoh::bytes::Encoding::IMAGE_BMP);
    #[classattr]
    const IMAGE_WEBP: Self = Self(zenoh::bytes::Encoding::IMAGE_WEBP);
    #[classattr]
    const APPLICATION_XML: Self = Self(zenoh::bytes::Encoding::APPLICATION_XML);
    #[classattr]
    const APPLICATION_X_WWW_FORM_URLENCODED: Self =
        Self(zenoh::bytes::Encoding::APPLICATION_X_WWW_FORM_URLENCODED);
    #[classattr]
    const TEXT_HTML: Self = Self(zenoh::bytes::Encoding::TEXT_HTML);
    #[classattr]
    const TEXT_XML: Self = Self(zenoh::bytes::Encoding::TEXT_XML);
    #[classattr]
    const TEXT_CSS: Self = Self(zenoh::bytes::Encoding::TEXT_CSS);
    #[classattr]
    const TEXT_JAVASCRIPT: Self = Self(zenoh::bytes::Encoding::TEXT_JAVASCRIPT);
    #[classattr]
    const TEXT_MARKDOWN: Self = Self(zenoh::bytes::Encoding::TEXT_MARKDOWN);
    #[classattr]
    const TEXT_CSV: Self = Self(zenoh::bytes::Encoding::TEXT_CSV);
    #[classattr]
    const APPLICATION_SQL: Self = Self(zenoh::bytes::Encoding::APPLICATION_SQL);
    #[classattr]
    const APPLICATION_COAP_PAYLOAD: Self = Self(zenoh::bytes::Encoding::APPLICATION_COAP_PAYLOAD);
    #[classattr]
    const APPLICATION_JSON_PATCH_JSON: Self =
        Self(zenoh::bytes::Encoding::APPLICATION_JSON_PATCH_JSON);
    #[classattr]
    const APPLICATION_JSON_SEQ: Self = Self(zenoh::bytes::Encoding::APPLICATION_JSON_SEQ);
    #[classattr]
    const APPLICATION_JSONPATH: Self = Self(zenoh::bytes::Encoding::APPLICATION_JSONPATH);
    #[classattr]
    const APPLICATION_JWT: Self = Self(zenoh::bytes::Encoding::APPLICATION_JWT);
    #[classattr]
    const APPLICATION_MP4: Self = Self(zenoh::bytes::Encoding::APPLICATION_MP4);
    #[classattr]
    const APPLICATION_SOAP_XML: Self = Self(zenoh::bytes::Encoding::APPLICATION_SOAP_XML);
    #[classattr]
    const APPLICATION_YANG: Self = Self(zenoh::bytes::Encoding::APPLICATION_YANG);
    #[classattr]
    const AUDIO_AAC: Self = Self(zenoh::bytes::Encoding::AUDIO_AAC);
    #[classattr]
    const AUDIO_FLAC: Self = Self(zenoh::bytes::Encoding::AUDIO_FLAC);
    #[classattr]
    const AUDIO_MP4: Self = Self(zenoh::bytes::Encoding::AUDIO_MP4);
    #[classattr]
    const AUDIO_OGG: Self = Self(zenoh::bytes::Encoding::AUDIO_OGG);
    #[classattr]
    const AUDIO_VORBIS: Self = Self(zenoh::bytes::Encoding::AUDIO_VORBIS);
    #[classattr]
    const VIDEO_H261: Self = Self(zenoh::bytes::Encoding::VIDEO_H261);
    #[classattr]
    const VIDEO_H263: Self = Self(zenoh::bytes::Encoding::VIDEO_H263);
    #[classattr]
    const VIDEO_H264: Self = Self(zenoh::bytes::Encoding::VIDEO_H264);
    #[classattr]
    const VIDEO_H265: Self = Self(zenoh::bytes::Encoding::VIDEO_H265);
    #[classattr]
    const VIDEO_H266: Self = Self(zenoh::bytes::Encoding::VIDEO_H266);
    #[classattr]
    const VIDEO_MP4: Self = Self(zenoh::bytes::Encoding::VIDEO_MP4);
    #[classattr]
    const VIDEO_OGG: Self = Self(zenoh::bytes::Encoding::VIDEO_OGG);
    #[classattr]
    const VIDEO_RAW: Self = Self(zenoh::bytes::Encoding::VIDEO_RAW);
    #[classattr]
    const VIDEO_VP8: Self = Self(zenoh::bytes::Encoding::VIDEO_VP8);
    #[classattr]
    const VIDEO_VP9: Self = Self(zenoh::bytes::Encoding::VIDEO_VP9);
}
