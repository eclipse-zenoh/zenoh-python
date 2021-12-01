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
use futures::prelude::*;
use pyo3::prelude::*;
use pyo3::{create_exception, wrap_pyfunction};
use zenoh::config::Config as ZConfig;

pub(crate) mod types;
pub(crate) use types::*;
mod session;
use session::*;
use zenoh_util::zerror;
mod encoding;
mod sample_kind;

create_exception!(zenoh, ZError, pyo3::exceptions::PyException);

fn to_pyerr(err: zenoh_util::core::Error) -> PyErr {
    PyErr::new::<ZError, _>(err.to_string())
}
/// The network level zenoh API.
///
/// Examples:
/// ^^^^^^^^^
///
/// Publish
/// """""""
///
/// >>> import zenoh
/// >>> s = zenoh.open({})
/// >>> s.put('/resource/name', bytes('value', encoding='utf8'))
///
/// Subscribe
/// """""""""
///
/// >>> import zenoh
/// >>> from zenoh import SubInfo, Reliability, SubMode
/// >>> def listener(sample):
/// ...     print("Received : {}".format(sample))
/// >>>
/// >>> s = zenoh.open({})
/// >>> sub_info = SubInfo(Reliability.Reliable, SubMode.Push)
/// >>> sub = s.subscribe('/resource/name', sub_info, listener)
///
/// Get
/// """
///
/// >>> import zenoh, time
/// >>> from zenoh import QueryTarget, queryable
/// >>> def get_callback(reply):
/// ...     print("Received : {}".format(reply))
/// >>>
/// >>> s = zenoh.open({})
/// >>> s.get('/resource/name', 'predicate', get_callback)
/// >>> time.sleep(1)
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

    m.add_class::<KeyExpr>()?;
    // force addition of "zenoh.resource_name" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    //     py.run(
    //         "\
    // import sys
    // sys.modules['zenoh.resource_name'] = resource_name
    //         ",
    //         None,
    //         Some(m.dict()),
    //     )?;

    m.add_class::<Config>()?;
    m.add_class::<CongestionControl>()?;
    m.add_class::<ConsolidationMode>()?;
    m.add_class::<encoding::Encoding>()?;
    m.add_class::<Hello>()?;
    m.add_class::<PeerId>()?;
    m.add_class::<Period>()?;
    m.add_class::<Query>()?;
    m.add_class::<Queryable>()?;
    m.add_class::<QueryConsolidation>()?;
    m.add_class::<QueryTarget>()?;
    m.add_class::<Reliability>()?;
    m.add_class::<Reply>()?;
    m.add_class::<KeyExpr>()?;
    m.add_class::<Sample>()?;
    m.add_class::<sample_kind::SampleKind>()?;
    m.add_class::<Session>()?;
    m.add_class::<SourceInfo>()?;
    m.add_class::<SubMode>()?;
    m.add_class::<Subscriber>()?;
    m.add_class::<Target>()?;
    m.add_class::<Timestamp>()?;
    m.add_class::<WhatAmI>()?;
    m.add_wrapped(wrap_pyfunction!(open))?;
    m.add_wrapped(wrap_pyfunction!(scout))?;
    m.add_wrapped(wrap_pyfunction!(init_logger))?;
    m.add_wrapped(wrap_pyfunction!(config_from_file))?;
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

/// Parse a configuration file for zenoh, returning a dictionary of str:str.
/// The file must contain 1 "key=value" property per line. Comments lines starting with '#' character are ignored.
///
/// :param path: The path to the config file.
///
#[pyfunction]
fn config_from_file(path: &str) -> PyResult<Config> {
    Config::from_file(path)
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Config {
    inner: ZConfig,
}
#[pymethods]
impl Config {
    #[new]
    pub fn new() -> Self {
        Config {
            inner: ZConfig::default(),
        }
    }

    pub fn insert_json5(&mut self, key: &str, value: &str) -> bool {
        self.inner.insert_json(key, value).is_ok()
    }
    pub fn json(&self) -> String {
        serde_json::to_string(&self.inner).unwrap()
    }

    pub fn keys(&self) -> Vec<String> {
        use validated_struct::ValidatedMap;
        self.inner.keys()
    }

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

    #[staticmethod]
    pub fn from_file(path: &str) -> PyResult<Self> {
        match ZConfig::from_file(path) {
            Ok(inner) => Ok(Config { inner }),
            Err(e) => Err(to_pyerr(
                match e {
                    zenoh::config::ConfigOpenErr::IoError(e) => zerror!(e),
                    zenoh::config::ConfigOpenErr::JsonParseErr(e) => {
                        zerror!(e)
                    }
                    zenoh::config::ConfigOpenErr::InvalidConfiguration(e) => {
                        zerror!("invalid configuration: {:?}", e)
                    }
                }
                .into(),
            )),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Open a zenoh-net Session.
///
/// :param config: The configuration of the zenoh-net session
/// :type config: dict {str: str}
/// :rtype: Session
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

/// Scout for routers and/or peers.
///
/// This spawns a task that periodically sends scout messages for a specified duration and returns
/// a list of received :class:`Hello` messages.
///
/// :param whatami: The kind of zenoh process to scout for
/// :type whatami: int
/// :param config: The configuration to use for scouting
/// :type config: dict {str: str}
/// :param scout_duration: the duration of scout (in seconds)
/// :type scout_duration: float
/// :rtype: list of :class:`Hello`
///
/// :Example:
///
/// >>> import zenoh
/// >>> hellos = zenoh.scout(zenoh.whatami.PEER | zenoh.whatami.ROUTER, {}, 1.0)
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
