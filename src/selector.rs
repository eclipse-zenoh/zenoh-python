use std::borrow::Cow;
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
    macros::{downcast_or_new, wrapper},
    utils::{IntoPyResult, MapInto},
};

wrapper!(zenoh::selector::Selector<'static>: Clone);
downcast_or_new!(Selector, None);

#[pymethods]
impl Selector {
    #[new]
    #[pyo3(signature = (arg, /, parameters = None))]
    pub(crate) fn new(
        arg: &Bound<PyAny>,
        #[pyo3(from_py_with = "Parameters::from_py_opt")] parameters: Option<Parameters>,
    ) -> PyResult<Self> {
        Ok(Self(if let Some(params) = parameters {
            (KeyExpr::from_py(arg)?.0, params.0).into()
        } else if let Ok(s) = String::extract_bound(arg) {
            s.parse().into_pyres()?
        } else if let Ok(k) = KeyExpr::extract_bound(arg) {
            k.0.into()
        } else {
            return Err(String::extract_bound(arg).unwrap_err());
        }))
    }

    #[getter]
    fn get_key_expr(&self) -> KeyExpr {
        self.0.key_expr.clone().into_owned().into()
    }

    #[setter]
    fn set_key_expr(&mut self, #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr) {
        self.0.key_expr = Cow::Owned(key_expr.0)
    }

    #[getter]
    fn get_parameters(&self) -> Parameters {
        self.0.parameters.clone().into_owned().into()
    }

    #[setter]
    fn set_parameters(
        &mut self,
        #[pyo3(from_py_with = "Parameters::from_py")] parameters: Parameters,
    ) {
        self.0.parameters = Cow::Owned(parameters.0)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
}

wrapper!(zenoh::selector::Parameters<'static>: Clone);
downcast_or_new!(Parameters);

#[pymethods]
impl Parameters {
    #[new]
    pub(crate) fn new(obj: &Bound<PyAny>) -> PyResult<Self> {
        if let Ok(map) = <HashMap<String, String>>::extract_bound(obj) {
            return Ok(Self(map.into()));
        }
        Ok(Self(String::extract_bound(obj)?.into()))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn contains_key(&self, key: String) -> bool {
        self.0.contains_key(key)
    }

    #[pyo3(signature = (key, default = None))]
    fn get(&self, key: String, default: Option<String>) -> Option<String> {
        self.0.get(key).map_into().or(default)
    }

    fn values(&self, key: String) -> Vec<&str> {
        self.0.values(key).collect()
    }

    fn insert(&mut self, key: String, value: String) -> Option<String> {
        self.0.insert(key, value)
    }

    fn remove(&mut self, key: String) -> Option<String> {
        self.0.remove(key)
    }

    fn extend(&mut self, #[pyo3(from_py_with = "Parameters::from_py")] parameters: Parameters) {
        self.0.extend(&parameters.0)
    }

    fn is_ordered(&self) -> bool {
        self.0.is_ordered()
    }

    fn __bool__(&self) -> bool {
        !self.0.is_empty()
    }

    fn __contains__(&self, key: String) -> bool {
        self.contains_key(key)
    }

    fn __getitem__(&self, key: String) -> Option<String> {
        self.get(key, None)
    }

    fn __iter__(&self) -> Vec<(&str, &str)> {
        self.0.iter().collect()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> &str {
        self.0.as_str()
    }
}
