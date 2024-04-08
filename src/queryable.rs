use pyo3::{
    prelude::*,
    types::{PyBytes, PyDict, PyIterator, PyTuple, PyType},
};
use zenoh::{
    payload::Payload,
    prelude::{QoSBuilderTrait, ValueBuilderTrait},
};

use crate::{
    encoding::Encoding,
    handlers::HandlerImpl,
    key_expr::KeyExpr,
    payload::{from_payload, into_payload, payload_to_bytes},
    publication::{CongestionControl, Priority},
    selector::Selector,
    utils::{allow_threads, build, generic, opt_wrapper, wrapper, MapInto, PySyncResolve},
    value::Value,
};

wrapper!(zenoh::queryable::Query: Clone);

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

    // #[getter]
    // fn parameters(&self) -> &str {
    //     self.0.parameters()
    // }

    #[getter]
    fn value(&self) -> Option<Value> {
        self.0.value().cloned().map_into()
    }

    #[getter]
    fn payload<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.0.payload().map(|p| payload_to_bytes(py, p))
    }

    fn payload_as(&self, r#type: &Bound<PyType>) -> PyResult<Option<PyObject>> {
        self.0
            .payload()
            .map(|p| from_payload(r#type, p))
            .transpose()
    }

    #[getter]
    fn encoding(&self) -> Option<Encoding> {
        self.0.encoding().cloned().map_into()
    }

    // TODO timestamp
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (key_expr, payload, *, encoding = None, congestion_control = None, priority = None, express = None))]
    fn reply(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::new")] key_expr: KeyExpr,
        #[pyo3(from_py_with = "into_payload")] payload: Payload,
        #[pyo3(from_py_with = "Encoding::opt")] encoding: Option<Encoding>,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
    ) -> PyResult<()> {
        allow_threads(py, || {
            let mut builder = self.0.reply(key_expr, payload);
            build!(builder, encoding, congestion_control, priority, express);
            builder.py_res_sync()
        })
    }

    #[pyo3(signature = (payload, *, encoding = None))]
    fn reply_err(
        &self,
        py: Python,
        payload: &Bound<PyAny>,
        encoding: Option<&Bound<PyAny>>,
    ) -> PyResult<()> {
        let value = Value::new(payload, encoding)?.0;
        allow_threads(py, || self.0.reply_err(value).py_res_sync())
    }

    #[pyo3(signature = (key_expr, *, congestion_control = None, priority = None, express = None))]
    fn reply_del(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::new")] key_expr: KeyExpr,
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
    ) -> PyResult<()> {
        allow_threads(py, || {
            let mut builder = self.0.reply_del(key_expr);
            build!(builder, congestion_control, priority, express);
            builder.py_res_sync()
        })
    }

    fn __repr__(&self) -> String {
        self.0.to_string()
    }
}

opt_wrapper!(
    zenoh::queryable::Queryable<'static, HandlerImpl<Query>>,
    "Undeclared queryable"
);

#[pymethods]
impl Queryable {
    #[classmethod]
    fn __class_getitem__(cls: &Bound<PyType>, args: &Bound<PyAny>) -> PyObject {
        generic(cls, args)
    }

    fn __enter__<'a, 'py>(this: &'a Bound<'py, Self>) -> PyResult<&'a Bound<'py, Self>> {
        Self::check(this)
    }

    #[pyo3(signature = (*_args, **_kwargs))]
    fn __exit__(
        &mut self,
        py: Python,
        _args: &Bound<PyTuple>,
        _kwargs: Option<&Bound<PyDict>>,
    ) -> PyResult<()> {
        self.undeclare(py)
    }

    #[getter]
    fn handler(&self, py: Python) -> PyResult<PyObject> {
        Ok(self.get_ref()?.handler().to_object(py))
    }

    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.handler().try_recv(py)
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.handler().recv(py)
    }

    fn undeclare(&mut self, py: Python) -> PyResult<()> {
        allow_threads(py, || self.take()?.undeclare().py_res_sync())
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        self.handler(py)?.bind(py).iter()
    }
}
