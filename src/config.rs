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
use zenoh::config::Config;
use zenoh_core::zerror;

use crate::{ToPyErr, ToPyResult};

#[pyclass(subclass)]
pub struct _Config(pub(crate) Option<Config>);

#[pymethods]
impl _Config {
    #[allow(clippy::new_without_default)]
    #[new]
    pub fn new() -> Self {
        _Config(Some(Default::default()))
    }
    #[staticmethod]
    pub fn from_file(expr: &str) -> PyResult<Self> {
        match Config::from_file(expr) {
            Ok(k) => Ok(Self(Some(k))),
            Err(e) => Err(e.to_pyerr()),
        }
    }
    #[staticmethod]
    pub fn from_json5(expr: &str) -> PyResult<Self> {
        match Config::from_deserializer(&mut json5::Deserializer::from_str(expr).to_pyres()?) {
            Ok(k) => Ok(Self(Some(k))),
            Err(Ok(_)) => Err(zenoh_core::zerror!(
                "{} did parse into a config, but invalid values were found",
                expr,
            )
            .to_pyerr()),
            Err(Err(e)) => Err(e.to_pyerr()),
        }
    }

    pub fn get_json(&self, path: &str) -> PyResult<String> {
        self.0
            .as_ref()
            .ok_or_else(|| zerror!("Attempted to use a moved configuration").to_pyerr())?
            .get_json(path)
            .to_pyres()
    }

    pub fn insert_json5(&mut self, path: &str, value: &str) -> PyResult<()> {
        self.0
            .as_mut()
            .ok_or_else(|| zerror!("Attempted to use a moved configuration").to_pyerr())?
            .insert_json5(path, value)
            .to_pyres()
    }
}
