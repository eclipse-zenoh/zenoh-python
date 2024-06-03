use std::thread;

use pyo3::{prelude::*, sync::GILOnceCell, types::PyDict};
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

fn handle_record(py: Python, record: LogRecord) -> PyResult<()> {
    let level = match record.level {
        tracing::Level::TRACE => 5,
        tracing::Level::DEBUG => 10,
        tracing::Level::INFO => 20,
        tracing::Level::WARN => 30,
        tracing::Level::ERROR => 40,
    };
    let logger = LOGGER.get(py).unwrap().bind(py);
    if !logger
        .call_method1("isEnabledFor", (level,))
        .and_then(|obj| bool::extract_bound(&obj))
        .unwrap_or(false)
    {
        return Ok(());
    }
    let factory = import!(py, logging.getLogRecordFactory).call0()?;
    let kwargs = PyDict::new_bound(py);
    for (k, v) in record.attributes {
        kwargs.set_item(k, v)?;
    }
    let record = factory.call(
        (
            LOGGER_NAME,
            level,
            record.file,
            record.line,
            record.message,
            py.None(),
            py.None(),
        ),
        Some(&kwargs),
    )?;
    logger.call_method1("handle", (record,))?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (basic_config = true, **kwargs))]
pub(crate) fn init_logging(
    py: Python,
    basic_config: bool,
    kwargs: Option<&Bound<PyDict>>,
) -> PyResult<()> {
    import!(py, logging.addLevelName).call1((5, "TRACE"))?;
    if basic_config {
        import!(py, logging.basicConfig).call((), kwargs)?;
    }
    let (tx, rx) = flume::unbounded();
    zenoh_util::init_log_with_callback(move |record| tx.send(record).unwrap());
    thread::spawn(move || {
        for record in rx {
            Python::with_gil(|gil| handle_record(gil, record)).ok();
        }
    });
    Ok(())
}
