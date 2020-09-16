//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use std::collections::HashMap;
use std::convert::TryFrom;
use zenoh::net::ZInt;

pub(crate) fn path_of_string(s: String) -> PyResult<zenoh::Path> {
    zenoh::Path::try_from(s).map_err(|e| PyErr::new::<exceptions::PyValueError, _>(e.to_string()))
}

pub(crate) fn pathexpr_of_string(s: String) -> PyResult<zenoh::PathExpr> {
    zenoh::PathExpr::try_from(s)
        .map_err(|e| PyErr::new::<exceptions::PyValueError, _>(e.to_string()))
}

pub(crate) fn selector_of_string(s: String) -> PyResult<zenoh::Selector> {
    zenoh::Selector::try_from(s)
        .map_err(|e| PyErr::new::<exceptions::PyValueError, _>(e.to_string()))
}

pub(crate) fn zvalue_of_pyany(obj: &PyAny) -> PyResult<zenoh::Value> {
    match obj.get_type().name().as_ref() {
        "Value" => {
            let v: Value = obj.extract()?;
            Ok(v.v)
        }
        "bytes" => {
            let buf: Vec<u8> = obj.extract()?;
            Ok(zenoh::Value::Raw(
                zenoh::net::encoding::APP_OCTET_STREAM,
                buf.into(),
            ))
        }
        "str" => {
            let s: String = obj.extract()?;
            Ok(zenoh::Value::StringUTF8(s))
        }
        "dict" => {
            let props: HashMap<String, String> = obj.extract()?;
            Ok(zenoh::Value::Properties(zenoh::Properties::from(props)))
        }
        "int" => {
            let i: i64 = obj.extract()?;
            Ok(zenoh::Value::Integer(i))
        }
        "float" => {
            let f: f64 = obj.extract()?;
            Ok(zenoh::Value::Float(f))
        }
        "tuple" => {
            let tuple: &PyTuple = obj.downcast()?;
            if tuple.len() == 2
                && tuple.get_item(0).get_type().name() == "str"
                && tuple.get_item(1).get_type().name() == "bytes"
            {
                let encoding_descr: String = tuple.get_item(0).extract()?;
                let buf: Vec<u8> = tuple.get_item(1).extract()?;
                if let Ok(encoding) = zenoh::net::encoding::from_str(&encoding_descr) {
                    Ok(zenoh::Value::Raw(encoding, buf.into()))
                } else {
                    Ok(zenoh::Value::Custom {
                        encoding_descr,
                        data: buf.into(),
                    })
                }
            } else {
                Err(PyErr::new::<exceptions::PyValueError, _>(format!(
                    "Cannot convert type '{:?}' to a zenoh Value",
                    tuple
                )))
            }
        }
        x => Err(PyErr::new::<exceptions::PyValueError, _>(format!(
            "Cannot convert type '{}' to a zenoh Value",
            x
        ))),
    }
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Value {
    pub(crate) v: zenoh::Value,
}

#[pymethods]
impl Value {
    #[staticmethod]
    fn Raw(encoding: ZInt, buffer: Vec<u8>) -> Value {
        Value {
            v: zenoh::Value::Raw(encoding, buffer.into()),
        }
    }

    #[staticmethod]
    fn Custom(encoding_descr: String, buffer: Vec<u8>) -> Value {
        Value {
            v: zenoh::Value::Custom {
                encoding_descr,
                data: buffer.into(),
            },
        }
    }

    #[staticmethod]
    fn StringUTF8(s: String) -> Value {
        Value {
            v: zenoh::Value::StringUTF8(s),
        }
    }

    #[staticmethod]
    fn Properties(p: HashMap<String, String>) -> Value {
        Value {
            v: zenoh::Value::Properties(zenoh::Properties::from(p)),
        }
    }

    #[staticmethod]
    fn Integer(i: i64) -> Value {
        Value {
            v: zenoh::Value::Integer(i),
        }
    }

    #[staticmethod]
    fn Float(f: f64) -> Value {
        Value {
            v: zenoh::Value::Float(f),
        }
    }
}
