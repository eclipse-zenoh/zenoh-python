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
use std::time::Duration;
use zenoh::net::{ResourceId, ZInt};

// zenoh.net.properties (use a class with class attributes for it)
#[allow(non_camel_case_types)]
#[pyclass]
pub(crate) struct properties {}

#[allow(non_snake_case)]
#[pymethods]
impl properties {
    #[classattr]
    fn ZN_USER_KEY() -> ZInt {
        zenoh::net::properties::ZN_USER_KEY
    }

    #[classattr]
    fn ZN_PASSWD_KEY() -> ZInt {
        zenoh::net::properties::ZN_PASSWD_KEY
    }

    #[classattr]
    fn ZN_INFO_PID_KEY() -> ZInt {
        zenoh::net::properties::ZN_INFO_PID_KEY
    }

    #[classattr]
    fn ZN_INFO_PEER_PID_KEY() -> ZInt {
        zenoh::net::properties::ZN_INFO_PEER_PID_KEY
    }

    #[classattr]
    fn ZN_INFO_ROUTER_PID_KEY() -> ZInt {
        zenoh::net::properties::ZN_INFO_ROUTER_PID_KEY
    }

    #[staticmethod]
    fn to_str(i: ZInt) -> PyResult<String> {
        zenoh::net::properties::to_str(i)
            .map_err(|e| PyErr::new::<exceptions::ValueError, _>(e.to_string()))
    }
}

// zenoh.net.whatami (use a class with class attributes for it)
#[allow(non_camel_case_types)]
#[pyclass]
pub(crate) struct whatami {}

#[allow(non_snake_case)]
#[pymethods]
impl whatami {
    #[classattr]
    fn PEER() -> ZInt {
        zenoh::net::whatami::PEER
    }

    #[classattr]
    fn CLIENT() -> ZInt {
        zenoh::net::whatami::CLIENT
    }
}

// zenoh.net.Config
#[pyclass]
#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) c: zenoh::net::Config,
}

#[pymethods]
impl Config {
    #[new]
    fn new(
        mode: Option<ZInt>,
        peers: Option<Vec<&str>>,
        listeners: Option<Vec<&str>>,
        multicast_interface: Option<String>,
        scouting_delay: Option<f64>,
        add_timestamp: Option<bool>,
    ) -> Config {
        let mut c = zenoh::net::Config::default();
        if let Some(m) = mode {
            c = c.mode(m);
        }
        if let Some(p) = peers {
            c = c.add_peers(p);
        }
        if let Some(l) = listeners {
            c = c.add_listeners(l);
        }
        if let Some(m) = multicast_interface {
            c = c.multicast_interface(m);
        }
        if let Some(d) = scouting_delay {
            c = c.scouting_delay(Duration::from_secs_f64(d));
        }
        if let Some(true) = add_timestamp {
            c = c.add_timestamp();
        }
        Config { c }
    }

    #[staticmethod]
    fn parse_mode(mode: &str) -> PyResult<ZInt> {
        zenoh::net::Config::parse_mode(mode).map_err(|_| {
            PyErr::new::<exceptions::ValueError, _>(format!("Invalid Config mode: '{}'", mode))
        })
    }
}

// zenoh.net.ResKey (enum simulated via a Python class)
#[pyclass]
pub(crate) struct ResKey {
    pub(crate) k: zenoh::net::ResKey,
}

#[allow(non_snake_case)]
#[pymethods]
impl ResKey {
    #[staticmethod]
    fn RName(name: String) -> ResKey {
        ResKey {
            k: zenoh::net::ResKey::RName(name),
        }
    }

    #[staticmethod]
    fn RId(id: ResourceId) -> ResKey {
        ResKey {
            k: zenoh::net::ResKey::RId(id),
        }
    }

    #[staticmethod]
    fn RIdWithSuffix(id: ResourceId, suffix: String) -> ResKey {
        ResKey {
            k: zenoh::net::ResKey::RIdWithSuffix(id, suffix),
        }
    }

    fn rid(&self) -> ResourceId {
        self.k.rid()
    }

    fn is_numerical(&self) -> bool {
        self.k.is_numerical()
    }

    fn to_string(&self) -> String {
        self.k.to_string()
    }
}
