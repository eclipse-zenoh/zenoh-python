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

use pyo3::{prelude::*, types::PyType};

use crate::utils::{downcast_or_parse, r#enum, wrapper, IntoPyResult, MapInto};

r#enum!(zenoh::key_expr::SetIntersectionLevel: u8 {
    Disjoint,
    Intersects,
    Includes,
    Equals,
});

wrapper!(zenoh::key_expr::KeyExpr<'static>: Clone);
downcast_or_parse!(KeyExpr);

#[pymethods]
impl KeyExpr {
    #[new]
    pub(crate) fn new(s: String) -> PyResult<Self> {
        Ok(Self(s.parse().into_pyres()?))
    }

    #[classmethod]
    fn autocanonize(_cls: &Bound<PyType>, key_expr: String) -> PyResult<Self> {
        zenoh::key_expr::KeyExpr::autocanonize(key_expr)
            .into_pyres()
            .map_into()
    }

    fn intersects(&self, #[pyo3(from_py_with = "KeyExpr::from_py")] other: KeyExpr) -> bool {
        self.0.intersects(&other.0)
    }

    fn includes(&self, #[pyo3(from_py_with = "KeyExpr::from_py")] other: KeyExpr) -> bool {
        self.0.includes(&other.0)
    }

    fn relation_to(
        &self,
        #[pyo3(from_py_with = "KeyExpr::from_py")] other: KeyExpr,
    ) -> SetIntersectionLevel {
        self.0.relation_to(&other.0).into()
    }

    fn join(&self, other: String) -> PyResult<Self> {
        self.0.join(&other).into_pyres().map_into()
    }

    fn concat(&self, other: String) -> PyResult<Self> {
        self.0.concat(&other).into_pyres().map_into()
    }

    // TODO paremeters

    // fn with_parameters(&self, parameters: String) -> Selector {
    //     self.0.clone().with_owned_parameters(parameters).into()
    // }

    fn __eq__(&self, #[pyo3(from_py_with = "KeyExpr::from_py")] other: KeyExpr) -> PyResult<bool> {
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