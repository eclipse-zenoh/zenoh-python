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
use zenoh::Session;
use zenoh_core::SyncResolve;

use crate::enums::{_CongestionControl, _Encoding, _Priority, _SampleKind};
use crate::{PyAnyToValue, ToPyErr};

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
            if let Some(arg) = kwargs.get_item("encoding") {
                builder = builder.encoding(arg.extract::<_Encoding>()?.0);
            }
            if let Some(arg) = kwargs.get_item("kind") {
                builder = builder.kind(arg.extract::<_SampleKind>()?.0);
            }
            if let Some(arg) = kwargs.get_item("congestion_control") {
                builder = builder.congestion_control(arg.extract::<_CongestionControl>()?.0);
            }
            if let Some(arg) = kwargs.get_item("priority") {
                builder = builder.priority(arg.extract::<_Priority>()?.0);
            }
            if let Some(arg) = kwargs.get_item("local_routing") {
                builder = builder.local_routing(arg.extract::<bool>()?);
            }
        }
        builder.res_sync().map_err(|e| e.to_pyerr())
    }
}
