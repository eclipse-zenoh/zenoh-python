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
pub(crate) use pyo3::prelude::*;
use zenoh::{prelude::QoSBuilderTrait, sample::QoSBuilder};

use crate::{
    bytes::ZBytes,
    encoding::Encoding,
    key_expr::KeyExpr,
    macros::{build, enum_mapper, wrapper},
    publication::{CongestionControl, Priority},
    time::Timestamp,
    utils::MapInto,
};

wrapper!(zenoh::sample::QoS);

#[pymethods]
impl QoS {
    #[new]
    fn new(
        priority: Option<Priority>,
        congestion_control: Option<CongestionControl>,
        express: Option<bool>,
    ) -> Self {
        let build = build!(
            QoSBuilder::from(zenoh::sample::QoS::default()),
            priority,
            congestion_control,
            express
        );
        Self(build().into())
    }

    #[getter]
    fn priority(&self) -> Priority {
        self.0.priority().into()
    }

    #[getter]
    fn congestion_control(&self) -> CongestionControl {
        self.0.congestion_control().into()
    }

    #[getter]
    fn express(&self) -> bool {
        self.0.express()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

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
    pub(crate) fn qos(&self) -> QoS {
        (*self.0.qos()).into()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
