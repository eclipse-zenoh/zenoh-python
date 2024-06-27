use pyo3::prelude::*;

use crate::macros::enum_mapper;

enum_mapper!(zenoh::qos::Priority: u8 {
    RealTime = 1,
    InteractiveHigh = 2,
    InteractiveLow = 3,
    DataHigh = 4,
    Data = 5,
    DataLow = 6,
    Background = 7,
});

#[pymethods]
impl Priority {
    #[classattr]
    const DEFAULT: Self = Self::Data;
    #[classattr]
    const MIN: Self = Self::Background;
    #[classattr]
    const MAX: Self = Self::RealTime;
    #[classattr]
    const NUM: usize = 1 + Self::MIN as usize - Self::MAX as usize;
}

enum_mapper!(zenoh::qos::CongestionControl: u8 {
    Drop = 0,
    Block = 1,
});

#[pymethods]
impl CongestionControl {
    #[classattr]
    const DEFAULT: Self = Self::Drop;
}
