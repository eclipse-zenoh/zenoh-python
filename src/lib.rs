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
pub(crate) mod config;
pub(crate) mod encoding;
pub(crate) mod handlers;
pub(crate) mod info;
pub(crate) mod key_expr;
pub(crate) mod payload;
pub(crate) mod publication;
pub(crate) mod query;
pub(crate) mod queryable;
pub(crate) mod sample;
pub(crate) mod scouting;
pub(crate) mod selector;
pub(crate) mod session;
pub(crate) mod subscriber;
pub(crate) mod time;
pub(crate) mod utils;
pub(crate) mod value;

use pyo3::prelude::*;

pyo3::create_exception!(zenoh, ZError, pyo3::exceptions::PyException);

#[pyclass]
struct _Annotation;

#[pymethods]
impl _Annotation {
    fn __getitem__<'py>(this: &Bound<'py, Self>, _key: &Bound<'py, PyAny>) -> Bound<'py, Self> {
        this.clone()
    }
}

#[pymodule]
pub(crate) mod zenoh {
    use pyo3::prelude::*;

    #[pymodule_export]
    use crate::{scouting::scout, session::open, session::Session, ZError};

    #[pymodule]
    mod config {
        use pyo3::prelude::*;

        use crate::_Annotation;
        #[pymodule_export]
        use crate::config::{client, default, empty, peer, Config, WhatAmI, ZenohId};

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            m.add("IntoWhatAmIMatcher", _Annotation)?;
            Ok(())
        }
    }

    #[pymodule]
    mod handlers {
        use pyo3::prelude::*;

        use crate::_Annotation;
        #[pymodule_export]
        use crate::handlers::{CallbackDrop, DefaultHandler, FifoChannel, Handler, RingChannel};

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            m.add("RustHandler", _Annotation)?;
            m.add("PythonHandler", _Annotation)?;
            Ok(())
        }
    }

    #[pymodule]
    mod info {
        #[pymodule_export]
        use crate::info::SessionInfo;
    }

    #[pymodule]
    mod key_expr {
        use pyo3::prelude::*;

        use crate::_Annotation;
        #[pymodule_export]
        use crate::key_expr::{KeyExpr, SetIntersectionLevel};

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            m.add("IntoKeyExpr", _Annotation)?;
            Ok(())
        }
    }

    #[pymodule]
    mod payload {
        #[pymodule_export]
        use crate::payload::{deserializer, serializer};
    }

    #[pymodule]
    mod prelude {
        use pyo3::prelude::*;

        use crate::_Annotation;
        // TODO add config module
        #[pymodule_export]
        use crate::{
            config::{Config, WhatAmI, ZenohId},
            encoding::Encoding,
            key_expr::KeyExpr,
            publication::{CongestionControl, Priority},
            query::{ConsolidationMode, QueryTarget},
            sample::{Sample, SampleKind},
            selector::Selector,
            session::Session,
            subscriber::Reliability,
            value::Value,
        };

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            m.add("IntoEncoding", _Annotation)?;
            Ok(())
        }
    }

    #[pymodule]
    mod publication {
        #[pymodule_export]
        use crate::publication::{CongestionControl, Priority, Publisher};
    }

    #[pymodule]
    mod query {
        #[pymodule_export]
        use crate::query::{ConsolidationMode, QueryTarget, Reply};
    }

    #[pymodule]
    mod queryable {
        #[pymodule_export]
        use crate::queryable::{Query, Queryable};
    }

    #[pymodule]
    mod sample {
        #[pymodule_export]
        use crate::sample::{QoS, Sample, SampleKind};
    }

    #[pymodule]
    mod scouting {
        #[pymodule_export]
        use crate::scouting::{Hello, Scout, WhatAmI};
    }

    #[pymodule]
    mod subscriber {
        #[pymodule_export]
        use crate::subscriber::{Reliability, Subscriber};
    }

    #[pymodule]
    mod time {
        #[pymodule_export]
        use crate::time::Timestamp;
    }

    #[pymodule]
    mod value {
        #[pymodule_export]
        use crate::value::Value;
    }

    #[pyfunction]
    pub(crate) fn init_logger() {
        let _ = env_logger::try_init();
    }

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        let sys_modules = m.py().import_bound("sys")?.getattr("modules")?;
        for module in [
            "config",
            "handlers",
            "info",
            "key_expr",
            "payload",
            "prelude",
            "publication",
            "query",
            "queryable",
            "sample",
            "scouting",
            "subscriber",
            "time",
            "value",
        ] {
            sys_modules.set_item(format!("zenoh.{module}"), m.getattr(module)?)?;
        }

        Ok(())
    }
}

// Test should be runned with `cargo test --no-default-features`
#[test]
#[cfg(not(feature = "default"))]
fn test_no_default_features() {
    assert_eq!(::zenoh::FEATURES, concat!(" zenoh/unstable"));
}
