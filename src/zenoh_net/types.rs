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
use crate::to_pyerr;
use async_std::sync::Sender;
use async_std::task;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDateTime};
use pyo3::PyObjectProtocol;
use std::time::Duration;
use zenoh::net::{ResourceId, ZInt};

// zenoh.net.properties (simulate the package as a class, and consts as class attributes)
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

// zenoh.net.whatami (simulate the package as a class, and consts as class attributes)
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

// zenoh.net.resource_name (simulate the package as a class with static methodss)
#[allow(non_camel_case_types)]
#[pyclass]
pub(crate) struct resource_name {}

#[allow(non_snake_case)]
#[pymethods]
impl resource_name {
    #[staticmethod]
    fn intersect(s1: &str, s2: &str) -> bool {
        zenoh::net::utils::resource_name::intersect(s1, s2)
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

// zenoh.net.ResKey (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/417 to be fixed)
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
}

#[pyproto]
impl PyObjectProtocol for ResKey {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.k.to_string())
    }
}

impl From<ResKey> for zenoh::net::ResKey {
    fn from(r: ResKey) -> zenoh::net::ResKey {
        r.k
    }
}

// zenoh.net.PeerId
#[pyclass]
pub(crate) struct PeerId {
    pub(crate) p: zenoh::net::PeerId,
}

#[pyproto]
impl PyObjectProtocol for PeerId {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.p.to_string())
    }
}

// zenoh.net.Timestamp
#[pyclass]
pub(crate) struct Timestamp {
    pub(crate) t: zenoh::Timestamp,
}

#[pymethods]
impl Timestamp {
    #[getter]
    fn time<'p>(&self, py: Python<'p>) -> PyResult<&'p PyDateTime> {
        let f = self.t.get_time().to_duration().as_secs_f64();
        PyDateTime::from_timestamp(py, f, None)
    }

    #[getter]
    fn id(&self) -> PyResult<&[u8]> {
        Ok(self.t.get_id().as_slice())
    }
}

#[pyproto]
impl PyObjectProtocol for Timestamp {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.t.to_string())
    }
}

// zenoh.net.DataInfo
#[pyclass]
#[derive(Clone)]
pub(crate) struct DataInfo {
    pub(crate) i: zenoh::net::DataInfo,
}

#[pymethods]
impl DataInfo {
    #[getter]
    fn source_id(&self) -> PyResult<Option<PeerId>> {
        Ok(self.i.source_id.as_ref().map(|p| PeerId { p: p.clone() }))
    }

    #[getter]
    fn source_sn(&self) -> PyResult<Option<ZInt>> {
        Ok(self.i.source_sn)
    }

    #[getter]
    fn first_router_id(&self) -> PyResult<Option<PeerId>> {
        Ok(self
            .i
            .first_router_id
            .as_ref()
            .map(|p| PeerId { p: p.clone() }))
    }

    #[getter]
    fn first_router_sn(&self) -> PyResult<Option<ZInt>> {
        Ok(self.i.first_router_sn)
    }

    #[getter]
    fn timestamp(&self) -> PyResult<Option<Timestamp>> {
        Ok(self
            .i
            .timestamp
            .as_ref()
            .map(|t| Timestamp { t: t.clone() }))
    }

    #[getter]
    fn kind(&self) -> PyResult<Option<ZInt>> {
        Ok(self.i.kind)
    }

    #[getter]
    fn encoding(&self) -> PyResult<Option<ZInt>> {
        Ok(self.i.encoding)
    }
}

// zenoh.net.Sample
#[pyclass]
#[derive(Clone)]
pub(crate) struct Sample {
    pub(crate) s: zenoh::net::Sample,
}

impl pyo3::conversion::ToPyObject for Sample {
    fn to_object(&self, py: Python) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(pyo3::Py::new(py, self.clone()).unwrap(), py)
    }
}

#[pymethods]
impl Sample {
    #[new]
    fn new(res_name: String, payload: Vec<u8>, data_info: Option<DataInfo>) -> Sample {
        Sample {
            s: zenoh::net::Sample {
                res_name,
                payload: payload.into(),
                data_info: data_info.map(|info| info.i),
            },
        }
    }

    #[getter]
    fn res_name(&self) -> PyResult<&str> {
        Ok(self.s.res_name.as_str())
    }

    #[getter]
    fn payload<'a>(&self, py: Python<'a>) -> PyResult<&'a PyBytes> {
        Ok(PyBytes::new(py, self.s.payload.to_vec().as_slice()))
    }

    #[getter]
    fn data_info(&self) -> PyResult<Option<DataInfo>> {
        Ok(self.s.data_info.as_ref().map(|i| DataInfo { i: i.clone() }))
    }
}

// zenoh.net.Reliability (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/834 to be fixed)
#[pyclass]
#[derive(Clone)]
pub(crate) struct Reliability {
    pub(crate) r: zenoh::net::Reliability,
}

#[allow(non_snake_case)]
#[pymethods]
impl Reliability {
    #[classattr]
    fn BestEffort() -> Reliability {
        Reliability {
            r: zenoh::net::Reliability::BestEffort,
        }
    }

