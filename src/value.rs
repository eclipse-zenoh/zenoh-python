use pyo3::{
    prelude::*,
    types::{PyBytes, PyType},
};

use crate::{
    encoding::Encoding,
    payload::{from_payload, into_payload, payload_to_bytes},
    utils::{try_downcast, wrapper},
};

wrapper!(zenoh::value::Value: Clone);

#[pymethods]
impl Value {
    #[new]
    pub(crate) fn new(payload: &Bound<PyAny>, encoding: Option<&Bound<PyAny>>) -> PyResult<Self> {
        try_downcast!(payload);
        Ok(Self(zenoh::value::Value::new(
            into_payload(payload)?,
            Encoding::new(encoding)?.0,
        )))
    }

    #[getter]
    fn payload<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        payload_to_bytes(py, self.0.payload())
    }

    fn payload_as(&self, r#type: &Bound<PyType>) -> PyResult<PyObject> {
        from_payload(r#type, self.0.payload())
    }

    #[getter]
    fn encoding(&self) -> Encoding {
        self.0.encoding().clone().into()
    }
}
