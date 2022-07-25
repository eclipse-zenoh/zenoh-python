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

use std::sync::Arc;

use pyo3::{prelude::*, types::PyDict};
use zenoh::prelude::SessionDeclarations;
use zenoh::subscriber::CallbackSubscriber;
use zenoh::Session;
use zenoh_core::SyncResolve;

use crate::enums::{_CongestionControl, _Priority, _Reliability, _SampleKind};
use crate::keyexpr::_KeyExpr;
use crate::value::_Sample;
use crate::{PyAnyToValue, PyExtract, ToPyErr};

#[pyclass(subclass)]
#[derive(Clone)]
pub struct _Session(Arc<Session>);

#[pymethods]
impl _Session {
    #[new]
    pub fn new(config: Option<&mut crate::config::_Config>) -> PyResult<Self> {
        let config = config.and_then(|c| c.0.take()).unwrap_or_default();
        let session = zenoh::open(config).res_sync().map_err(|e| e.to_pyerr())?;
        Ok(_Session(Arc::new(session)))
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
            match kwargs.extract_item::<bool>("local_routing") {
                Ok(local_routing) => builder = builder.local_routing(local_routing),
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
            match kwargs.extract_item::<bool>("local_routing") {
                Ok(local_routing) => builder = builder.local_routing(local_routing),
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
    pub fn declare_subscriber(
        &self,
        key_expr: &_KeyExpr,
        callback: Py<PyAny>,
        kwargs: Option<&PyDict>,
    ) -> PyResult<_Subscriber> {
        let mut builder = self
            .0
            .declare_subscriber(&key_expr.0)
            .callback(move |sample| {
                Python::with_gil(|py| {
                    callback.call1(py, (_Sample::from(sample),)).unwrap();
                })
            });
        if let Some(kwargs) = kwargs {
            match kwargs.extract_item::<bool>("best_effort") {
                Ok(true) => builder = builder.best_effort(),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
            match kwargs.extract_item::<bool>("local") {
                Ok(true) => builder = builder.local(),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
            match kwargs.extract_item::<_Reliability>("reliability") {
                Ok(reliabilty) => builder = builder.reliability(reliabilty.0),
                Err(crate::ExtractError::Other(e)) => return Err(e),
                _ => {}
            }
        }
        let subscriber = builder.res().map_err(|e| e.to_pyerr())?;
        Ok(_Subscriber(subscriber))
    }
}

#[pyclass]
pub struct _Subscriber(CallbackSubscriber<'static>);
