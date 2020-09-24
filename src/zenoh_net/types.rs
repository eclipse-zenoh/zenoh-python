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
use pyo3::types::{PyBytes, PyDateTime, PyTuple};
use pyo3::PyObjectProtocol;
use std::time::Duration;
use zenoh::net::{ResourceId, ZInt};

// zenoh.net.properties (simulate the package as a class, and consts as class attributes)
/// Constants defining the keys of the properties returned in :meth:`Session.info`.
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
            .map_err(|e| PyErr::new::<exceptions::PyValueError, _>(e.to_string()))
    }
}

// zenoh.net.whatami (simulate the package as a class, and consts as class attributes)
/// Constants defining the different configuration modes of a zenoh :class:`Session`.
#[allow(non_camel_case_types)]
#[pyclass]
pub(crate) struct whatami {}

#[allow(non_snake_case)]
#[pymethods]
impl whatami {
    #[classattr]
    fn ROUTER() -> ZInt {
        zenoh::net::whatami::ROUTER
    }

    #[classattr]
    fn PEER() -> ZInt {
        zenoh::net::whatami::PEER
    }

    #[classattr]
    fn CLIENT() -> ZInt {
        zenoh::net::whatami::CLIENT
    }

    #[staticmethod]
    fn to_str(i: ZInt) -> PyResult<String> {
        Ok(zenoh::net::whatami::to_str(i))
    }
}

/// A Hello message received as a response to a :meth:`scout`
#[pyclass]
#[derive(Clone)]
pub(crate) struct Hello {
    pub(crate) h: zenoh::net::Hello,
}

#[pymethods]
impl Hello {
    /// The PeerId of the Hello message sender
    ///
    /// :type: :class:`PeerId` or ``None``
    #[getter]
    fn pid(&self) -> PyResult<Option<PeerId>> {
        Ok(self.h.pid.as_ref().map(|p| PeerId { p: p.clone() }))
    }

    /// The mode of the Hello message sender (bitmask of constants from :class:`whatami`)
    ///
    /// :type: int or ``None``
    #[getter]
    fn whatami(&self) -> PyResult<Option<ZInt>> {
        Ok(self.h.whatami)
    }

    /// The locators list of the Hello message sender
    ///
    /// :type: list of str or ``None``
    #[getter]
    fn locators(&self) -> PyResult<Option<Vec<String>>> {
        Ok(self
            .h
            .locators
            .as_ref()
            .map(|v| v.iter().map(|l| l.to_string()).collect()))
    }
}

#[pyproto]
impl PyObjectProtocol for Hello {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.h.to_string())
    }
}

// zenoh.net.resource_name (simulate the package as a class with static methodss)
#[allow(non_camel_case_types)]
#[pyclass]
pub(crate) struct resource_name {}

#[allow(non_snake_case)]
#[pymethods]
impl resource_name {
    /// Return true if both resource names intersect.
    ///
    /// :param s1: the 1st resource name
    /// :type s1: str
    /// :param s2: the 2nd resource name
    /// :type s2: str
    #[staticmethod]
    #[text_signature = "(s1, s2)"]
    fn intersect(s1: &str, s2: &str) -> bool {
        zenoh::net::utils::resource_name::intersect(s1, s2)
    }
}

/// Struct to pass to :meth:`zenoh.net.open` to configure the zenoh-net :class:`Session`.
///
/// :param mode: the Session mode, as a bitmask of constants from :class:`whatami`. (Default: :attr:`whatami.PEER`)
/// :type mode: int
/// :param peers: a list of peers locators to connect.
/// :type peers: list of str
/// :param listeners: a list of locators to listen for connections
/// :type listeners: list of str
/// :param multicast_interface: a network interface to use for multicast scouting
/// :type multicast_interface: str
/// :param scouting_delay: the delay of scouting (in seconds)
/// :type scouting_delay: float
/// :param add_timestamp: true if a timestamp must be added with each published data
/// :type add_timestamp: bool
/// :rtype: Session
#[pyclass]
#[text_signature = "(mode=zenoh.net.whatami.PEER, peers=None, listeners=None, multicast_interface=None, scouting_delay=0.25, add_timestamp=False)"]
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

    /// Parse a string representing a mode. Accepted values: 'peer', 'client', 'router'
    ///
    /// :param mode: the mode as a string
    /// :type mode: str
    /// :rtype: int
    #[staticmethod]
    #[text_signature = "(mode)"]
    fn parse_mode(mode: &str) -> PyResult<ZInt> {
        zenoh::net::Config::parse_mode(mode).map_err(|_| {
            PyErr::new::<exceptions::PyValueError, _>(format!("Invalid Config mode: '{}'", mode))
        })
    }
}

#[pyproto]
impl PyObjectProtocol for Config {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.c.to_string())
    }
}

