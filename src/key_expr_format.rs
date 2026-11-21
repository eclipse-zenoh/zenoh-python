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

use std::collections::HashMap;

use pyo3::prelude::*;

use crate::{
    key_expr::KeyExpr,
    utils::{IntoPyErr, IntoRust},
};

#[pyclass]
pub(crate) struct KeFormat(pub(crate) String);

#[pymethods]
impl KeFormat {
    #[new]
    pub(crate) fn new(format_spec: String) -> PyResult<Self> {
        let format = zenoh::key_expr::format::KeFormat::new(&format_spec)
            .map_err(IntoPyErr::into_pyerr)?;
        // Validate format; we only need to know it parses.
        drop(format);
        Ok(Self(format_spec))
    }

    fn formatter(slf: PyRef<Self>) -> KeFormatter {
        KeFormatter {
            format_spec: slf.0.clone(),
            values: HashMap::new(),
        }
    }

    fn parse(&self, key_expr: &Bound<PyAny>) -> PyResult<Parsed> {
        let key_expr = KeyExpr::from_py(key_expr)?;
        let key_expr_rust = key_expr.into_rust();
        let format = zenoh::key_expr::format::KeFormat::new(&self.0)
            .map_err(IntoPyErr::into_pyerr)?;
        let parsed = format
            .parse(key_expr_rust.as_ref())
            .map_err(IntoPyErr::into_pyerr)?;
        let values: HashMap<String, String> = parsed
            .iter()
            .map(|(id, value)| {
                (
                    id.to_string(),
                    value.map(|v| v.to_string()).unwrap_or_default(),
                )
            })
            .collect();
        Ok(Parsed { values })
    }

    fn __repr__(&self) -> String {
        format!("KeFormat({:?})", self.0)
    }

    fn __str__(&self) -> &str {
        &self.0
    }
}

#[pyclass]
pub(crate) struct Parsed {
    pub(crate) values: HashMap<String, String>,
}

#[pymethods]
impl Parsed {
    fn get(&self, id: &str) -> PyResult<String> {
        self.values
            .get(id)
            .cloned()
            .ok_or_else(|| crate::ZError::new_err(format!("unknown parameter: {}", id)))
    }

    fn __repr__(&self) -> String {
        format!("Parsed({:?})", self.values)
    }
}

#[pyclass]
pub(crate) struct KeFormatter {
    format_spec: String,
    values: HashMap<String, String>,
}

#[pymethods]
impl KeFormatter {
    #[pyo3(signature = (id, value))]
    fn set(&mut self, id: &str, value: &str) -> PyResult<()> {
        let format = zenoh::key_expr::format::KeFormat::new(&self.format_spec)
            .map_err(IntoPyErr::into_pyerr)?;
        let mut formatter = format.formatter();
        formatter.set(id, value).map_err(IntoPyErr::into_pyerr)?;
        self.values.insert(id.to_string(), value.to_string());
        Ok(())
    }

    fn get(&self, id: &str) -> Option<String> {
        self.values.get(id).cloned()
    }

    fn build(&self) -> PyResult<KeyExpr> {
        let format = zenoh::key_expr::format::KeFormat::new(&self.format_spec)
            .map_err(IntoPyErr::into_pyerr)?;
        let mut formatter = format.formatter();
        for (id, value) in &self.values {
            formatter.set(id, value).map_err(IntoPyErr::into_pyerr)?;
        }
        let key_expr = formatter.build().map_err(IntoPyErr::into_pyerr)?;
        Ok(KeyExpr(key_expr.into()))
    }

    fn clear(&mut self) -> () {
        self.values.clear();
    }

    fn __repr__(&self) -> String {
        format!("KeFormatter({:?})", self.format_spec)
    }
}
