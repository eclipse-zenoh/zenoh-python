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
use pyo3::prelude::*;

use crate::{
    key_expr::KeyExpr,
    utils::{downcast_or_parse, wrapper, IntoPyResult},
};

wrapper!(zenoh::selector::Selector<'static>: Clone);
downcast_or_parse!(Selector);

#[pymethods]
impl Selector {
    #[new]
    pub(crate) fn new(s: String) -> PyResult<Self> {
        Ok(Self(s.parse().into_pyres()?))
    }

    #[getter]
    fn key_expr(&self) -> KeyExpr {
        self.0.key_expr().clone().into_owned().into()
    }

    // TODO parameters

    // TODO time_range

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
}