// zenoh.net.ResKey (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/417 to be fixed)
//
/// A resource key
#[pyclass]
#[derive(Clone)]
pub(crate) struct ResKey {
    pub(crate) k: zenoh::net::ResKey,
}

#[allow(non_snake_case)]
#[pymethods]
impl ResKey {
    /// Creates a resource key from a name.
    ///
    /// :param name: the resrouce name
    /// :type name: str
    #[staticmethod]
    #[text_signature = "(name)"]
    fn RName(name: String) -> ResKey {
        ResKey {
            k: zenoh::net::ResKey::RName(name),
        }
    }

    /// Creates a resource key from a resource id returned by :meth:`Session.declare_resource`.
    ///
    /// :param id: the resrouce id
    /// :type id: int
    #[staticmethod]
    #[text_signature = "(id)"]
    fn RId(id: ResourceId) -> ResKey {
        ResKey {
            k: zenoh::net::ResKey::RId(id),
        }
    }

    /// Creates a resource key from a resource id returned by :meth:`Session.declare_resource` and a suffix.
    ///
    /// :param id: the resrouce id
    /// :type id: int
    /// :param suffix: the suffix
    /// :type suffix: str
    #[staticmethod]
    #[text_signature = "(id, suffix)"]
    fn RIdWithSuffix(id: ResourceId, suffix: String) -> ResKey {
        ResKey {
            k: zenoh::net::ResKey::RIdWithSuffix(id, suffix),
        }
    }

    /// Returns the resource id, or ``0`` if the resource key is a :meth:`RName`.
    fn rid(&self) -> ResourceId {
        self.k.rid()
    }

    /// Returns ``True`` if the resource key is a :meth:`RId`.
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

pub(crate) fn znreskey_of_pyany(obj: &PyAny) -> PyResult<zenoh::net::ResKey> {
    match obj.get_type().name().as_ref() {
        "ResKey" => {
            let rk: ResKey = obj.extract()?;
            Ok(rk.k)
        }
        "int" => {
            let id: u64 = obj.extract()?;
            Ok(zenoh::net::ResKey::RId(id))
        }
        "str" => {
            let name: String = obj.extract()?;
            Ok(zenoh::net::ResKey::RName(name))
        }
        "tuple" => {
            let tuple: &PyTuple = obj.downcast()?;
            if tuple.len() == 2
                && tuple.get_item(0).get_type().name() == "int"
                && tuple.get_item(1).get_type().name() == "str"
            {
                let id: u64 = tuple.get_item(0).extract()?;
                let suffix: String = tuple.get_item(1).extract()?;
                Ok(zenoh::net::ResKey::RIdWithSuffix(id, suffix))
            } else {
                Err(PyErr::new::<exceptions::PyValueError, _>(format!(
                    "Cannot convert type '{:?}' to a zenoh-net ResKey",
                    tuple
                )))
            }
        }
        x => Err(PyErr::new::<exceptions::PyValueError, _>(format!(
            "Cannot convert type '{}' to a zenoh-net ResKey",
            x
        ))),
    }
}

/// A Peer id
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

/// A Timestamp composed of a time and the identifier of the timestamp source.
#[pyclass]
pub(crate) struct Timestamp {
    pub(crate) t: zenoh::Timestamp,
}

#[pymethods]
impl Timestamp {
    /// The time
    ///
    /// :type: datetime.datetime
    #[getter]
    fn time<'p>(&self, py: Python<'p>) -> PyResult<&'p PyDateTime> {
        let f = self.t.get_time().to_duration().as_secs_f64();
        PyDateTime::from_timestamp(py, f, None)
    }

    /// The identifier of the timestamp source
    ///
    /// :type: bytes
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

/// Some informations about the associated data
#[pyclass]
#[derive(Clone)]
pub(crate) struct DataInfo {
    pub(crate) i: zenoh::net::DataInfo,
}

#[pymethods]
impl DataInfo {
    /// The :class:`PeerId` of the data source.
    ///
    /// :type: :class:`PeerId` or ``None``
    #[getter]
    fn source_id(&self) -> PyResult<Option<PeerId>> {
        Ok(self.i.source_id.as_ref().map(|p| PeerId { p: p.clone() }))
    }

    /// The source sequence number of the data.
    ///
    /// :type: int or ``None``
    #[getter]
    fn source_sn(&self) -> PyResult<Option<ZInt>> {
        Ok(self.i.source_sn)
    }

    /// The :class:`PeerId` of the 1st router that routed the data.
    ///
    /// :type: :class:`PeerId` or ``None``
    #[getter]
    fn first_router_id(&self) -> PyResult<Option<PeerId>> {
        Ok(self
            .i
            .first_router_id
            .as_ref()
            .map(|p| PeerId { p: p.clone() }))
    }

