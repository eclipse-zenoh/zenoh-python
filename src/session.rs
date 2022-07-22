//
// Copyright (c) 2017, 2022 ZettaScale Technology Inc.
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh team, <zenoh@zettascale.tech>
//

use std::sync::Arc;

use pyo3::prelude::*;
use zenoh::Session;
use zenoh_core::SyncResolve;

use crate::ToPyErr;

#[pyclass]
#[derive(Clone)]
pub struct _Session(Arc<Session>);

#[pymethods]
impl _Session {
    #[new]
    pub fn new(config: Option<&mut crate::config::_Config>) -> PyResult<Self> {
        let config = config.and_then(|c| c.0.take()).unwrap_or_default();
        let session = zenoh::open(config).res_sync().map_err(|e| e.to_pyerr())?;
        Ok(_Session(Arc::new(session)))
    }
}
