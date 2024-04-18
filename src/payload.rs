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
    exceptions::PyValueError,
    prelude::*,
    sync::GILOnceCell,
    types::{PyBool, PyBytes, PyDict, PyFloat, PyInt, PyList, PyString, PyType},
    PyTypeInfo,
};
#[doc(inline)]
use zenoh::payload::Payload;
use zenoh_buffers::{buffer::SplitBuffer, ZBuf};

use crate::utils::{bail, into_rust, MapIntoPy, ToPyResult};

into_rust!(Payload);

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

pub(crate) fn payload_to_bytes<'py>(py: Python<'py>, payload: &Payload) -> Bound<'py, PyBytes> {
    PyBytes::new_bound_with(py, payload.len(), |mut bytes| {
        for slice in ZBuf::from(payload).slices() {
            let len = slice.len();
            bytes[..len].copy_from_slice(slice);
            bytes = &mut bytes[len..];
        }
        Ok(())
    })
    .unwrap()
}

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

pub(crate) fn into_payload(obj: &Bound<PyAny>) -> PyResult<Payload> {
    if let Ok(b) = obj.downcast::<PyBytes>() {
        return Ok(Payload::new(b.as_bytes().to_vec()));
    }
    if let Ok(s) = String::extract_bound(obj) {
        return Ok(Payload::serialize(s));
    }
    if let Ok(i) = i64::extract_bound(obj) {
        return Ok(Payload::serialize(i));
    }
    if let Ok(f) = f64::extract_bound(obj) {
        return Ok(Payload::serialize(f));
    }
    if let Ok(b) = bool::extract_bound(obj) {
        return Ok(Payload::serialize(b));
    }
    let py = obj.py();
    if obj.is_instance_of::<PyList>() || obj.is_instance_of::<PyDict>() {
        let s = String::extract_bound(&dumps(py).bind(py).call1((obj,))?)?;
        return Ok(Payload::serialize(s));
    }
    if let Ok(Some(ser)) = serializers(py).bind(py).get_item(obj.get_type()) {
        if let Ok(b) = ser.call1((obj,))?.downcast::<PyBytes>() {
            return Ok(Payload::new(b.as_bytes().to_vec()));
        }
        bail!("serializer {} didn't return bytes", ser.str()?);
    }
    bail!("No serializer registered for type {type}", type = obj.get_type().name()?);
}

pub(crate) fn from_payload(tp: &Bound<PyType>, payload: &Payload) -> PyResult<PyObject> {
    let py = tp.py();
    if tp.eq(PyBytes::type_object_bound(py))? {
        return Ok(payload_to_bytes(py, payload).into_any().unbind());
    }
    if tp.eq(PyString::type_object_bound(py))? {
        return payload.deserialize::<Cow<str>>().to_pyres().map_into_py(py);
    }
    if tp.eq(PyInt::type_object_bound(py))? {
        return payload.deserialize::<i64>().to_pyres().map_into_py(py);
    }
    if tp.eq(PyFloat::type_object_bound(py))? {
        return payload.deserialize::<f64>().to_pyres().map_into_py(py);
    }
    if tp.eq(PyBool::type_object_bound(py))? {
        return payload.deserialize::<bool>().to_pyres().map_into_py(py);
    }
    if tp.eq(PyList::type_object_bound(py))? || tp.eq(PyDict::type_object_bound(py))? {
        return Ok(loads(py)
            .bind(py)
            .call1((payload_to_bytes(py, payload),))?
            .unbind());
    }
    if let Ok(Some(de)) = deserializers(py).bind(py).get_item(tp) {
        return Ok(de.call1((payload_to_bytes(py, payload),))?.unbind());
    }
    bail!("No deserializer registered for type {type}", type = tp.name()?);
}

pub(crate) fn into_payload_opt(obj: &Bound<PyAny>) -> PyResult<Option<Payload>> {
    if obj.is_none() {
        return Ok(None);
    }
    into_payload(obj).map(Some)
}
