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

use std::convert::TryInto;
use std::sync::Arc;

use pyo3::{prelude::*, types::PyDict};
use zenoh::config::whatami::{WhatAmI, WhatAmIMatcher};
use zenoh::prelude::SessionDeclarations;
use zenoh::publication::Publisher;
use zenoh::scouting::Scout;
use zenoh::subscriber::{PullSubscriber, Subscriber};
use zenoh::Session;
use zenoh_core::SyncResolve;

use crate::closures::PyClosure;
use crate::config::{PyConfig, _Config};
use crate::enums::{
    _CongestionControl, _Priority, _QueryConsolidation, _QueryTarget, _Reliability, _SampleKind,
};
use crate::keyexpr::{_KeyExpr, _Selector};
use crate::queryable::{_Query, _Queryable};
use crate::value::{_Hello, _Reply, _Sample, _Value, _ZenohId};
use crate::{PyAnyToValue, PyExtract, ToPyErr};

#[pyclass(subclass)]
#[derive(Clone)]
pub struct _Session(pub(crate) Arc<Session>);

#[pymethods]
impl _Session {
    #[new]
    pub fn new(mut config: Option<&mut crate::config::_Config>) -> PyResult<Self> {
        let c = match &mut config {
            Some(c) => c.0.take().unwrap_or_default(),
            None => Default::default(),
        };
        let session = zenoh::open(c).res_sync().map_err(|e| e.to_pyerr())?;
        if let Some(config) = config {
            *config = _Config(PyConfig::Notifier(session.config().clone()))
        }
        Ok(_Session(Arc::new(session)))
    }
    pub fn config(&self) -> _Config {
        _Config(PyConfig::Notifier(self.0.config().clone()))
    }
    #[args(kwargs = "**")]
    pub fn put(
        &self,
        key_expr: &crate::keyexpr::_KeyExpr,
        value: &PyAny,
        kwargs: Option<&PyDict>,
    ) -> PyResult<()> {
        let s = &self.0;
        let k = &key_expr.0;
        let v = value.to_value()?;
        let mut builder = s.put(k, v);
        if let Some(kwargs) = kwargs {
            match kwargs.extract_item::<_SampleKind>("kind") {
                Ok(kind) => builder = builder.kind(kind.0),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
            match kwargs.extract_item::<_CongestionControl>("congestion_control") {
                Ok(congestion_control) => {
                    builder = builder.congestion_control(congestion_control.0)
                }
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
            match kwargs.extract_item::<_Priority>("priority") {
                Ok(priority) => builder = builder.priority(priority.0),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
        }
        builder.res_sync().map_err(|e| e.to_pyerr())
    }
    #[args(kwargs = "**")]
    pub fn delete(
        &self,
        key_expr: &crate::keyexpr::_KeyExpr,
        kwargs: Option<&PyDict>,
    ) -> PyResult<()> {
        let s = &self.0;
        let k = &key_expr.0;
        let mut builder = s.delete(k);
        if let Some(kwargs) = kwargs {
            match kwargs.extract_item::<_SampleKind>("kind") {
                Ok(kind) => builder = builder.kind(kind.0),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
            match kwargs.extract_item::<_CongestionControl>("congestion_control") {
                Ok(congestion_control) => {
                    builder = builder.congestion_control(congestion_control.0)
                }
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
            match kwargs.extract_item::<_Priority>("priority") {
                Ok(priority) => builder = builder.priority(priority.0),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
        }
        builder.res_sync().map_err(|e| e.to_pyerr())
    }

    #[args(kwargs = "**")]
    pub fn get(
        &self,
        selector: &_Selector,
        callback: &PyAny,
        kwargs: Option<&PyDict>,
    ) -> PyResult<()> {
        let callback: PyClosure<(_Reply,)> = <_ as TryInto<_>>::try_into(callback)?;
        let mut builder = self.0.get(&selector.0).with(callback);
        if let Some(kwargs) = kwargs {
            match kwargs.extract_item::<_QueryConsolidation>("consolidation") {
                Ok(_QueryConsolidation(Some(value))) => builder = builder.consolidation(value),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
            match kwargs.extract_item::<_QueryTarget>("target") {
                Ok(value) => builder = builder.target(value.0),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
            match kwargs.extract_item::<_Value>("value") {
                Ok(value) => builder = builder.with_value(value),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
        }
        builder.res_sync().map_err(|e| e.to_pyerr())
    }

    pub fn declare_keyexpr(&self, key_expr: &_KeyExpr) -> PyResult<_KeyExpr> {
        match self.0.declare_keyexpr(&key_expr.0).res_sync() {
            Ok(k) => Ok(_KeyExpr(k.into_owned())),
            Err(e) => Err(e.to_pyerr()),
        }
    }

    #[args(kwargs = "**")]
    pub fn declare_queryable(
        &self,
        key_expr: _KeyExpr,
        callback: &PyAny,
        kwargs: Option<&PyDict>,
    ) -> PyResult<_Queryable> {
        let callback: PyClosure<(_Query,)> = <_ as TryInto<_>>::try_into(callback)?;
        let mut builder = self.0.declare_queryable(key_expr.0).with(callback);
        if let Some(kwargs) = kwargs {
            match kwargs.extract_item::<bool>("complete") {
                Ok(value) => builder = builder.complete(value),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
        }
        match builder.res_sync() {
            Ok(o) => Ok(_Queryable(o)),
            Err(e) => Err(e.to_pyerr()),
        }
    }

    #[args(kwargs = "**")]
    pub fn declare_publisher(
        &self,
        key_expr: _KeyExpr,
        kwargs: Option<&PyDict>,
    ) -> PyResult<_Publisher> {
        let mut builder = self.0.declare_publisher(key_expr.0);
        if let Some(kwargs) = kwargs {
            match kwargs.extract_item::<_Priority>("priority") {
                Ok(value) => builder = builder.priority(value.0),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
            match kwargs.extract_item::<_CongestionControl>("congestion_control") {
                Ok(value) => builder = builder.congestion_control(value.0),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
        }
        match builder.res_sync() {
            Ok(o) => Ok(_Publisher(o)),
            Err(e) => Err(e.to_pyerr()),
        }
    }

    #[args(kwargs = "**")]
    pub fn declare_subscriber(
        &self,
        key_expr: &_KeyExpr,
        callback: &PyAny,
        kwargs: Option<&PyDict>,
    ) -> PyResult<_Subscriber> {
        let callback: PyClosure<(_Sample,)> = <_ as TryInto<_>>::try_into(callback)?;
        let mut builder = self.0.declare_subscriber(&key_expr.0).with(callback);
        if let Some(kwargs) = kwargs {
            match kwargs.extract_item::<_Reliability>("reliability") {
                Ok(reliabilty) => builder = builder.reliability(reliabilty.0),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
        }
        let subscriber = builder.res().map_err(|e| e.to_pyerr())?;
        Ok(_Subscriber(subscriber))
    }

    #[args(kwargs = "**")]
    pub fn declare_pull_subscriber(
        &self,
        key_expr: &_KeyExpr,
        callback: &PyAny,
        kwargs: Option<&PyDict>,
    ) -> PyResult<_PullSubscriber> {
        let callback: PyClosure<(_Sample,)> = <_ as TryInto<_>>::try_into(callback)?;
        let mut builder = self
            .0
            .declare_subscriber(&key_expr.0)
            .pull_mode()
            .with(callback);
        if let Some(kwargs) = kwargs {
            match kwargs.extract_item::<_Reliability>("reliability") {
                Ok(reliabilty) => builder = builder.reliability(reliabilty.0),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
        }
        let subscriber = builder.res().map_err(|e| e.to_pyerr())?;
        Ok(_PullSubscriber(subscriber))
    }

    pub fn zid(&self) -> _ZenohId {
        _ZenohId(self.0.zid())
    }
    pub fn routers_zid(&self) -> Vec<_ZenohId> {
        self.0
            .info()
            .routers_zid()
            .res_sync()
            .map(_ZenohId)
            .collect()
    }
    pub fn peers_zid(&self) -> Vec<_ZenohId> {
        self.0.info().peers_zid().res_sync().map(_ZenohId).collect()
    }
}

#[pyclass(subclass)]
#[derive(Clone)]
pub struct _Publisher(Publisher<'static>);
#[pymethods]
impl _Publisher {
    #[new]
    pub fn pynew(this: Self) -> Self {
        this
    }
    #[getter]
    pub fn key_expr(&self) -> _KeyExpr {
        _KeyExpr(self.0.key_expr().clone())
    }
    pub fn put(&self, value: _Value) -> PyResult<()> {
        self.0.put(value).res_sync().map_err(|e| e.to_pyerr())
    }
    pub fn delete(&self) -> PyResult<()> {
        self.0.delete().res_sync().map_err(|e| e.to_pyerr())
    }
}

#[pyclass(subclass)]
pub struct _Subscriber(Subscriber<'static, ()>);

#[pyclass(subclass)]
pub struct _PullSubscriber(PullSubscriber<'static, ()>);
#[pymethods]
impl _PullSubscriber {
    fn pull(&self) -> PyResult<()> {
        self.0.pull().res_sync().map_err(|e| e.to_pyerr())
    }
}

#[pyclass(subclass)]
pub struct _Scout(Scout<()>);

#[pyfunction]
pub fn scout(callback: &PyAny, config: Option<&_Config>, what: Option<&str>) -> PyResult<_Scout> {
    let callback: PyClosure<(_Hello,)> = <_ as TryInto<_>>::try_into(callback)?;
    let what: WhatAmIMatcher = match what {
        None => WhatAmI::Client | WhatAmI::Peer | WhatAmI::Router,
        Some(s) => match s.parse() {
            Ok(w) => w,
            Err(_) => return Err(zenoh_core::zerror!("Couldn't parse `{}` into a WhatAmiMatcher: must be a `|`-separated list of `peer`, `client` or `router`", s).to_pyerr())
        },
    };
    let config = config.and_then(|c| c.0.clone().take()).unwrap_or_default();
    let scout = zenoh::scout(what, config).with(callback).res_sync();
    match scout {
        Ok(scout) => Ok(_Scout(scout)),
        Err(e) => Err(e.to_pyerr()),
    }
}
