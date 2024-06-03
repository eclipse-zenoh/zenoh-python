use std::{fmt, thread};

use pyo3::{prelude::*, sync::GILOnceCell, types::PyDict};
use tracing::{
    field::Field,
    span::{Attributes, Record},
    Event, Id, Subscriber,
};
use tracing_subscriber::{
    layer::{Context, SubscriberExt},
    registry::LookupSpan,
};

use crate::{
    macros::import,
    utils::{IntoPyResult, MapInto},
};

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

struct LogRecord {
    level: tracing::Level,
    file: Option<&'static str>,
    line: Option<u32>,
    message: Option<String>,
    kwargs: Vec<(&'static str, String)>,
}

impl LogRecord {
    fn emit(self, py: Python) -> PyResult<()> {
        let level = match self.level {
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
        for (k, v) in self.kwargs {
            kwargs.set_item(k, v)?;
        }
        let record = factory.call(
            (
                LOGGER_NAME,
                level,
                self.file,
                self.line,
                self.message,
                py.None(),
                py.None(),
            ),
            Some(&kwargs),
        )?;
        logger.call_method1("handle", (record,))?;
        Ok(())
    }
}

#[derive(Clone)]
struct SpanFields(Vec<(&'static str, String)>);

struct Layer(flume::Sender<LogRecord>);

impl<S: Subscriber + for<'a> LookupSpan<'a>> tracing_subscriber::Layer<S> for Layer {
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span = ctx.span(id).unwrap();
        let mut extensions = span.extensions_mut();
        let mut fields = vec![];
        attrs.record(&mut |field: &Field, value: &dyn fmt::Debug| {
            fields.push((field.name(), format!("{value:?}")))
        });
        extensions.insert(SpanFields(fields));
    }
    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let span = ctx.span(id).unwrap();
        let mut extensions = span.extensions_mut();
        let fields = extensions.get_mut::<SpanFields>().unwrap();
        values.record(&mut |field: &Field, value: &dyn fmt::Debug| {
            fields.0.push((field.name(), format!("{value:?}")))
        });
    }
    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let mut record = LogRecord {
            level: *event.metadata().level(),
            file: event.metadata().file(),
            line: event.metadata().line(),
            message: None,
            kwargs: vec![],
        };
        if let Some(scope) = ctx.event_scope(event) {
            for span in scope.from_root() {
                let extensions = span.extensions();
                let fields = extensions.get::<SpanFields>().unwrap();
                record.kwargs.extend(fields.0.iter().cloned());
            }
        }
        event.record(&mut |field: &Field, value: &dyn fmt::Debug| {
            if field.name() == "message" {
                record.message = Some(format!("{value:?}"));
            } else {
                record.kwargs.push((field.name(), format!("{value:?}")))
            }
        });
        self.0.send(record).unwrap();
    }
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
    let subscriber = tracing_subscriber::registry().with(Layer(tx));
    tracing::subscriber::set_global_default(subscriber).into_pyres()?;
    thread::spawn(move || {
        for record in rx {
            Python::with_gil(|gil| record.emit(gil)).ok();
        }
    });
    Ok(())
}
