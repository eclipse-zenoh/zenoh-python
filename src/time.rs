use std::time::SystemTime;

use pyo3::prelude::*;

use crate::utils::wrapper;

wrapper!(zenoh::time::Timestamp: Clone, PartialEq, PartialOrd);

#[pymethods]
impl Timestamp {
    fn get_time(&self) -> SystemTime {
        self.0.get_time().to_system_time()
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
}
