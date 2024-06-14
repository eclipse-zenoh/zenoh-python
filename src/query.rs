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
use zenoh::prelude::*;

use crate::{
    bytes::ZBytes,
    config::ZenohId,
    encoding::Encoding,
    key_expr::KeyExpr,
    macros::{build, enum_mapper, wrapper},
    publisher::{CongestionControl, Priority},
    selector::{Parameters, Selector},
    utils::{wait, IntoPython, MapInto},
};

enum_mapper!(zenoh::query::QueryTarget: u8 {
    BestMatching,
    All,
    AllComplete,
});

#[pymethods]
impl QueryTarget {
    #[classattr]
    const DEFAULT: Self = Self::BestMatching;
}

enum_mapper!(zenoh::query::ConsolidationMode: u8 {
    Auto,
    None,
    Monotonic,
    Latest,
});

#[pymethods]
impl ConsolidationMode {
    #[classattr]
    const DEFAULT: Self = Self::Auto;
}

wrapper!(zenoh::query::Query: Clone);

#[pymethods]
impl Query {
    #[getter]
    fn selector(&self) -> Selector {
        self.0.selector().into_owned().into()
    }

    #[getter]
    fn key_expr(&self) -> KeyExpr {
        self.0.key_expr().clone().into_owned().into()
    }

    #[getter]
    fn parameters(&self) -> Parameters {
        self.0.parameters().clone().into_owned().into()
    }

    #[getter]
    fn payload(&self) -> Option<ZBytes> {
        self.0.payload().cloned().map_into()
    }

    #[getter]
    fn encoding(&self) -> Option<Encoding> {
        self.0.encoding().cloned().map_into()
    }

    #[getter]
    fn attachment(&self) -> Option<ZBytes> {
        self.0.attachment().cloned().map_into()
    }

    // TODO timestamp
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (key_expr, payload, *, encoding = None, congestion_control = None, priority = None, express = None, attachment = None))]
    fn reply(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        #[pyo3(from_py_with = "ZBytes::from_py")] payload: ZBytes,
        #[pyo3(from_py_with = "Encoding::from_py_opt")] encoding: Option<Encoding>,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
        #[pyo3(from_py_with = "ZBytes::from_py_opt")] attachment: Option<ZBytes>,
    ) -> PyResult<()> {
        let build = build!(
            self.0.reply(key_expr, payload),
            encoding,
            congestion_control,
            priority,
            express,
            attachment,
        );
        wait(py, build)
    }
    #[pyo3(signature = (payload, *, encoding = None))]
    fn reply_err(
        &self,
        py: Python,
        #[pyo3(from_py_with = "ZBytes::from_py")] payload: ZBytes,
        #[pyo3(from_py_with = "Encoding::from_py_opt")] encoding: Option<Encoding>,
    ) -> PyResult<()> {
        let build = build!(self.0.reply_err(payload), encoding);
        wait(py, build)
    }

    #[pyo3(signature = (key_expr, *, congestion_control = None, priority = None, express = None, attachment = None))]
    fn reply_del(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
        #[pyo3(from_py_with = "ZBytes::from_py_opt")] attachment: Option<ZBytes>,
    ) -> PyResult<()> {
        let build = build!(
            self.0.reply_del(key_expr),
            congestion_control,
            priority,
            express,
            attachment,
        );
        wait(py, build)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
}

wrapper!(zenoh::query::Reply);

#[pymethods]
impl Reply {
    #[getter]
    fn result(&self, py: Python) -> PyObject {
        match self.0.result() {
            Ok(sample) => sample.clone().into_pyobject(py),
            Err(value) => value.clone().into_pyobject(py),
        }
    }

    #[getter]
    fn ok(&self, py: Python) -> PyObject {
        match self.0.result() {
            Ok(sample) => sample.clone().into_pyobject(py),
            _ => py.None(),
        }
    }

    #[getter]
    fn err(&self, py: Python) -> PyObject {
        match self.0.result() {
            Err(value) => value.clone().into_pyobject(py),
            _ => py.None(),
        }
    }

    #[getter]
    fn replier_id(&self) -> Option<ZenohId> {
        self.0.replier_id().map_into()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }
}

wrapper!(zenoh::query::ReplyError: Clone);

#[pymethods]
impl ReplyError {
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
