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
use std::{collections::HashMap, sync::Arc};

use pyo3::prelude::*;
use zenoh::{
    queryable::{Query, Queryable},
    selector::Parameters,
};
use zenoh_core::SyncResolve;

use crate::{
    keyexpr::{_KeyExpr, _Selector},
    value::{_Sample, _Value},
    ToPyErr,
};

#[pyclass(subclass)]
#[derive(Clone)]
pub struct _Query(pub(crate) Arc<Query>);
#[pymethods]
impl _Query {
    #[new]
    pub fn pynew(this: Self) -> Self {
        this
    }
    #[getter]
    pub fn key_expr(&self) -> _KeyExpr {
        _KeyExpr(self.0.key_expr().clone())
    }
    #[getter]
    pub fn parameters(&self) -> &str {
        self.0.parameters()
    }
    pub fn decode_parameters(&self) -> PyResult<HashMap<String, String>> {
        let mut res = HashMap::new();
        for (k, v) in self.0.parameters().decode() {
            let k = k.into_owned();
            match res.entry(k) {
                std::collections::hash_map::Entry::Occupied(e) => {
                    return Err(zenoh_core::zerror!(
                        "Detected duplicate key {} in value selector {}",
                        e.key(),
                        self.0.parameters()
                    )
                    .to_pyerr())
                }
                std::collections::hash_map::Entry::Vacant(e) => {
                    e.insert(v.into_owned());
                }
            }
        }
        Ok(res)
    }
    #[getter]
    pub fn selector(&self) -> _Selector {
        _Selector(self.0.selector().clone().into_owned())
    }
    #[getter]
    pub fn value(&self) -> Option<_Value> {
        self.0.value().map(|v| v.clone().into())
    }
    pub fn reply(&self, sample: _Sample) -> PyResult<()> {
        self.0
            .reply(Ok(sample.into()))
            .res_sync()
            .map_err(|e| e.to_pyerr())
    }
}
impl From<Query> for _Query {
    fn from(q: Query) -> Self {
        Self(Arc::new(q))
    }
}

#[pyclass(subclass)]
pub struct _Queryable(pub(crate) Queryable<'static, ()>);
