use std::collections::HashMap;

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
use pyo3::prelude::*;

use crate::{
    key_expr::KeyExpr,
    utils::{downcast_or_parse, wrapper, IntoPyResult},
};

wrapper!(zenoh::selector::Selector<'static>: Clone);
downcast_or_parse!(Selector);

#[pymethods]
impl Selector {
    #[new]
    pub(crate) fn new(s: String) -> PyResult<Self> {
        Ok(Self(s.parse().into_pyres()?))
    }

    #[getter]
    fn key_expr(&self) -> KeyExpr {
        self.0.key_expr().clone().into_owned().into()
    }

    #[getter]
    fn get_parameters(&self) -> Parameters {
        self.0.parameters().clone().into()
    }

    #[setter]
    fn set_parameters(
        &mut self,
        #[pyo3(from_py_with = "Parameters::from_py")] parameters: Parameters,
    ) {
        self.0.set_parameters(parameters.0)
    }

    fn split(&self) -> (KeyExpr, Parameters) {
        let (k, p) = self.0.clone().split();
        (k.into(), p.into())
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
}

wrapper!(zenoh::selector::Parameters<'static>: Clone);

impl Parameters {
    pub(crate) fn from_py(obj: &Bound<PyAny>) -> PyResult<Self> {
        if let Ok(this) = Self::extract_bound(obj) {
            return Ok(this);
        }
        Self::new(obj)
    }
}

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

    fn get(&self, key: String) -> Option<&str> {
        self.0.get(key)
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

    fn __getitem__(&self, key: String) -> Option<&str> {
        self.get(key)
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
