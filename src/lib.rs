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
mod bytes;
mod config;
mod encoding;
mod handlers;
mod info;
mod key_expr;
mod logging;
mod macros;
mod publisher;
mod query;
mod queryable;
mod sample;
mod scouting;
mod selector;
mod session;
mod subscriber;
mod time;
mod utils;

use pyo3::prelude::*;

pyo3::create_exception!(zenoh, ZError, pyo3::exceptions::PyException);

#[pymodule]
pub(crate) mod zenoh {
    use pyo3::prelude::*;

    #[pymodule]
    mod handlers {
        #[pymodule_export]
        use crate::handlers::{CallbackDrop, DefaultHandler, FifoChannel, Handler, RingChannel};
    }

    #[pymodule_export]
    use crate::{
        bytes::{deserializer, serializer},
        config::{Config, WhatAmI, ZenohId},
        encoding::Encoding,
        handlers::Handler,
        info::SessionInfo,
        key_expr::KeyExpr,
        key_expr::SetIntersectionLevel,
        logging::init_logging,
        publisher::{CongestionControl, Priority, Publisher},
        query::{ConsolidationMode, Query, QueryTarget, Reply, ReplyError},
        queryable::Queryable,
        sample::{Sample, SampleKind},
        scouting::scout,
        scouting::Scout,
        selector::Selector,
        session::{open, Session},
        subscriber::Reliability,
        subscriber::Subscriber,
        time::Timestamp,
        ZError,
    };

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        let sys_modules = m.py().import_bound("sys")?.getattr("modules")?;
        sys_modules.set_item("zenoh.handlers", m.getattr("handlers")?)?;
        crate::logging::init_logger(m.py())?;
        Ok(())
    }
}

// Test should be runned with `cargo test --no-default-features`
#[test]
#[cfg(not(feature = "default"))]
fn test_no_default_features() {
    assert_eq!(::zenoh::FEATURES, concat!(" zenoh/unstable"));
}
