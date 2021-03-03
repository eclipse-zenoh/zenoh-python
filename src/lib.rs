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
#![recursion_limit = "256"]

use async_std::task;
use pyo3::create_exception;
use pyo3::prelude::*;
use pyo3::{exceptions, wrap_pyfunction, wrap_pymodule};
use std::collections::HashMap;
use std::convert::TryFrom;

mod zenoh_net;
use zenoh_net::*;

mod types;
use types::*;
mod workspace;
use workspace::*;

/// The module of the zenoh API.
///
/// See the :class:`zenoh.Zenoh` class for details
///
/// Quick start examples:
/// ^^^^^^^^^^^^^^^^^^^^^
///
/// Put a key/value into zenoh
/// """"""""""""""""""""""""""
///
/// >>> import zenoh
/// >>> z = zenoh.Zenoh({})
/// >>> w = z.workspace()
/// >>> w.put('/demo/example/hello', 'Hello World!')
/// >>> z.close()
///
/// Subscribe for keys/values changes from zenoh
/// """"""""""""""""""""""""""""""""""""""""""""
///
/// >>> import zenoh, time
/// >>> def listener(change):
/// ...    print(">> [Subscription listener] received {:?} for {} : {} with timestamp {}"
/// ...    .format(change.kind, change.path, '' if change.value is None else change.value.get_content(), change.timestamp))
/// >>>
/// >>> z = zenoh.Zenoh({})
/// >>> w = z.workspace()
/// >>> sub = w.subscribe('/demo/example/**', listener)
/// >>> time.sleep(60)
/// >>> sub.close()
/// >>> z.close()
///
/// Get keys/values from zenoh
/// """"""""""""""""""""""""""
///
/// >>> import zenoh
/// >>> z = zenoh.Zenoh({})
/// >>> w = z.workspace()
/// >>> for data in w.get('/demo/example/**'):
/// ...     print('  {} : {}  (encoding: {} , timestamp: {})'.format(
/// ...         data.path, data.value.get_content(), data.value.encoding_descr(), data.timestamp))
/// >>> z.close()
///
#[pymodule]
fn zenoh(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(net))?;
    // force addition of "zenoh.net" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.net'] = net
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_wrapped(wrap_pyfunction!(init_logger))?;
    m.add_wrapped(wrap_pyfunction!(config_from_file))?;

    m.add_class::<Zenoh>()?;
    m.add_class::<Workspace>()?;
    m.add_class::<Selector>()?;
    m.add_class::<Value>()?;
    m.add_class::<Data>()?;
    m.add_class::<ChangeKind>()?;
    m.add_class::<Change>()?;
    m.add_class::<types::Subscriber>()?;
    m.add_class::<GetRequest>()?;
    m.add_class::<Eval>()?;

    Ok(())
}

create_exception!(zenoh, ZError, exceptions::PyException);

fn to_pyerr(err: zenoh::ZError) -> PyErr {
    PyErr::new::<ZError, _>(err.to_string())
}

/// Initialize the logger used by the Rust implementation of this API.
///
/// Once initialized, you can configure the logs displayed by the API using the ``RUST_LOG`` environment variable.
/// For instance, start python with the *debug* logs available::
///
///    $ RUST_LOG=debug python
///
/// More details on the RUST_LOG configuration on https://docs.rs/env_logger/latest/env_logger
///
#[pyfunction]
fn init_logger() {
    env_logger::init();
}

/// Parse a configuration file for zenoh, returning a dictionary of str:str.
/// The file must contain 1 "key=value" property per line. Comments lines starting with '#' character are ignored.
///
/// :param path: The path to the config file.
///
#[pyfunction]
fn config_from_file(path: &str) -> PyResult<HashMap<String, String>> {
    zenoh::Properties::try_from(std::path::Path::new(path))
        .map(|p| p.0)
        .map_err(to_pyerr)
}

/// The zenoh client API.
///
/// Creates a zenoh API, establishing a zenoh-net session with discovered peers and/or routers.
///
/// :param config: The configuration of the zenoh session
/// :param config: dict of str:str
#[pyclass]
#[text_signature = "(config)"]
pub(crate) struct Zenoh {
    z: Option<zenoh::Zenoh>,
}

#[pymethods]
impl Zenoh {
    #[new]
    fn new(config: HashMap<String, String>) -> PyResult<Zenoh> {
        let z = task::block_on(zenoh::Zenoh::new(config.into())).map_err(to_pyerr)?;
        Ok(Zenoh { z: Some(z) })
    }

    /// Closes the zenoh API and the associated zenoh-net session
    fn close(&mut self) -> PyResult<()> {
        let z = self.take()?;
        task::block_on(z.close()).map_err(to_pyerr)
    }

    /// Returns the PeerId of the zenoh router this zenoh API is connected to (if any).
    fn router_pid(&self) -> PyResult<Option<String>> {
        let z = self.as_ref()?;
        Ok(task::block_on(z.router_pid()))
    }

    /// Creates a [`Workspace`] with an optional [`Path`] as `prefix`.
    ///
    /// :param prefix: an optional prefix
    /// :type prefix: str
    /// :return: a Workspace
    /// :rtype: Workspace
    ///
    /// :Example:
    ///
    /// >>> z = zenoh.Zenoh(zenoh.net.Config())
    /// >>> w = z.workspace()
    ///
    #[text_signature = "(prefix=None)"]
    fn workspace(&self, prefix: Option<String>) -> PyResult<Workspace> {
        let p = prefix.map(path_of_string).transpose()?;
        let z = self.as_ref()?;
        let workspace = task::block_on(z.workspace(p)).map_err(to_pyerr)?;

        // Note: this is a workaround for pyo3 not supporting lifetime in PyClass. See https://github.com/PyO3/pyo3/issues/502.
        // We extend zenoh::Workspace's lifetime to 'static to be wrapped in Publisher PyClass
        let w = unsafe {
            std::mem::transmute::<zenoh::Workspace<'_>, zenoh::Workspace<'static>>(workspace)
        };

        Ok(Workspace { w })
    }
}

impl Zenoh {
    #[inline]
    fn as_ref(&self) -> PyResult<&zenoh::Zenoh> {
        self.z
            .as_ref()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh was closed"))
    }

    #[inline]
    fn take(&mut self) -> PyResult<zenoh::Zenoh> {
        self.z
            .take()
            .ok_or_else(|| PyErr::new::<ZError, _>("zenoh was closed"))
    }
}
