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
use async_std::prelude::FutureExt;
use async_std::task;
use encoding::KnownEncoding;
use futures::prelude::*;
use pyo3::prelude::*;
use pyo3::{create_exception, wrap_pyfunction};
use pyo3_asyncio::async_std::future_into_py;
use zenoh::config::{Config as ZConfig, Notifier};
use zenoh::prelude::ValidatedMap;
use zenoh_core::zerror;

pub(crate) mod types;
pub(crate) use types::*;
mod session;
use session::*;
mod async_types;
mod encoding;
mod sample_kind;
use async_types::*;
mod async_session;
use async_session::*;

create_exception!(zenoh, ZError, pyo3::exceptions::PyException);

fn to_pyerr(err: zenoh_core::Error) -> PyErr {
    PyErr::new::<ZError, _>(err.to_string())
}
/// The zenoh API.
///
/// Examples of use:
/// ^^^^^^^^^^^^^^^^
///
/// Publish
/// """""""
///
/// >>> import zenoh
/// >>> s = zenoh.open()
/// >>> s.put('/resource/name', bytes('value', 'utf8'))
///
/// Subscribe
/// """""""""
///
/// >>> import zenoh
/// >>> def listener(sample):
/// ...     print("Received: {} = {}".format(sample.key_expr, sample.payload.decode("utf-8")))
/// >>>
/// >>> s = zenoh.open()
/// >>> sub = s.subscribe('/resource/*', listener)
///
/// Get
/// """
///
/// >>> import zenoh
/// >>> s = zenoh.open()
/// >>> for reply in s.get('/resource/name'):
/// ...     print("Received: {} = {}".format(reply.sample.key_expr, reply.sample.payload.decode('utf-8'))
#[pymodule]
pub fn zenoh(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<config>()?;
    // force addition of "zenoh.config" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.config'] = config
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<info>()?;
    // force addition of "zenoh.info" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.info'] = info
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<queryable>()?;
    // force addition of "zenoh.queryable" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.queryable'] = queryable
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<AsyncQueryable>()?;
    m.add_class::<AsyncSession>()?;
    m.add_class::<AsyncSubscriber>()?;
    m.add_class::<Config>()?;
    m.add_class::<CongestionControl>()?;
    m.add_class::<ConsolidationMode>()?;
    m.add_class::<encoding::Encoding>()?;
    m.add_class::<Hello>()?;
    m.add_class::<KeyExpr>()?;
    m.add_class::<KnownEncoding>()?;
    m.add_class::<PeerId>()?;
    m.add_class::<Period>()?;
    m.add_class::<Priority>()?;
    m.add_class::<Query>()?;
    m.add_class::<Queryable>()?;
    m.add_class::<QueryConsolidation>()?;
    m.add_class::<QueryTarget>()?;
    m.add_class::<Reliability>()?;
    m.add_class::<Reply>()?;
    m.add_class::<Sample>()?;
    m.add_class::<sample_kind::SampleKind>()?;
    m.add_class::<Selector>()?;
    m.add_class::<Session>()?;
    m.add_class::<SourceInfo>()?;
    m.add_class::<SubMode>()?;
    m.add_class::<Subscriber>()?;
    m.add_class::<Target>()?;
    m.add_class::<Timestamp>()?;
    m.add_class::<Value>()?;
    m.add_class::<ValueSelector>()?;
    m.add_class::<WhatAmI>()?;
    m.add("ZError", py.get_type::<ZError>())?;
    m.add_wrapped(wrap_pyfunction!(init_logger))?;
    m.add_wrapped(wrap_pyfunction!(config_from_file))?;
    m.add_wrapped(wrap_pyfunction!(open))?;
    m.add_wrapped(wrap_pyfunction!(async_open))?;
    m.add_wrapped(wrap_pyfunction!(scout))?;
    m.add_wrapped(wrap_pyfunction!(async_scout))?;
    Ok(())
}
/// Initialize the logger used by the Rust implementation of this API.
///
/// Once initialized, you can configure the logs displayed by the API using the ``RUST_LOG`` environment variable.
/// For instance, start python with the *debug* logs available::
///
///    $ RUST_LOG=debug python
///
/// More details on the RUST_LOG configuration on https://docs.rs/env_logger/latest/env_logger
///
#[pyfunction]
fn init_logger() {
    env_logger::init();
}

