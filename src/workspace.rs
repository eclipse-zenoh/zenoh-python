//
// Copyright (c) 2017, 2020 ADLINK Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ADLINK zenoh team, <zenoh@adlink-labs.tech>
//
use crate::to_pyerr;
use crate::types::*;
use async_std::task;
use futures::prelude::*;
use pyo3::prelude::*;

#[pyclass]
pub(crate) struct Workspace {
    pub(crate) w: zenoh::Workspace<'static>,
}

#[pymethods]
impl Workspace {
    fn put(&self, path: String, value: &PyAny) -> PyResult<()> {
        let p = path_of_string(path)?;
        let v = zvalue_of_pyany(value)?;
        task::block_on(self.w.put(&p, v)).map_err(to_pyerr)
    }

    fn delete(&self, path: String) -> PyResult<()> {
        let p = path_of_string(path)?;
        task::block_on(self.w.delete(&p)).map_err(to_pyerr)
    }

    fn get(&self, selector: String) -> PyResult<Vec<Data>> {
        let s = selector_of_string(selector)?;
        task::block_on(async {
            let mut data_stream = self.w.get(&s).await.map_err(to_pyerr)?;
            let mut result = vec![];
            while let Some(d) = data_stream.next().await {
                result.push(Data { d })
            }
            Ok(result)
        })
    }
}
