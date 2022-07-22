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
use std::convert::TryInto;
use zenoh::prelude::KeyExpr;

use crate::ToPyErr;

#[pyclass]
#[derive(Clone)]
pub struct _KeyExpr(KeyExpr<'static>);

#[pymethods]
impl _KeyExpr {
    #[new]
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

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