/// Parse a configuration file for zenoh, returning a Config object.
///
/// :param path: The path to the config file.
/// :rtype: :class:`Config`
#[pyfunction]
fn config_from_file(path: &str) -> PyResult<Config> {
    Config::from_file(path)
}

/// The main configuration structure for Zenoh.
///
/// To construct a configuration, we advise that you use a configuration file
/// (JSON, JSON5 and YAML are currently supported, please use the proper extension for your format as the deserializer will be picked according to it).
/// A Config object can then be amended calling :func:`Config.insert_json5`.
///
/// :Example:
///
/// >>> import zenoh, json
/// >>> conf = zenoh.Config.from_file('zenoh-config.json5')
/// >>> conf.insert_json5(zenoh.config.MODE_KEY, json.dumps('client'))
/// >>> print(conf.json())
#[pyclass]
#[derive(Debug, Clone)]
pub struct Config {
    inner: ZConfig,
}
#[pymethods]
impl Config {
    /// Constructor of a default configuration.
    #[new]
    pub fn new() -> Self {
        Config {
            inner: ZConfig::default(),
        }
    }

    /// Parse a configuration file for zenoh, returning a Config object.
    ///
    /// :param path: The path to the config file.
    /// :rtype: :class:`Config`
    /// :raise: :class:`ZError`
    pub fn insert_json5(&mut self, key: &str, value: &str) -> PyResult<()> {
        match self.inner.insert_json5(key, value) {
            Ok(()) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                e.to_string(),
            )),
        }
    }

    /// Returns the config as a JSON string
    ///
    /// :rtype: str
    pub fn json(&self) -> String {
        serde_json::to_string(&self.inner).unwrap()
    }

    /// Parse a JSON5 string configuration for zenoh, returning a Config object.
    ///
    /// :param input: The configuration as a JSON5 string.
    /// :rtype: :class:`Config`
    /// :raise: :class:`ZError`
    #[staticmethod]
    pub fn from_json5(input: &str) -> PyResult<Self> {
        let mut d = match json5::Deserializer::from_str(input) {
            Ok(d) => d,
            Err(e) => return Err(to_pyerr(zerror!(e).into())),
        };
        match ZConfig::from_deserializer(&mut d) {
            Ok(inner) => Ok(Config { inner }),
            Err(e) => Err(to_pyerr(
                match e {
                    Ok(c) => zerror!("invalid configuration: {:?}", c),
                    Err(e) => zerror!(e),
                }
                .into(),
            )),
        }
    }

    /// Parse a configuration file for zenoh, returning a Config object.
    ///
    /// :param path: The path to the config file.
    /// :rtype: :class:`Config`
    /// :raise: :class:`ZError`
    #[staticmethod]
    pub fn from_file(path: &str) -> PyResult<Self> {
        match ZConfig::from_file(path) {
            Ok(inner) => Ok(Config { inner }),
            Err(e) => Err(to_pyerr(e)),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[pyclass]
#[derive(Clone)]
pub struct ConfigNotifier {
    inner: Notifier<ZConfig>,
}
#[pymethods]
impl ConfigNotifier {
    pub fn insert_json5(&mut self, key: &str, value: &str) -> PyResult<()> {
        match self.inner.insert_json5(key, value) {
            Ok(()) => Ok(()),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyException, _>(
                e.to_string(),
            )),
        }
    }
    pub fn json(&self) -> String {
        serde_json::to_string(&*self.inner.lock()).unwrap()
    }
}

/// Open a zenoh Session.
///
/// :param config: The configuration of the zenoh session
/// :type config: :class:`Config`, optional
/// :rtype: :class:`Session`
/// :raise: :class:`ZError`
///
/// :Example:
///
/// >>> import zenoh
/// >>> z = zenoh.open(zenoh.config::peer())
#[pyfunction]
#[pyo3(text_signature = "(config)")]
fn open(config: Option<Config>) -> PyResult<Session> {
    let s = task::block_on(zenoh::open(config.unwrap_or_default().inner)).map_err(to_pyerr)?;
    Ok(Session::new(s))
}

/// Coroutine to open a zenoh AsyncSession (similar to a Session, but for asyncio usage).
///
/// :param config: The configuration of the zenoh session
/// :type config: :class:`Config`, optional
/// :rtype: :class:`AsyncSession`
/// :raise: :class:`ZError`
///
/// :Example:
///
/// >>> import asyncio, zenoh
/// >>> async def main():
/// >>>    z = await zenoh.async_open()
/// >>>
/// >>> asyncio.run(main())
#[pyfunction]
#[pyo3(text_signature = "(config)")]
fn async_open(py: Python, config: Option<Config>) -> PyResult<&PyAny> {
    future_into_py(py, async {
        let s = zenoh::open(config.unwrap_or_default().inner)
            .await
            .map_err(to_pyerr)?;
        Ok(AsyncSession::new(s))
    })
}

/// Scout for routers and/or peers.
///
/// Sends scout messages for a specified duration and returns
/// a list of received :class:`Hello` messages.
///
/// :param whatami: The kind of zenoh process to scout for
/// :type whatami: **int**
/// :param scout_duration: the duration of scout (in seconds)
/// :type scout_duration: **float**
/// :param config: The configuration to use for scouting
/// :type config: :class:`Config`, optional
/// :rtype: list of :class:`Hello`
/// :raise: :class:`ZError`
///
/// :Example:
///
/// >>> import zenoh
/// >>> hellos = zenoh.scout(WhatAmI.Peer | WhatAmI.Router, 1.0)
/// >>> for hello in hellos:
/// >>>     print(hello)
#[pyfunction]
#[pyo3(text_signature = "(whatami, scout_duration, config)")]
fn scout(whatami: WhatAmI, scout_duration: f64, config: Option<Config>) -> PyResult<Vec<Hello>> {
    task::block_on(async move {
        let mut result = Vec::<Hello>::new();
        let mut receiver = zenoh::scout(whatami, config.unwrap_or_default().inner)
            .await
            .unwrap();
        let scout = async {
            while let Some(h) = receiver.next().await {
                result.push(Hello { h })
            }
        };
        let timeout = async_std::task::sleep(std::time::Duration::from_secs_f64(scout_duration));
        FutureExt::race(scout, timeout).await;
        Ok(result)
    })
}

/// Coroutine to scout for routers and/or peers.
///
/// Sends scout messages for a specified duration and returns
/// a list of received :class:`Hello` messages.
///
/// :param whatami: The kind of zenoh process to scout for
/// :type whatami: **int**
/// :param scout_duration: the duration of scout (in seconds)
/// :type scout_duration: **float**
/// :param config: The configuration to use for scouting
/// :type config: :class:`Config`, optional
/// :rtype: list of :class:`Hello`
/// :raise: :class:`ZError`
///
/// :Example:
///
/// >>> import asyncio, zenoh
/// >>> async def main():
/// >>>    hellos = await zenoh.async_scout(WhatAmI.Peer | WhatAmI.Router, 1.0)
/// >>>    for hello in hellos:
/// >>>       print(hello)
/// >>>
/// >>> asyncio.run(main())
#[pyfunction]
#[pyo3(text_signature = "(whatami, scout_duration, config)")]
fn async_scout(
    whatami: WhatAmI,
    scout_duration: f64,
    config: Option<Config>,
    py: Python,
) -> PyResult<&PyAny> {
    future_into_py(py, async move {
        let mut result = Vec::<Hello>::new();
        let mut receiver = zenoh::scout(whatami, config.unwrap_or_default().inner)
            .await
            .unwrap();
        let scout = async {
            while let Some(h) = receiver.next().await {
                result.push(Hello { h })
            }
        };
        let timeout = async_std::task::sleep(std::time::Duration::from_secs_f64(scout_duration));
        FutureExt::race(scout, timeout).await;
        Ok(result)
    })
}
