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
use std::{
    hash::{Hash, Hasher},
    time::{Duration, SystemTime},
};

use pyo3::{prelude::*, types::PyType};

use crate::{macros::wrapper, utils::IntoPyResult};

wrapper!(zenoh::time::TimestampId: Copy, Clone, PartialEq, PartialOrd);

#[pymethods]
impl TimestampId {
    fn __richcmp__(&self, other: &Self, op: pyo3::pyclass::CompareOp) -> bool {
        match op {
            pyo3::pyclass::CompareOp::Lt => self < other,
            pyo3::pyclass::CompareOp::Le => self <= other,
            pyo3::pyclass::CompareOp::Eq => self == other,
            pyo3::pyclass::CompareOp::Ne => self != other,
            pyo3::pyclass::CompareOp::Gt => self > other,
            pyo3::pyclass::CompareOp::Ge => self >= other,
        }
    }

    fn __bytes__(&self) -> [u8; zenoh::time::TimestampId::MAX_SIZE] {
        self.0.to_le_bytes()
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        self.__bytes__().into_py(py).bind(py).hash()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
}

wrapper!(zenoh::time::Timestamp: Clone, PartialEq, PartialOrd, Hash);

#[pymethods]
impl Timestamp {
    fn get_time(&self) -> SystemTime {
        self.0.get_time().to_system_time()
    }

    fn get_id(&self) -> TimestampId {
        (*self.0.get_id()).into()
    }

    fn get_diff_duration(&self, other: Timestamp) -> Duration {
        self.0.get_diff_duration(&other.0)
    }

    fn to_string_rfc3339_lossy(&self) -> String {
        self.0.to_string_rfc3339_lossy()
    }

    #[classmethod]
    fn parse_rfc3339(_cls: &Bound<PyType>, s: &str) -> PyResult<Self> {
        Ok(Self(
            zenoh::time::Timestamp::parse_rfc3339(s)
                .map_err(|err| err.cause)
                .into_pyres()?,
        ))
    }

    fn __richcmp__(&self, other: &Self, op: pyo3::pyclass::CompareOp) -> bool {
        match op {
            pyo3::pyclass::CompareOp::Lt => self < other,
            pyo3::pyclass::CompareOp::Le => self <= other,
            pyo3::pyclass::CompareOp::Eq => self == other,
            pyo3::pyclass::CompareOp::Ne => self != other,
            pyo3::pyclass::CompareOp::Gt => self > other,
            pyo3::pyclass::CompareOp::Ge => self >= other,
        }
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
}
