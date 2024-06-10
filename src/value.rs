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
use pyo3::{prelude::*, types::PyType};

use crate::{
    bytes::ZBytes,
    encoding::Encoding,
    macros::{downcast_or_new, wrapper},
};

wrapper!(zenoh::value::Value: Clone);
downcast_or_new!(Value => ZBytes, Encoding::default());

#[pymethods]
impl Value {
    #[new]
    pub(crate) fn new(
        #[pyo3(from_py_with = "ZBytes::from_py")] payload: ZBytes,
        #[pyo3(from_py_with = "Encoding::from_py")] encoding: Encoding,
    ) -> PyResult<Self> {
        Ok(Self(zenoh::value::Value::new(payload.0, encoding.0)))
    }

    #[classmethod]
    fn empty(_cls: &Bound<PyType>) -> Self {
        Self(zenoh::value::Value::empty())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[getter]
    fn payload(&self) -> ZBytes {
        self.0.payload().clone().into()
    }

    #[getter]
    fn encoding(&self) -> Encoding {
        self.0.encoding().clone().into()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}
