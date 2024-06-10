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
    bytes::ZBytes,
    encoding::Encoding,
    key_expr::KeyExpr,
    macros::{enum_mapper, wrapper},
    publisher::{CongestionControl, Priority},
    time::Timestamp,
    utils::MapInto,
};

enum_mapper!(zenoh::sample::SampleKind: u8 {
    Put = 0,
    Delete = 1,
});

wrapper!(zenoh::sample::Sample);

#[pymethods]
impl Sample {
    #[getter]
    pub(crate) fn key_expr(&self) -> KeyExpr {
        self.0.key_expr().clone().into()
    }

    #[getter]
    pub(crate) fn payload(&self) -> ZBytes {
        self.0.payload().clone().into()
    }

    #[getter]
    pub(crate) fn kind(&self) -> SampleKind {
        self.0.kind().into()
    }

    #[getter]
    pub(crate) fn encoding(&self) -> Encoding {
        self.0.encoding().clone().into()
    }

    #[getter]
    pub(crate) fn timestamp(&self) -> Option<Timestamp> {
        self.0.timestamp().cloned().map_into()
    }

    #[getter]
    pub fn congestion_control(&self) -> CongestionControl {
        self.0.congestion_control().into()
    }

    #[getter]
    pub fn priority(&self) -> Priority {
        self.0.priority().into()
    }

    #[getter]
    pub fn express(&self) -> bool {
        self.0.express()
    }

    #[getter]
    pub fn attachment(&self) -> Option<ZBytes> {
        self.0.attachment().cloned().map_into()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
