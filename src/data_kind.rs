//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
use zenoh::prelude::SampleKind as ZSampleKind;

// zenoh.data_kind (simulate the package as a class, and consts as class attributes)
/// Constants defining the different data kinds.
#[allow(non_camel_case_types)]
#[pyclass]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SampleKind {
    pub kind: ZSampleKind,
}
impl From<ZSampleKind> for SampleKind {
    fn from(kind: ZSampleKind) -> Self {
        SampleKind { kind }
    }
}
impl From<SampleKind> for ZSampleKind {
    fn from(kind: SampleKind) -> Self {
        kind.kind
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl SampleKind {
    #[classattr]
    pub fn PUT() -> Self {
        ZSampleKind::Put.into()
    }

    #[classattr]
    pub fn PATCH() -> Self {
        ZSampleKind::Patch.into()
    }

    #[classattr]
    pub fn DELETE() -> Self {
        ZSampleKind::Delete.into()
    }

    #[classattr]
    pub fn DEFAULT() -> Self {
        ZSampleKind::default().into()
    }
}

#[pyproto]
impl PyObjectProtocol for SampleKind {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

impl std::fmt::Display for SampleKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}
