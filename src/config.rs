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
use std::path::PathBuf;

use pyo3::{prelude::*, types::PyType};

use crate::{
    macros::{downcast_or_new, enum_mapper, wrapper},
    utils::{IntoPyResult, IntoRust},
};

wrapper!(zenoh::Config: Default, Clone);

#[pymethods]
impl Config {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    #[classattr]
    const DEFAULT_CONFIG_PATH_ENV: &'static str = zenoh::Config::DEFAULT_CONFIG_PATH_ENV;

    #[classmethod]
    fn from_env(_cls: &Bound<PyType>) -> PyResult<Self> {
        Ok(Self(zenoh::config::Config::from_env().into_pyres()?))
    }

    #[classmethod]
    fn from_file(_cls: &Bound<PyType>, path: PathBuf) -> PyResult<Self> {
        Ok(Self(zenoh::config::Config::from_file(path).into_pyres()?))
    }

    #[classmethod]
    fn from_json5(_cls: &Bound<PyType>, json: &str) -> PyResult<Self> {
        Ok(Self(zenoh::config::Config::from_json5(json).into_pyres()?))
    }

    fn get_json(&self, key: &str) -> PyResult<String> {
        self.0.get_json(key).into_pyres()
    }

    fn insert_json5(&mut self, key: &str, value: &str) -> PyResult<()> {
        self.0.insert_json5(key, value).into_pyres()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
}

enum_mapper!(zenoh::config::WhatAmI: u8 {
    Router = 0b001,
    Peer = 0b010,
    Client = 0b100,
});
downcast_or_new!(WhatAmI => String);

#[pymethods]
impl WhatAmI {
    #[new]
    fn new(s: String) -> PyResult<Self> {
        Ok(s.parse::<zenoh::config::WhatAmI>().into_pyres()?.into())
    }

    fn __or__(&self, #[pyo3(from_py_with = "WhatAmI::from_py")] other: WhatAmI) -> WhatAmIMatcher {
        (self.into_rust() | other.into_rust()).into()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.into_rust())
    }

    fn __str__(&self) -> &str {
        (*self).into_rust().to_str()
    }
}

wrapper!(zenoh::config::WhatAmIMatcher: Clone, Copy);
downcast_or_new!(WhatAmIMatcher => Option<String>);

impl Default for WhatAmIMatcher {
    fn default() -> Self {
        zenoh::config::WhatAmIMatcher::empty()
            .router()
            .peer()
            .client()
            .into()
    }
}

#[pymethods]
impl WhatAmIMatcher {
    #[new]
    pub(crate) fn new(s: Option<String>) -> PyResult<Self> {
        let Some(s) = s else {
            return Ok(Self(zenoh::config::WhatAmIMatcher::empty()));
        };
        let res = s.parse().map_err(|_| "invalid WhatAmI matcher");
        Ok(Self(res.into_pyres()?))
    }

    #[classmethod]
    fn empty(_cls: &Bound<PyType>) -> Self {
        Self(zenoh::config::WhatAmIMatcher::empty())
    }

    fn router(&self) -> Self {
        Self(zenoh::config::WhatAmIMatcher::router(self.0))
    }

    fn peer(&self) -> Self {
        Self(zenoh::config::WhatAmIMatcher::peer(self.0))
    }

    fn client(&self) -> Self {
        Self(zenoh::config::WhatAmIMatcher::client(self.0))
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn matches(&self, #[pyo3(from_py_with = "WhatAmI::from_py")] whatami: WhatAmI) -> bool {
        self.0.matches(whatami.into())
    }

    fn __contains__(&self, #[pyo3(from_py_with = "WhatAmI::from_py")] whatami: WhatAmI) -> bool {
        self.0.matches(whatami.into())
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> &'static str {
        self.0.to_str()
    }
}

wrapper!(zenoh::config::ZenohId);

#[pymethods]
impl ZenohId {
    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
}
