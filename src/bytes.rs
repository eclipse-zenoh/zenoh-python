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
use std::borrow::Cow;

use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    sync::GILOnceCell,
    types::{PyBool, PyBytes, PyDict, PyFloat, PyInt, PyList, PyString, PyType},
    PyTypeInfo,
};
use zenoh_buffers::{buffer::SplitBuffer, ZBuf};

use crate::utils::{bail, downcast_or_new, wrapper, IntoPyResult};

macro_rules! import {
    ($module:ident, $attr:ident) => {
        fn $attr(py: Python) -> &'static PyObject {
            static MODULE: GILOnceCell<PyObject> = GILOnceCell::new();
            let import = || {
                PyResult::Ok(
                    py.import_bound(stringify!($module))?
                        .getattr(stringify!($attr))?
                        .unbind(),
                )
            };
            MODULE.get_or_try_init(py, import).unwrap()
        }
    };
}

import!(json, dumps);
import!(json, loads);
import!(inspect, signature);
import!(typing, get_type_hints);

fn serializers(py: Python) -> &'static Py<PyDict> {
    static SERIALIZERS: GILOnceCell<Py<PyDict>> = GILOnceCell::new();
    SERIALIZERS.get_or_init(py, || PyDict::new_bound(py).unbind())
}
fn deserializers(py: Python) -> &'static Py<PyDict> {
    static DESERIALIZERS: GILOnceCell<Py<PyDict>> = GILOnceCell::new();
    DESERIALIZERS.get_or_init(py, || PyDict::new_bound(py).unbind())
}

fn get_type<'py>(func: &Bound<'py, PyAny>, name: impl ToPyObject) -> PyResult<Bound<'py, PyType>> {
    let py = func.py();
    Ok(get_type_hints(py)
        .bind(py)
        .call1((func,))?
        .downcast::<PyDict>()?
        .get_item(name)?
        .unwrap_or_else(|| py.None().into_bound(py))
        .downcast_into::<PyType>()?)
}

fn get_first_parameter<'py>(func: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyString>> {
    let py = func.py();
    Ok(signature(py)
        .bind(py)
        .call1((func,))?
        .getattr("parameters")?
        .iter()?
        .next()
        .unwrap_or_else(|| Ok(py.None().into_bound(py)))?
        .downcast_into::<PyString>()?)
}

#[pyfunction]
#[pyo3(signature = (arg, /))]
pub(crate) fn serializer<'py, 'a>(arg: &'a Bound<'py, PyAny>) -> PyResult<&'a Bound<'py, PyAny>> {
    let tp = if arg.is_callable() {
        match get_first_parameter(arg).and_then(|param| get_type(arg, param)) {
            Ok(tp) => tp,
            _ => {
                return Err(PyValueError::new_err(
                    "Cannot extract type from serializer signature",
                ))
            }
        }
    } else {
        arg.downcast::<PyType>()?.clone()
    };
    serializers(arg.py()).bind(arg.py()).set_item(tp, arg)?;
    Ok(arg)
}

#[pyfunction]
#[pyo3(signature = (arg, /))]
pub(crate) fn deserializer<'py, 'a>(arg: &'a Bound<'py, PyAny>) -> PyResult<&'a Bound<'py, PyAny>> {
    let tp = if arg.is_callable() {
        match get_type(arg, "return") {
            Ok(tp) => tp,
            _ => {
                return Err(PyValueError::new_err(
                    "Cannot extract type from serializer signature",
                ))
            }
        }
    } else {
        arg.downcast::<PyType>()?.clone()
    };
    deserializers(arg.py()).bind(arg.py()).set_item(tp, arg)?;
    Ok(arg)
}

wrapper!(zenoh::bytes::ZBytes: Clone);
downcast_or_new!(ZBytes);

#[pymethods]
impl ZBytes {
    #[new]
    fn new(obj: &Bound<PyAny>) -> PyResult<ZBytes> {
        let py = obj.py();
        Ok(Self(if let Ok(b) = obj.downcast::<PyBytes>() {
            zenoh::bytes::ZBytes::new(b.as_bytes().to_vec())
        } else if let Ok(s) = String::extract_bound(obj) {
            zenoh::bytes::ZBytes::serialize(s)
        } else if let Ok(i) = i64::extract_bound(obj) {
            zenoh::bytes::ZBytes::serialize(i)
        } else if let Ok(f) = f64::extract_bound(obj) {
            zenoh::bytes::ZBytes::serialize(f)
        } else if let Ok(b) = bool::extract_bound(obj) {
            zenoh::bytes::ZBytes::serialize(b)
        } else if obj.is_instance_of::<PyList>() || obj.is_instance_of::<PyDict>() {
            let s = String::extract_bound(&dumps(py).bind(py).call1((obj,))?)?;
            zenoh::bytes::ZBytes::serialize(s)
        } else if let Ok(Some(ser)) = serializers(py).bind(py).get_item(obj.get_type()) {
            return match ZBytes::extract_bound(&ser.call1((obj,))?) {
                Ok(b) => Ok(b),
                _ => Err(PyTypeError::new_err(format!(
                    "serializer {} didn't return ZBytes",
                    ser.repr()?
                ))),
            };
        } else {
            bail!("No serializer registered for type {type}", type = obj.get_type().name()?);
        }))
    }

    fn deserialize(this: PyRef<Self>, tp: &Bound<PyType>) -> PyResult<PyObject> {
        let py = tp.py();
        Ok(if tp.eq(PyBytes::type_object_bound(py))? {
            this.__bytes__(py).into_any().unbind()
        } else if tp.eq(PyString::type_object_bound(py))? {
            this.0.deserialize::<Cow<str>>().into_pyres()?.into_py(py)
        } else if tp.eq(PyInt::type_object_bound(py))? {
            this.0.deserialize::<i64>().into_pyres()?.into_py(py)
        } else if tp.eq(PyFloat::type_object_bound(py))? {
            this.0.deserialize::<f64>().into_pyres()?.into_py(py)
        } else if tp.eq(PyBool::type_object_bound(py))? {
            this.0.deserialize::<bool>().into_pyres()?.into_py(py)
        } else if tp.eq(PyList::type_object_bound(py))? || tp.eq(PyDict::type_object_bound(py))? {
            loads(py).bind(py).call1((this.__bytes__(py),))?.unbind()
        } else if let Ok(Some(de)) = deserializers(py).bind(py).get_item(tp) {
            de.call1((this,))?.unbind()
        } else {
            bail!("No deserializer registered for type {type}", type = tp.name()?);
        })
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new_bound_with(py, self.0.len(), |mut bytes| {
            for slice in ZBuf::from(&self.0).slices() {
                let len = slice.len();
                bytes[..len].copy_from_slice(slice);
                bytes = &mut bytes[len..];
            }
            Ok(())
        })
        .unwrap()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}