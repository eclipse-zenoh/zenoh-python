//
// Copyright (c) 2017, 2022 ZettaScale Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh team, <zenoh@zettascale.tech>
//

#![allow(clippy::borrow_deref_ref)] // false positives with pyo3 macros

use pyo3::prelude::*;
use validated_struct::ValidatedMap;
use zenoh::config::{Config, Notifier};
use zenoh_core::zerror;

use crate::{ToPyErr, ToPyResult};
#[derive(Clone)]
pub(crate) enum PyConfig {
    None,
    Config(Box<Config>),
    Notifier(Notifier<Config>),
}
impl Default for PyConfig {
    fn default() -> Self {
        Self::Config(Default::default())
    }
}
impl PyConfig {
    pub fn take(&mut self) -> Option<Config> {
        if let Self::Config(c) = self {
            let c = unsafe { std::ptr::read(c) };
            unsafe { std::ptr::write(self, PyConfig::None) }
            Some(*c)
        } else {
            None
        }
    }
}
#[pyclass(subclass)]
pub struct _Config(pub(crate) PyConfig);

#[pymethods]
impl _Config {
    #[allow(clippy::new_without_default)]
    #[new]
    pub fn new() -> Self {
        _Config(Default::default())
    }
    #[staticmethod]
    pub fn from_file(expr: &str) -> PyResult<Self> {
        match Config::from_file(expr) {
            Ok(k) => Ok(Self(PyConfig::Config(Box::new(k)))),
            Err(e) => Err(e.to_pyerr()),
        }
    }
    #[staticmethod]
    pub fn from_json5(expr: &str) -> PyResult<Self> {
        match Config::from_deserializer(&mut json5::Deserializer::from_str(expr).to_pyres()?) {
            Ok(k) => Ok(Self(PyConfig::Config(Box::new(k)))),
            Err(Ok(_)) => Err(zenoh_core::zerror!(
                "{} did parse into a config, but invalid values were found",
                expr,
            )
            .to_pyerr()),
            Err(Err(e)) => Err(e.to_pyerr()),
        }
    }

    pub fn get_json(&self, path: &str) -> PyResult<String> {
        match &self.0 {
            PyConfig::None => Err(zerror!("Attempted to use a destroyed configuration").to_pyerr()),
            PyConfig::Config(c) => c.get_json(path).to_pyres(),
            PyConfig::Notifier(c) => c.get_json(path).map_err(|e| e.to_pyerr()),
        }
    }

    pub fn insert_json5(&mut self, path: &str, value: &str) -> PyResult<()> {
        match &mut self.0 {
            PyConfig::None => Err(zerror!("Attempted to use a destroyed configuration").to_pyerr()),
            PyConfig::Config(c) => c.insert_json5(path, value).to_pyres(),
            PyConfig::Notifier(c) => c.insert_json5(path, value).map_err(|e| e.to_pyerr()),
        }
    }
}
