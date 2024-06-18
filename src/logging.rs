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
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
};

use pyo3::{exceptions::PyKeyError, prelude::*, sync::GILOnceCell, types::PyDict};
use tracing::level_filters::LevelFilter;
use zenoh_util::LogRecord;

use crate::{macros::import, utils::MapInto};

const LOGGER_NAME: &str = "zenoh";
static LOGGER: GILOnceCell<PyObject> = GILOnceCell::new();

pub(crate) fn init_logger(py: Python) -> PyResult<()> {
    LOGGER.get_or_try_init(py, || {
        import!(py, logging.getLogger)
            .call1((LOGGER_NAME,))
            .map_into()
    })?;
    Ok(())
}

fn handle_record(
    py: Python,
    loggers: &mut HashMap<String, PyObject>,
    record: LogRecord,
) -> PyResult<()> {
    let mut logger_name = record.target.replace("::", ".");
    if !logger_name.starts_with("zenoh") {
        logger_name.insert_str(0, "zenoh.dependency.");
    }
    let logger = loggers
        .entry(record.target.clone())
        .or_insert_with(|| {
            import!(py, logging.getLogger)
                .call1((&logger_name,))
                .map_or_else(|_| py.None(), Bound::unbind)
        })
        .bind(py);
    if logger.is_none() {
        return Ok(());
    }
    let level = match record.level {
        tracing::Level::TRACE => 5,
        tracing::Level::DEBUG => 10,
        tracing::Level::INFO => 20,
        tracing::Level::WARN => 30,
        tracing::Level::ERROR => 40,
    };
    if !logger
        .call_method1("isEnabledFor", (level,))
        .and_then(|obj| bool::extract_bound(&obj))
        .unwrap_or(false)
    {
        return Ok(());
    }
    let extra = PyDict::new_bound(py);
    for (k, v) in &record.attributes {
        extra.set_item(k, v)?;
    }
    extra.set_item("raw_attributes", extra.copy()?)?;
    let formatted_attributes = record
        .attributes
        .iter()
        .flat_map(|(k, v)| [k, "=", v])
        .collect::<Vec<&str>>()
        .join("=");
    extra.set_item("formatted_attributes", formatted_attributes)?;
    let record = logger.call_method1(
        "makeRecord",
        (
            logger_name,
            level,
            record.file,
            record.line,
            record.message,
            py.None(),
            py.None(),
            py.None(),
            extra,
        ),
    )?;
    logger.call_method1("handle", (record,))?;
    Ok(())
}

#[derive(Clone)]
struct LogFilter(Arc<AtomicUsize>);

impl LogFilter {
    // These constants normally matches `LevelFilter` internal representation
    const TRACE: usize = 0;
    const DEBUG: usize = 1;
    const INFO: usize = 2;
    const WARN: usize = 3;
    const ERROR: usize = 4;
    const OFF: usize = 5;

    fn new(py: Python) -> Self {
        let this = Self(Arc::new(AtomicUsize::new(Self::OFF)));
        this.reset(py);
        this
    }

    fn reset(&self, py: Python) {
        let logger = LOGGER.get(py).unwrap().bind(py);
        let level = logger
            .call_method0("getEffectiveLevel")
            .unwrap()
            .extract::<isize>()
            .unwrap();
        let filter = match level {
            l if l <= 5 => Self::TRACE,
            l if l <= 10 => Self::DEBUG,
            l if l <= 20 => Self::INFO,
            l if l <= 30 => Self::WARN,
            l if l <= 40 => Self::ERROR,
            _ => Self::OFF,
        };
        self.0.store(filter, Ordering::Relaxed);
    }

    fn filter(&self, level: tracing::Level) -> bool {
        let filter = match self.0.load(Ordering::Relaxed) {
            Self::TRACE => LevelFilter::TRACE,
            Self::DEBUG => LevelFilter::DEBUG,
            Self::INFO => LevelFilter::INFO,
            Self::WARN => LevelFilter::WARN,
            Self::ERROR => LevelFilter::ERROR,
            Self::OFF => LevelFilter::OFF,
            _ => unreachable!(),
        };
        level <= filter
    }
}

#[pyclass]
struct LoggerCache {
    inner: Py<PyDict>,
    filter: LogFilter,
}

#[pymethods]
impl LoggerCache {
    fn __getitem__<'py>(
        &self,
        py: Python<'py>,
        key: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        match self.inner.bind(py).get_item(key)? {
            Some(value) => Ok(value),
            None => Err(PyKeyError::new_err(key.clone().unbind())),
        }
    }
    fn __setitem__(&self, py: Python, key: &Bound<PyAny>, value: &Bound<PyAny>) -> PyResult<()> {
        self.inner.bind(py).set_item(key, value)
    }
    fn clear(&self, py: Python) {
        self.inner.bind(py).clear();
        self.filter.reset(py);
        tracing::callsite::rebuild_interest_cache();
    }
}

#[pyfunction]
#[pyo3(signature = (*, raw = false, basic_config = true, **kwargs))]
pub(crate) fn init_logging(
    py: Python,
    raw: bool,
    basic_config: bool,
    kwargs: Option<&Bound<PyDict>>,
) -> PyResult<()> {
    if raw {
        zenoh_util::try_init_log_from_env();
        return Ok(());
    }
    import!(py, logging.addLevelName).call1((5, "TRACE"))?;
    if basic_config {
        import!(py, logging.basicConfig).call((), kwargs)?;
    }
    let filter = LogFilter::new(py);
    let logger = LOGGER.get(py).unwrap().bind(py);
    let cache = LoggerCache {
        inner: PyDict::new_bound(py).unbind(),
        filter: filter.clone(),
    };
    logger.setattr("_cache", cache.into_py(py))?;
    let (tx, rx) = flume::unbounded();
    zenoh_util::init_log_with_callback(
        move |meta| filter.filter(*meta.level()),
        move |record| tx.send(record).unwrap(),
    );
    let mut loggers = HashMap::new();
    thread::spawn(move || {
        for record in rx {
            Python::with_gil(|gil| handle_record(gil, &mut loggers, record)).ok();
        }
    });
    Ok(())
}