    /// The first router sequence number of the data.
    ///
    /// :type: int or ``None``
    #[getter]
    fn first_router_sn(&self) -> PyResult<Option<ZInt>> {
        Ok(self.i.first_router_sn)
    }

    /// The :class:`Timestamp` of the data.
    ///
    /// :type: :class:`Timestamp` or ``None``
    #[getter]
    fn timestamp(&self) -> PyResult<Option<Timestamp>> {
        Ok(self
            .i
            .timestamp
            .as_ref()
            .map(|t| Timestamp { t: t.clone() }))
    }

    /// The kind of the data.
    ///
    /// :type: int or ``None``
    #[getter]
    fn kind(&self) -> PyResult<Option<ZInt>> {
        Ok(self.i.kind)
    }

    /// The encoding flag of the data.
    ///
    /// :type: int or ``None``
    #[getter]
    fn encoding(&self) -> PyResult<Option<ZInt>> {
        Ok(self.i.encoding)
    }
}

/// A zenoh sample.
///
/// :param res_name: the resource name
/// :type res_name: str
/// :param payload: the data payload
/// :type payload: bytes
/// :param data_info: some information about the data
/// :type data_info: DataInfo, optional
#[pyclass]
#[text_signature = "(res_name, payload, data_info=None)"]
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

    /// The resource name
    ///
    /// :type: str
    #[getter]
    fn res_name(&self) -> PyResult<&str> {
        Ok(self.s.res_name.as_str())
    }

    /// The data payload
    ///
    /// :type: bytes
    #[getter]
    fn payload<'a>(&self, py: Python<'a>) -> PyResult<&'a PyBytes> {
        Ok(PyBytes::new(py, self.s.payload.to_vec().as_slice()))
    }

    /// Some information about the data
    ///
    /// :type: :class:`DataInfo` or ``None``
    #[getter]
    fn data_info(&self) -> PyResult<Option<DataInfo>> {
        Ok(self.s.data_info.as_ref().map(|i| DataInfo { i: i.clone() }))
    }
}

#[pyproto]
impl PyObjectProtocol for Sample {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.s))
    }

    fn __repr__(&self) -> PyResult<String> {
        self.__str__()
    }

    fn __format__(&self, _format_spec: &str) -> PyResult<String> {
        self.__str__()
    }
}

// zenoh.net.Reliability (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/834 to be fixed)
//
/// The kind of reliability
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
//
/// The subscription mode.
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

/// A time period.
#[pyclass]
#[text_signature = "(origin, period, duration)"]
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

/// Informations to configure a subscription.
///
/// :param reliability: the reliability mode (default: :attr:`Reliability.Reliable`)
/// :type reliability: Reliability, optional
/// :param mode: the subscription mode (default: :attr:`SubMode.Push`)
/// :type mode: SubMode, optional
/// :param period: the pull period
/// :type period: Period, optional
#[pyclass]
#[text_signature = "(reliability=None, mode=None, period=None)"]
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

/// A publisher
#[pyclass(unsendable)]
pub(crate) struct Publisher {
    // Note: because pyo3 doesn't supporting lifetime in PyClass, a workaround is to
    // extend the lifetime of wrapped struct to 'static.
    pub(crate) p: Option<zenoh::net::Publisher<'static>>,
}

#[pymethods]
impl Publisher {
    /// Undeclare the publisher.
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

/// A subscriber
#[pyclass]
pub(crate) struct Subscriber {
    pub(crate) undeclare_tx: Sender<ZnSubOps>,
    pub(crate) loop_handle: Option<async_std::task::JoinHandle<()>>,
}

#[pymethods]
impl Subscriber {
    /// Pull available data for a pull-mode subscriber.
    fn pull(&self) {
        task::block_on(async {
            self.undeclare_tx.send(ZnSubOps::Pull).await;
        });
    }

    /// Undeclare the subscriber.
    fn undeclare(&mut self) {
        if let Some(handle) = self.loop_handle.take() {
            task::block_on(async {
                self.undeclare_tx.send(ZnSubOps::Undeclare).await;
                handle.await;
            });
        }
    }
}

// zenoh.net.queryable (simulate the package as a class, and consts as class attributes)
//
/// Constants defining the different modes of a zenoh :class:`Queryable`.
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

/// Type received by a queryable callback. See :meth:`Session.declare_queryable`.
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
    /// The resrouce name of the query
    ///
    /// :type: str
    #[getter]
    fn res_name(&self) -> PyResult<&str> {
        Ok(self.q.res_name.as_str())
    }

    /// The predicate of the query
    ///
    /// :type: str
    #[getter]
    fn predicate(&self) -> PyResult<&str> {
        Ok(self.q.predicate.as_str())
    }

