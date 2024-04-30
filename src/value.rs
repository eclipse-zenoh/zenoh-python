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
use pyo3::{
    prelude::*,
    types::{PyBytes, PyType},
};

use crate::{
    encoding::Encoding,
    payload::{from_payload, into_payload, payload_to_bytes},
    utils::wrapper,
};

wrapper!(zenoh::value::Value: Clone);

#[pymethods]
impl Value {
    #[new]
    pub(crate) fn new(payload: &Bound<PyAny>, encoding: Option<&Bound<PyAny>>) -> PyResult<Self> {
        Ok(Self(zenoh::value::Value::new(
            into_payload(payload)?,
            encoding.map(Encoding::from_py).transpose()?.unwrap().0,
        )))
    }

    #[getter]
    fn payload<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        payload_to_bytes(py, self.0.payload())
    }

    fn deserialize_payload(&self, r#type: &Bound<PyType>) -> PyResult<PyObject> {
        from_payload(r#type, self.0.payload())
    }

    #[getter]
    fn encoding(&self) -> Encoding {
        self.0.encoding().clone().into()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
