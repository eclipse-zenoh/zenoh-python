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

use pyo3::prelude::*;
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    convert::{TryFrom, TryInto},
};
use zenoh::prelude::{sync::SyncResolve, KeyExpr, Selector};

use crate::{session::_Session, ToPyErr};

#[pyclass(subclass)]
#[derive(Clone)]
pub struct _KeyExpr(pub(crate) KeyExpr<'static>);

#[pymethods]
impl _KeyExpr {
    #[new]
    pub fn pynew(this: Self) -> Self {
        this
    }
    #[staticmethod]
    pub fn new(expr: String) -> PyResult<Self> {
        match expr.try_into() {
            Ok(k) => Ok(Self(k)),
            Err(e) => Err(e.to_pyerr()),
        }
    }
    #[staticmethod]
    pub fn autocanonize(expr: String) -> PyResult<Self> {
        match KeyExpr::autocanonize(expr) {
            Ok(k) => Ok(Self(k)),
            Err(e) => Err(e.to_pyerr()),
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.0.intersects(&other.0)
    }

    pub fn includes(&self, other: &Self) -> bool {
        self.0.includes(&other.0)
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    pub fn undeclare(&self, session: &_Session) -> PyResult<()> {
        session
            .0
            .undeclare(self.0.clone())
            .res_sync()
            .map_err(|e| e.to_pyerr())
    }

    pub fn __str__(&self) -> &str {
        self.0.as_str()
    }

    pub fn __hash__(&self) -> isize {
        use std::hash::*;
        let mut hasher: DefaultHasher = BuildHasherDefault::default().build_hasher();
        self.0.hash(&mut hasher);
        hasher.finish() as isize
    }

    pub fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[pyclass(subclass)]
#[derive(Clone)]
pub struct _Selector(pub(crate) Selector<'static>);
#[pymethods]
impl _Selector {
    #[new]
    pub fn pynew(this: Self) -> Self {
        this
    }
    #[staticmethod]
    pub fn new(expr: String) -> PyResult<Self> {
        match Selector::try_from(expr) {
            Ok(o) => Ok(_Selector(o)),
            Err(e) => Err(e.to_pyerr()),
        }
    }
    #[getter]
    pub fn key_expr(&self) -> _KeyExpr {
        _KeyExpr(self.0.key_expr.clone())
    }
    #[getter]
    pub fn get_parameters(&self) -> &str {
        self.0.parameters()
    }
    #[setter]
    pub fn set_parameters(&mut self, parameters: String) {
        self.0.set_parameters(parameters)
    }
    pub fn decode_parameters(&self) -> PyResult<HashMap<String, String>> {
        self.0.parameters_map().map_err(|e| e.to_pyerr())
    }
    pub fn __str__(&self) -> String {
        self.0.to_string()
    }
}