    /// Send a reply to the query
    ///
    /// :param sample: the reply sample
    /// :type: Sample
    #[text_signature = "(self, sample)"]
    fn reply(&self, sample: Sample) {
        task::block_on(async {
            self.q.reply(sample.s).await;
        });
    }
}

/// An entity able to reply to queries.
#[pyclass]
pub(crate) struct Queryable {
    pub(crate) undeclare_tx: Sender<bool>,
    pub(crate) loop_handle: Option<async_std::task::JoinHandle<()>>,
}

#[pymethods]
impl Queryable {
    /// Undeclare the queryable.
    fn undeclare(&mut self) {
        if let Some(handle) = self.loop_handle.take() {
            task::block_on(async {
                self.undeclare_tx.send(true).await;
                handle.await;
            });
        }
    }
}

// zenoh.net.Target (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/834 to be fixed)
//
/// The queryables that should be target of a :class:`Query`
#[pyclass]
#[derive(Clone)]
pub(crate) struct Target {
    pub(crate) t: zenoh::net::Target,
}

#[allow(non_snake_case)]
#[pymethods]
impl Target {
    #[staticmethod]
    fn BestMatching() -> Target {
        Target {
            t: zenoh::net::Target::BestMatching,
        }
    }

    #[staticmethod]
    #[text_signature = "(n)"]
    fn Complete(n: ZInt) -> Target {
        Target {
            t: zenoh::net::Target::Complete { n },
        }
    }

    #[staticmethod]
    fn All() -> Target {
        Target {
            t: zenoh::net::Target::All,
        }
    }

    #[staticmethod]
    fn None() -> Target {
        Target {
            t: zenoh::net::Target::None,
        }
    }
}

/// The queryables that should be target of a :class:`Query`.
///
/// :param kind: the kind of queryable (one constant from :class:`queryable`)
/// :type kind: int, optional
/// :param target: a characteristic of the queryable.
/// :type target: Target, optional
#[pyclass]
#[text_signature = "(kind=None, target=None)"]
#[derive(Clone)]
pub(crate) struct QueryTarget {
    pub(crate) t: zenoh::net::QueryTarget,
}

#[pymethods]
impl QueryTarget {
    #[new]
    fn new(kind: Option<ZInt>, target: Option<Target>) -> QueryTarget {
        let mut t = zenoh::net::QueryTarget::default();
        if let Some(k) = kind {
            t.kind = k;
        }
        if let Some(target) = target {
            t.target = target.t;
        }
        QueryTarget { t }
    }
}

impl Default for QueryTarget {
    fn default() -> Self {
        QueryTarget {
            t: zenoh::net::QueryTarget::default(),
        }
    }
}

// zenoh.net.QueryConsolidation (simulate the enum as a class with static methods for the cases,
// waiting for https://github.com/PyO3/pyo3/issues/834 to be fixed)
//
/// The kind of consolidation that should be applied on replies to a :meth:`Session.query`.
#[pyclass]
#[derive(Clone)]
pub(crate) struct QueryConsolidation {
    pub(crate) c: zenoh::net::QueryConsolidation,
}

#[allow(non_snake_case)]
#[pymethods]
impl QueryConsolidation {
    #[classattr]
    fn None() -> QueryConsolidation {
        QueryConsolidation {
            c: zenoh::net::QueryConsolidation::None,
        }
    }

    #[classattr]
    fn LastHop() -> QueryConsolidation {
        QueryConsolidation {
            c: zenoh::net::QueryConsolidation::LastHop,
        }
    }

    #[classattr]
    fn Incremental() -> QueryConsolidation {
        QueryConsolidation {
            c: zenoh::net::QueryConsolidation::Incremental,
        }
    }
}

impl Default for QueryConsolidation {
    fn default() -> Self {
        QueryConsolidation {
            c: zenoh::net::QueryConsolidation::default(),
        }
    }
}

/// Type received by a query callback. See :meth:`Session.query`.
#[pyclass]
#[derive(Clone)]
pub(crate) struct Reply {
    pub(crate) r: zenoh::net::Reply,
}

impl pyo3::conversion::ToPyObject for Reply {
    fn to_object(&self, py: Python) -> pyo3::PyObject {
        pyo3::IntoPy::into_py(pyo3::Py::new(py, self.clone()).unwrap(), py)
    }
}

#[pymethods]
impl Reply {
    /// The data
    ///
    /// :type: Sample
    #[getter]
    fn data(&self) -> Sample {
        Sample {
            s: self.r.data.clone(),
        }
    }

    /// The kind of reply source
    ///
    /// :type: int
    #[getter]
    fn source_kind(&self) -> ZInt {
        self.r.source_kind
    }

    /// The identifier of reply source
    ///
    /// :type: PeerId
    fn replier_id(&self) -> PeerId {
        PeerId {
            p: self.r.replier_id.clone(),
        }
    }
}
