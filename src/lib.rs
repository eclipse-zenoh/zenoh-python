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
// TODO https://github.com/eclipse-zenoh/zenoh-python/pull/235#discussion_r1644498390
// mod logging;
mod bytes;
mod config;
#[cfg(feature = "zenoh-ext")]
mod ext;
mod handlers;
mod key_expr;
mod liveliness;
mod macros;
mod matching;
mod pubsub;
mod qos;
mod query;
mod sample;
mod scouting;
mod session;
#[cfg(feature = "shared-memory")]
mod shm;
mod time;
mod utils;

use pyo3::prelude::*;

pyo3::create_exception!(zenoh, ZError, pyo3::exceptions::PyException);
// must be defined here or exporting doesn't work
#[cfg(feature = "zenoh-ext")]
pyo3::create_exception!(zenoh, ZDeserializeError, pyo3::exceptions::PyException);

#[pymodule]
pub(crate) mod zenoh {
    use pyo3::prelude::*;

    #[pyfunction]
    fn try_init_log_from_env() {
        zenoh::try_init_log_from_env();
    }

    #[pyfunction]
    fn init_log_from_env_or(level: &str) {
        zenoh::init_log_from_env_or(level);
    }

    #[pymodule_export]
    use crate::{
        bytes::{Encoding, ZBytes},
        config::{Config, WhatAmI, WhatAmIMatcher, ZenohId},
        handlers::Handler,
        key_expr::{KeyExpr, SetIntersectionLevel},
        liveliness::{Liveliness, LivelinessToken},
        matching::{MatchingListener, MatchingStatus},
        pubsub::{Publisher, Subscriber},
        qos::{CongestionControl, Priority, Reliability},
        query::{
            ConsolidationMode, Parameters, Querier, Query, QueryConsolidation, QueryTarget,
            Queryable, Reply, ReplyError, Selector,
        },
        sample::{Locality, Sample, SampleKind, SourceInfo},
        scouting::{scout, Hello, Scout},
        session::{open, EntityGlobalId, Session, SessionInfo},
        time::{Timestamp, TimestampId},
        ZError,
    };

    #[pymodule]
    mod handlers {
        #[pymodule_export]
        use crate::handlers::{Callback, DefaultHandler, FifoChannel, Handler, RingChannel};
    }

    #[cfg(feature = "zenoh-ext")]
    #[pymodule]
    mod _ext {
        #[pymodule_export]
        use crate::{
            ext::{
                declare_advanced_publisher, declare_advanced_subscriber, z_deserialize,
                z_serialize, AdvancedPublisher, AdvancedSubscriber, CacheConfig, HistoryConfig,
                Miss, MissDetectionConfig, RecoveryConfig, RepliesConfig, SampleMissListener,
            },
            ZDeserializeError,
        };
    }

    #[cfg(feature = "shared-memory")]
    #[pymodule]
    mod shm {
        #[pymodule_export]
        use crate::shm::{
            AllocAlignment, BlockOn, Deallocate, Defragment, GarbageCollect, JustAlloc,
            MemoryLayout, ShmProvider, ZShmMut,
        };
    }

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        let sys_modules = m.py().import("sys")?.getattr("modules")?;
        sys_modules.set_item("zenoh.handlers", m.getattr("handlers")?)?;
        #[cfg(feature = "zenoh-ext")]
        sys_modules.set_item("zenoh._ext", m.getattr("_ext")?)?;
        #[cfg(feature = "shared-memory")]
        sys_modules.set_item("zenoh.shm", m.getattr("shm")?)?;
        // TODO
        // crate::logging::init_logger(m.py())?;
        Ok(())
    }
}

// Test should be runned with `cargo test --no-default-features`
#[test]
#[cfg(not(feature = "default"))]
fn test_no_default_features() {
    assert_eq!(::zenoh::FEATURES, " zenoh/unstable");
}
