//
// Copyright (c) 2026 ZettaScale Technology
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
use pyo3::{prelude::*, types::PyBytes};

use crate::{
    macros::{enum_mapper, wrapper},
    time::Timestamp,
};

enum_mapper!(zenoh::timestamp_stack::InterceptionPoint: u8 {
    Send,
    Route,
    Receive,
});

wrapper!(zenoh::timestamp_stack::TimestampInstrumentation: Clone, Copy, PartialEq, Eq);

#[pymethods]
impl TimestampInstrumentation {
    fn is_instrumented(&self, point: InterceptionPoint) -> bool {
        self.0.is_instrumented(point.into())
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

wrapper!(zenoh::timestamp_stack::TimestampInstrumentationBuilder: Clone, Copy);

#[pymethods]
impl TimestampInstrumentationBuilder {
    #[new]
    fn new() -> Self {
        Self(zenoh::timestamp_stack::TimestampInstrumentationBuilder::new())
    }

    fn set_send(&self, enabled: bool) -> Self {
        Self(self.0.set_send(enabled))
    }

    fn set_route(&self, enabled: bool) -> Self {
        Self(self.0.set_route(enabled))
    }

    fn set_receive(&self, enabled: bool) -> Self {
        Self(self.0.set_receive(enabled))
    }

    fn build(&self) -> PyResult<TimestampInstrumentation> {
        self.0
            .build()
            .map(TimestampInstrumentation)
            .map_err(|e| crate::ZError::new_err(e.to_string()))
    }
}

wrapper!(zenoh::timestamp_stack::TimestampStackRecord: Clone);

#[pymethods]
impl TimestampStackRecord {
    fn point(&self) -> InterceptionPoint {
        self.0.point().into()
    }

    fn is_custom(&self) -> bool {
        self.0.is_custom()
    }

    fn timestamp<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        match self.0.timestamp() {
            zenoh::timestamp_stack::InstrumentationTimestamp::UHLC(ts) => {
                Ok(Timestamp::from(*ts).into_pyobject(py)?.into_any())
            }
            zenoh::timestamp_stack::InstrumentationTimestamp::Custom(bytes) => {
                Ok(PyBytes::new(py, bytes).into_pyobject(py)?.into_any())
            }
        }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

wrapper!(zenoh::timestamp_stack::TimestampStack: Clone);

#[pymethods]
impl TimestampStack {
    #[getter]
    fn instrumentation(&self) -> TimestampInstrumentation {
        self.0.instrumentation().into()
    }

    #[getter]
    fn records(&self) -> Vec<TimestampStackRecord> {
        self.0.records().iter().cloned().map(Into::into).collect()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
