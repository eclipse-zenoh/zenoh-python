//
// Copyright (c) 2024 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//
use std::collections::hash_map::DefaultHasher;

use pyo3::prelude::*;

use crate::utils::{r#enum, try_downcast_or_parse, wrapper, IntoPyResult, MapInto};

r#enum!(zenoh::key_expr::SetIntersectionLevel: u8 {
    Disjoint,
    Intersects,
    Includes,
    Equals,
});

wrapper!(zenoh::key_expr::KeyExpr<'static>: Clone);

enum KeyExprOrString {
    KeyExpr(KeyExpr),
    String(String),
}

impl KeyExprOrString {
    fn new(obj: &Bound<PyAny>) -> PyResult<Self> {
        Ok(match KeyExpr::extract_bound(obj) {
            Ok(obj) => Self::KeyExpr(obj),
            _ => Self::String(String::extract_bound(obj)?),
        })
    }
}

impl AsRef<str> for KeyExprOrString {
    fn as_ref(&self) -> &str {
        match self {
            Self::KeyExpr(s) => &s.0,
            Self::String(s) => s,
        }
    }
}

#[pymethods]
impl KeyExpr {
    #[new]
    pub(crate) fn new(key_expr: &Bound<PyAny>) -> PyResult<Self> {
        try_downcast_or_parse!(key_expr)
    }

    #[staticmethod]
    fn autocanonize(key_expr: String) -> PyResult<Self> {
        zenoh::key_expr::KeyExpr::autocanonize(key_expr)
            .into_pyres()
            .map_into()
    }

    fn intersects(&self, #[pyo3(from_py_with = "KeyExpr::new")] other: KeyExpr) -> bool {
        self.0.intersects(&other.0)
    }

    fn includes(&self, #[pyo3(from_py_with = "KeyExpr::new")] other: KeyExpr) -> bool {
        self.0.includes(&other.0)
    }

    fn relation_to(
        &self,
        #[pyo3(from_py_with = "KeyExpr::new")] other: KeyExpr,
    ) -> SetIntersectionLevel {
        self.0.relation_to(&other.0).into()
    }

    fn join(
        &self,
        #[pyo3(from_py_with = "KeyExprOrString::new")] other: KeyExprOrString,
    ) -> PyResult<Self> {
        self.0.join(&other).into_pyres().map_into()
    }

    fn concat(
        &self,
        #[pyo3(from_py_with = "KeyExprOrString::new")] other: KeyExprOrString,
    ) -> PyResult<Self> {
        self.0.concat(&other).into_pyres().map_into()
    }

    // TODO paremeters

    // fn with_parameters(&self, parameters: String) -> Selector {
    //     self.0.clone().with_owned_parameters(parameters).into()
    // }

    fn __eq__(&self, #[pyo3(from_py_with = "KeyExpr::new")] other: KeyExpr) -> PyResult<bool> {
        Ok(self.0 == other.0)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __hash__(&self) -> isize {
        use std::hash::*;
        let mut hasher: DefaultHasher = BuildHasherDefault::default().build_hasher();
        self.0.hash(&mut hasher);
        hasher.finish() as isize
    }
}
