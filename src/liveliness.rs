use std::time::Duration;

use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple},
};

use crate::{
    handlers::{into_handler, HandlerImpl},
    key_expr::KeyExpr,
    macros::{build, option_wrapper},
    pubsub::Subscriber,
    query::Reply,
    utils::{timeout, wait, MapInto},
};

#[pyclass]
pub(crate) struct Liveliness(pub(crate) zenoh::Session);

#[pymethods]
impl Liveliness {
    fn declare_token(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
    ) -> PyResult<LivelinessToken> {
        let liveliness = self.0.liveliness();
        let builder = liveliness.declare_token(key_expr);
        wait(py, builder).map_into()
    }

    #[pyo3(signature = (key_expr, handler = None, *, history = None))]
    fn declare_subscriber(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        handler: Option<&Bound<PyAny>>,
        history: Option<bool>,
    ) -> PyResult<Subscriber> {
        let (handler, background) = into_handler(py, handler)?;
        let liveliness = self.0.liveliness();
        let builder = build!(liveliness.declare_subscriber(key_expr), history);
        let mut subscriber = wait(py, builder.with(handler))?;
        if background {
            subscriber.set_background(true);
        }
        Ok(subscriber.into())
    }

    #[pyo3(signature = (key_expr, handler = None, *, timeout = None))]
    fn get(
        &self,
        py: Python,
        #[pyo3(from_py_with = "KeyExpr::from_py")] key_expr: KeyExpr,
        handler: Option<&Bound<PyAny>>,
        #[pyo3(from_py_with = "timeout")] timeout: Option<Duration>,
    ) -> PyResult<HandlerImpl<Reply>> {
        let (handler, _) = into_handler(py, handler)?;
        let liveliness = self.0.liveliness();
        let builder = build!(liveliness.get(key_expr), timeout);
        wait(py, builder.with(handler)).map_into()
    }
}

option_wrapper!(
    zenoh::liveliness::LivelinessToken,
    "Undeclared LivelinessToken"
);

#[pymethods]
impl LivelinessToken {
    fn __enter__<'a, 'py>(this: &'a Bound<'py, Self>) -> PyResult<&'a Bound<'py, Self>> {
        Self::check(this)
    }

    #[pyo3(signature = (*_args, **_kwargs))]
    fn __exit__(
        &mut self,
        py: Python,
        _args: &Bound<PyTuple>,
        _kwargs: Option<&Bound<PyDict>>,
    ) -> PyResult<PyObject> {
        self.undeclare(py)?;
        Ok(py.None())
    }

    fn undeclare(&mut self, py: Python) -> PyResult<()> {
        wait(py, self.take()?.undeclare())
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.get_ref()?))
    }
}
