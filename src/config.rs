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
use pyo3::{
    prelude::*,
    sync::GILOnceCell,
    types::{PyIterator, PyString},
};
use validated_struct::ValidatedMap;
use zenoh::config::Notifier;

use crate::{
    key_expr::KeyExpr,
    utils::{bail, downcast_or_parse, r#enum, try_process, wrapper, IntoPyErr, IntoPyResult},
};

fn string_or_dumps(obj: &Bound<PyAny>) -> PyResult<String> {
    if let Ok(s) = obj.downcast::<PyString>() {
        return Ok(s.to_string());
    }
    static DUMPS: GILOnceCell<PyObject> = GILOnceCell::new();
    let import = || {
        let module = obj.py().import_bound("json")?;
        PyResult::Ok(module.getattr("dumps")?.into())
    };
    Ok(DUMPS
        .get_or_try_init(obj.py(), import)?
        .bind(obj.py())
        .call1((obj,))?
        .downcast_into::<PyString>()?
        .to_string())
}

#[derive(Clone)]
pub(crate) enum ConfigInner {
    Init(zenoh::config::Config),
    Notifier(Notifier<zenoh::config::Config>),
}

#[pyclass]
#[derive(Clone)]
pub(crate) struct Config(pub(crate) ConfigInner);

impl From<zenoh::config::Config> for Config {
    fn from(value: zenoh::config::Config) -> Self {
        Self(ConfigInner::Init(value))
    }
}

impl From<Notifier<zenoh::config::Config>> for Config {
    fn from(value: Notifier<zenoh::config::Config>) -> Self {
        Self(ConfigInner::Notifier(value))
    }
}

impl From<Config> for zenoh::config::Config {
    fn from(value: Config) -> Self {
        match value.0 {
            ConfigInner::Init(cfg) => cfg.clone(),
            ConfigInner::Notifier(cfg) => cfg.lock().clone(),
        }
    }
}

#[pymethods]
impl Config {
    #[new]
    fn new() -> Self {
        Self(ConfigInner::Init(Default::default()))
    }

    #[getter]
    fn id(&self) -> ZenohId {
        match &self.0 {
            ConfigInner::Init(cfg) => *cfg.id(),
            ConfigInner::Notifier(cfg) => *cfg.lock().id(),
        }
        .into()
    }

    #[staticmethod]
    fn from_env() -> PyResult<Self> {
        Ok(Self(ConfigInner::Init(
            zenoh::config::Config::from_env().into_pyres()?,
        )))
    }

    #[staticmethod]
    fn from_file(path: &str) -> PyResult<Self> {
        Ok(Self(ConfigInner::Init(
            zenoh::config::Config::from_file(path).into_pyres()?,
        )))
    }

    #[staticmethod]
    fn from_json5(obj: &Bound<PyAny>) -> PyResult<Self> {
        let json = string_or_dumps(obj)?;
        let mut deserializer = json5::Deserializer::from_str(&json).into_pyres()?;
        match zenoh::config::Config::from_deserializer(&mut deserializer) {
            Ok(cfg) => Ok(Self(ConfigInner::Init(cfg))),
            Err(Ok(_)) => bail!("{json} did parse into a config, but invalid values were found",),
            Err(Err(err)) => Err(err.into_pyerr()),
        }
    }

    fn get_json(&self, key: &str) -> PyResult<String> {
        match &self.0 {
            ConfigInner::Init(cfg) => cfg.get_json(key).into_pyres(),
            ConfigInner::Notifier(cfg) => cfg.get_json(key).into_pyres(),
        }
    }

    fn insert_json5(&mut self, key: &str, value: &Bound<PyAny>) -> PyResult<()> {
        match &mut self.0 {
            ConfigInner::Init(cfg) => cfg.insert_json5(key, &string_or_dumps(value)?).into_pyres(),
            ConfigInner::Notifier(cfg) => {
                cfg.insert_json5(key, &string_or_dumps(value)?).into_pyres()
            }
        }
    }

    fn __repr__(&self) -> String {
        match &self.0 {
            ConfigInner::Init(cfg) => cfg.to_string(),
            ConfigInner::Notifier(cfg) => cfg.lock().to_string(),
        }
    }
}

r#enum!(zenoh::config::WhatAmI: u8 {
    Router = 0b001,
    Peer = 0b010,
    Client = 0b100,
});
downcast_or_parse!(WhatAmI);

#[pymethods]
impl WhatAmI {
    #[new]
    fn new(s: String) -> PyResult<Self> {
        Ok(s.parse::<zenoh::config::WhatAmI>().into_pyres()?.into())
    }

    fn __str__(&self) -> &str {
        zenoh::config::WhatAmI::to_str((*self).into())
    }
}

wrapper!(zenoh::config::WhatAmIMatcher: Clone, Copy);
downcast_or_parse!(WhatAmIMatcher);

#[pymethods]
impl WhatAmIMatcher {
    #[new]
    pub(crate) fn new(s: String) -> PyResult<Self> {
        let res = s.parse().map_err(|_| "invalid WhatAmI matcher");
        Ok(Self(res.into_pyres()?))
    }

    #[staticmethod]
    fn empty() -> Self {
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

    fn __str__(&self) -> &'static str {
        self.0.to_str()
    }
}

wrapper!(zenoh::config::ZenohId);

#[pymethods]
impl ZenohId {
    #[allow(clippy::wrong_self_convention)]
    fn into_keyexpr(&self) -> KeyExpr {
        KeyExpr(self.0.into_keyexpr().into())
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }
}

#[pyfunction]
pub(crate) fn client(peers: Bound<PyIterator>) -> PyResult<Config> {
    let config = try_process(
        peers.map(|obj| {
            zenoh::config::EndPoint::try_from(String::extract_bound(&obj?)?).into_pyres()
        }),
        |peers| zenoh::config::client(peers),
    )?;
    Ok(config.into())
}

#[pyfunction]
pub(crate) fn default() -> Config {
    peer()
}

#[pyfunction]
pub(crate) fn empty() -> Config {
    Config::new()
}

#[pyfunction]
pub(crate) fn peer() -> Config {
    zenoh::config::peer().into()
}
