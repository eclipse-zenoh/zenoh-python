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

use pyo3::{
    exceptions::PyTypeError,
    prelude::*,
    types::{PyBytes, PyDateTime, PyType},
};

use crate::{
    macros::{downcast_or_new, wrapper},
    utils::{IntoPyErr, IntoPyResult},
};

wrapper!(zenoh::time::TimestampId: Copy, Clone, PartialEq, PartialOrd);
downcast_or_new!(TimestampId => Vec<u8>);

#[pymethods]
impl TimestampId {
    #[new]
    fn new(bytes: Vec<u8>) -> PyResult<Self> {
        Ok(Self(bytes.as_slice().try_into().into_pyres()?))
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

    pub(crate) fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.to_le_bytes()[..self.0.size()])
    }

    fn __hash__(&self, py: Python) -> PyResult<isize> {
        self.__bytes__(py).hash()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

wrapper!(zenoh::time::Timestamp: Clone, PartialEq, PartialOrd, Hash);

#[pymethods]
impl Timestamp {
    #[new]
    fn new(
        time: Bound<PyAny>,
        #[pyo3(from_py_with = TimestampId::from_py)] id: TimestampId,
    ) -> PyResult<Self> {
        let ntp = if let Ok(time) = time.downcast::<PyDateTime>() {
            time.extract::<SystemTime>()?
                .duration_since(SystemTime::UNIX_EPOCH)
                .into_pyres()?
                .into()
        } else if let Ok(ntp) = time.extract::<NTP64>() {
            ntp.0
        } else {
            return Err(PyTypeError::new_err("expected a `datetime` or a `NTP64`"));
        };
        Ok(Self(zenoh::time::Timestamp::new(ntp, id.0)))
    }

    fn get_time(&self) -> SystemTime {
        self.0.get_time().to_system_time()
    }

    fn get_time_as_ntp64(&self) -> NTP64 {
        (*self.0.get_time()).into()
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

wrapper!(zenoh::time::NTP64: Clone, PartialEq, PartialOrd, Hash);

#[pymethods]
impl NTP64 {
    #[new]
    fn new(seconds: u64, nanoseconds: u32) -> Self {
        Self(Duration::new(seconds, nanoseconds).into())
    }

    fn as_secs_f64(&self) -> f64 {
        self.0.as_secs_f64()
    }

    fn as_secs(&self) -> u32 {
        self.0.as_secs()
    }

    fn as_nanos(&self) -> u64 {
        self.0.as_nanos()
    }

    fn subsec_nanos(&self) -> u32 {
        self.0.subsec_nanos()
    }

    fn to_datetime(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + self.0.to_duration()
    }

    fn to_string_rfc3339_lossy(&self) -> String {
        self.0.to_string_rfc3339_lossy()
    }

    #[classmethod]
    fn parse_rfc3339(_cls: &Bound<PyType>, s: &str) -> PyResult<Self> {
        zenoh::time::NTP64::parse_rfc3339(s)
            .map(Self)
            .map_err(|err| err.cause.into_pyerr())
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
