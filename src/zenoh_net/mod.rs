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
use crate::{pydict_to_props, to_pyerr};
use async_std::prelude::FutureExt;
use async_std::task;
use futures::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;
use zenoh::net::ZInt;

pub(crate) mod types;
pub(crate) use types::*;
mod session;
use session::*;
mod data_kind;
mod encoding;

/// The network level zenoh API.
///
/// Examples:
/// ^^^^^^^^^
///
/// Publish
/// """""""
///
/// >>> import zenoh
/// >>> s = zenoh.net.open({})
/// >>> s.write('/resource/name', bytes('value', encoding='utf8'))
///
/// Subscribe
/// """""""""
///
/// >>> import zenoh
/// >>> from zenoh.net import SubInfo, Reliability, SubMode
/// >>> def listener(sample):
/// ...     print("Received : {}".format(sample))
/// >>>
/// >>> s = zenoh.net.open({})
/// >>> sub_info = SubInfo(Reliability.Reliable, SubMode.Push)
/// >>> sub = s.declare_subscriber('/resource/name', sub_info, listener)
///
/// Query
/// """""
///
/// >>> import zenoh, time
/// >>> from zenoh.net import QueryTarget, queryable
/// >>> def query_callback(reply):
/// ...     print("Received : {}".format(reply))
/// >>>
/// >>> s = zenoh.net.open({})
/// >>> s.query('/resource/name', 'predicate', query_callback)
/// >>> time.sleep(1)
#[pymodule]
pub(crate) fn net(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<config>()?;
    // force addition of "zenoh.net.config" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.net.config'] = config
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<info>()?;
    // force addition of "zenoh.net.info" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.net.info'] = info
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<whatami>()?;
    // force addition of "zenoh.net.whatami" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.net.whatami'] = whatami
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<queryable>()?;
    // force addition of "zenoh.net.queryable" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.net.queryable'] = queryable
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<resource_name>()?;
    // force addition of "zenoh.net.resource_name" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.net.resource_name'] = resource_name
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<encoding::encoding>()?;
    // force addition of "zenoh.net.encoding" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.net.encoding'] = encoding
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<data_kind::data_kind>()?;
    // force addition of "zenoh.net.data_kind" module
    // (see https://github.com/PyO3/pyo3/issues/759#issuecomment-653964601)
    py.run(
        "\
import sys
sys.modules['zenoh.net.data_kind'] = data_kind
        ",
        None,
        Some(m.dict()),
    )?;

    m.add_class::<Hello>()?;
    m.add_class::<ResKey>()?;
    m.add_class::<PeerId>()?;
    m.add_class::<Timestamp>()?;
    m.add_class::<DataInfo>()?;
    m.add_class::<Sample>()?;
    m.add_class::<Reliability>()?;
    m.add_class::<SubMode>()?;
    m.add_class::<Period>()?;
    m.add_class::<SubInfo>()?;
    m.add_class::<CongestionControl>()?;
    m.add_class::<Publisher>()?;
    m.add_class::<Subscriber>()?;
    m.add_class::<Query>()?;
    m.add_class::<Queryable>()?;
    m.add_class::<Target>()?;
    m.add_class::<QueryTarget>()?;
    m.add_class::<ConsolidationMode>()?;
    m.add_class::<QueryConsolidation>()?;
    m.add_class::<Reply>()?;
    m.add_class::<Session>()?;
    m.add_wrapped(wrap_pyfunction!(open))?;
    m.add_wrapped(wrap_pyfunction!(scout))?;
    Ok(())
}

/// Open a zenoh-net Session.
///
/// :param config: The configuration of the zenoh-net session
/// :type config: dict {str: str}
/// :rtype: Session
///
/// :Example:
///
/// >>> import zenoh
/// >>> z = zenoh.net.open(zenoh.net.config::peer())
#[pyfunction]
#[text_signature = "(config)"]
fn open(config: &PyDict) -> PyResult<Session> {
    let s = task::block_on(zenoh::net::open(pydict_to_props(config).into())).map_err(to_pyerr)?;
    Ok(Session::new(s))
}

/// Scout for routers and/or peers.
///
/// This spawns a task that periodically sends scout messages for a specified duration and returns
/// a list of received :class:`Hello` messages.
///
/// :param whatami: The kind of zenoh process to scout for
/// :type whatami: int
/// :param config: The configuration to use for scouting
/// :type config: dict {str: str}
/// :param scout_duration: the duration of scout (in seconds)
/// :type scout_duration: float
/// :rtype: list of :class:`Hello`
///
/// :Example:
///
/// >>> import zenoh
/// >>> hellos = zenoh.net.scout(zenoh.net.whatami.PEER | zenoh.net.whatami.ROUTER, {}, 1.0)
/// >>> for hello in hellos:
/// >>>     print(hello)
#[pyfunction]
#[text_signature = "(whatami, iface, scout_duration)"]
fn scout(whatami: ZInt, config: &PyDict, scout_duration: f64) -> PyResult<Vec<Hello>> {
    task::block_on(async move {
        let mut result = Vec::<Hello>::new();
        let mut receiver = zenoh::net::scout(whatami, pydict_to_props(config).into())
            .await
            .unwrap();
        let scout = async {
            while let Some(h) = receiver.next().await {
                result.push(Hello { h })
            }
        };
        let timeout = async_std::task::sleep(std::time::Duration::from_secs_f64(scout_duration));
        FutureExt::race(scout, timeout).await;
        Ok(result)
    })
}