    #[classattr]
    fn Reliable() -> Reliability {
        Reliability {
            r: zenoh::net::Reliability::Reliable,
        }
    }
}

// zenoh.net.SubMode (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/834 to be fixed)
#[pyclass]
#[derive(Clone)]
pub(crate) struct SubMode {
    pub(crate) m: zenoh::net::SubMode,
}

#[allow(non_snake_case)]
#[pymethods]
impl SubMode {
    #[classattr]
    fn Push() -> SubMode {
        SubMode {
            m: zenoh::net::SubMode::Push,
        }
    }

    #[classattr]
    fn Pull() -> SubMode {
        SubMode {
            m: zenoh::net::SubMode::Pull,
        }
    }
}

// zenoh.net.Period
#[pyclass]
#[derive(Clone)]
pub(crate) struct Period {
    pub(crate) p: zenoh::net::Period,
}

#[pymethods]
impl Period {
    #[new]
    fn new(origin: ZInt, period: ZInt, duration: ZInt) -> Period {
        Period {
            p: zenoh::net::Period {
                origin,
                period,
                duration,
            },
        }
    }
}

// zenoh.net.SubInfo
#[pyclass]
pub(crate) struct SubInfo {
    pub(crate) i: zenoh::net::SubInfo,
}

#[pymethods]
impl SubInfo {
    #[new]
    fn new(
        reliability: Option<Reliability>,
        mode: Option<SubMode>,
        period: Option<Period>,
    ) -> SubInfo {
        let mut i = zenoh::net::SubInfo::default();
        if let Some(r) = reliability {
            i.reliability = r.r;
        }
        if let Some(m) = mode {
            i.mode = m.m;
        }
        if let Some(p) = period {
            i.period = Some(p.p);
        }
        SubInfo { i }
    }
}

// zenoh.net.Publisher
#[pyclass(unsendable)]
pub(crate) struct Publisher {
    // Note: because pyo3 doesn't supporting lifetime in PyClass, a workaround is to
    // extend the lifetime of wrapped struct to 'static.
    pub(crate) p: Option<zenoh::net::Publisher<'static>>,
}

#[pymethods]
impl Publisher {
    fn undeclare(&mut self) -> PyResult<()> {
        match self.p.take() {
            Some(p) => task::block_on(p.undeclare()).map_err(to_pyerr),
            None => Ok(()),
        }
    }
}

pub(crate) enum ZnSubOps {
    Pull,
    Undeclare,
}

// zenoh.net.Subscriber
#[pyclass]
pub(crate) struct Subscriber {
    pub(crate) undeclare_tx: Sender<ZnSubOps>,
    pub(crate) loop_handle: Option<async_std::task::JoinHandle<()>>,
}

#[pymethods]
impl Subscriber {
    fn pull(&self) {
        task::block_on(async {
            self.undeclare_tx.send(ZnSubOps::Pull).await;
        });
    }

    fn undeclare(&mut self) {
        if let Some(handle) = self.loop_handle.take() {
            task::block_on(async {
                self.undeclare_tx.send(ZnSubOps::Undeclare).await;
                handle.await;
            });
        }
    }
}

// zenoh.net.queryable (use a class with class attributes for it)
#[allow(non_camel_case_types)]
#[pyclass]
pub(crate) struct queryable {}

#[allow(non_snake_case)]
#[pymethods]
impl queryable {
    #[classattr]
    fn ALL_KINDS() -> ZInt {
        zenoh::net::queryable::ALL_KINDS
    }

    #[classattr]
    fn STORAGE() -> ZInt {
        zenoh::net::queryable::STORAGE
    }

    #[classattr]
    fn EVAL() -> ZInt {
        zenoh::net::queryable::EVAL
    }
}

// zenoh.net.Query
#[pyclass]
#[derive(Clone)]
pub(crate) struct Query {
    pub(crate) q: async_std::sync::Arc<zenoh::net::Query>,
}

impl pyo3::conversion::ToPyObject for Query {
    fn to_object(&self, py: Python) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(pyo3::Py::new(py, self.clone()).unwrap(), py)
    }
}

#[pymethods]
impl Query {
    #[getter]
    fn res_name<'p>(&self) -> PyResult<&str> {
        Ok(self.q.res_name.as_str())
    }

    #[getter]
    fn predicate<'p>(&self) -> PyResult<&str> {
        Ok(self.q.predicate.as_str())
    }

    fn reply(&self, msg: Sample) {
        task::block_on(async {
            self.q.reply(msg.s).await;
        });
    }
}

// zenoh.net.Queryable
#[pyclass]
pub(crate) struct Queryable {
    pub(crate) undeclare_tx: Sender<bool>,
    pub(crate) loop_handle: Option<async_std::task::JoinHandle<()>>,
}

#[pymethods]
impl Queryable {
    fn undeclare(&mut self) {
        if let Some(handle) = self.loop_handle.take() {
            task::block_on(async {
                self.undeclare_tx.send(true).await;
                handle.await;
            });
        }
    }
}
