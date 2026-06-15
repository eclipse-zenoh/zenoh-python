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
use std::sync::Arc;

use pyo3::{prelude::*, types::PyBytes};

use crate::{
    config::{WhatAmI, ZenohId},
    macros::{enum_mapper, wrapper},
    time::Timestamp,
};

enum_mapper!(zenoh::timestamp_stack::InterceptionPoint: u8 {
    Send,
    Route,
    Receive,
});

#[pyclass]
pub(crate) struct TsStackContext(pub(crate) zenoh::timestamp_stack::TsStackContext);

#[pymethods]
impl TsStackContext {
    #[getter]
    fn zid(&self) -> ZenohId {
        ZenohId(self.0.zid)
    }

    #[getter]
    fn whatami(&self) -> WhatAmI {
        self.0.whatami.into()
    }

    #[getter]
    fn interception_point(&self) -> InterceptionPoint {
        self.0.interception_point.into()
    }

    fn __repr__(&self) -> String {
        format!(
            "TsStackContext(zid={}, whatami={:?}, interception_point={:?})",
            self.0.zid, self.0.whatami, self.0.interception_point
        )
    }
}

fn log_timestamp_callback_error(py: Python, err: PyErr) {
    if let Ok(logging) = py.import("logging") {
        if let Ok(logger) = logging.call_method1("getLogger", ("zenoh",)) {
            let _ = logger.call_method1("error", (format!("timestamp callback error: {err}"),));
        }
    }
}

pub(crate) fn create_timestamp_callback(
    callback: Py<PyAny>,
) -> zenoh::timestamp_stack::GetTimestampCallback {
    Arc::new(
        move |ctx: zenoh::timestamp_stack::TsStackContext| -> Vec<u8> {
            Python::with_gil(|py| {
                let py_ctx = match Py::new(py, TsStackContext(ctx)) {
                    Ok(ctx) => ctx,
                    Err(e) => {
                        log_timestamp_callback_error(py, e);
                        return Vec::new();
                    }
                };
                match callback.call1(py, (py_ctx,)) {
                    Ok(result) => result.extract::<Vec<u8>>(py).unwrap_or_else(|e| {
                        log_timestamp_callback_error(py, e);
                        Vec::new()
                    }),
                    Err(e) => {
                        log_timestamp_callback_error(py, e);
                        Vec::new()
                    }
                }
            })
        },
    )
}

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
