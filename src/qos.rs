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

enum_mapper!(zenoh::qos::Reliability: u8 {
    BestEffort,
    Reliable
});

#[pymethods]
impl Reliability {
    #[classattr]
    const DEFAULT: Self = Self::BestEffort;
}
