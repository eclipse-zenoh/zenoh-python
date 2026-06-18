use std::time::Duration;

use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
    types::{
        PyBool, PyByteArray, PyBytes, PyDict, PyFloat, PyFrozenSet, PyInt, PyIterator, PyList,
        PySet, PyString, PyTuple, PyType,
    },
    IntoPyObjectExt, PyTypeInfo,
};
use zenoh_ext::{
    AdvancedPublisherBuilderExt, AdvancedSubscriberBuilderExt, Deserialize, VarInt, ZDeserializer,
    ZSerializer,
};

use crate::{
    bytes::{Encoding, ZBytes},
    handlers::{into_handler, HandlerImpl},
    key_expr::KeyExpr,
    macros::{build, import, option_wrapper, py_static, try_import, wrapper},
    pubsub::Subscriber,
    qos::{CongestionControl, Priority, Reliability},
    sample::{Locality, Sample},
    session::{EntityGlobalId, Session},
    time::Timestamp,
    utils::{duration, generic, wait, MapInto},
    ZDeserializeError,
};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum SupportedType {
    ZBytes,
    ByteArray,
    Bytes,
    Str,
    Int,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    UInt128,
    Float,
    Float32,
    Float64,
    Bool,
    List,
    Tuple,
    Dict,
    Set,
    FrozenSet,
}

impl SupportedType {
    fn init_dict(py: Python) -> Py<PyDict> {
        let dict = PyDict::new(py);
        fn add_type<T: PyTypeInfo>(py: Python, dict: &Bound<PyDict>, tp: SupportedType) {
            dict.set_item(T::type_object(py), tp as u8).unwrap()
        }
        let ext = py.import("zenoh.ext").unwrap();
        let add_wrapper_type = |name, tp| {
            let wrapper = ext.getattr(name).unwrap();
            dict.set_item(wrapper, tp as u8).unwrap();
        };
        add_type::<ZBytes>(py, &dict, SupportedType::ZBytes);
        add_type::<PyByteArray>(py, &dict, SupportedType::ByteArray);
        add_type::<PyBytes>(py, &dict, SupportedType::Bytes);
        add_type::<PyString>(py, &dict, SupportedType::Str);
        add_type::<PyInt>(py, &dict, SupportedType::Int);
        add_wrapper_type("Int8", SupportedType::Int8);
        add_wrapper_type("Int16", SupportedType::Int16);
        add_wrapper_type("Int32", SupportedType::Int32);
        add_wrapper_type("Int64", SupportedType::Int64);
        add_wrapper_type("Int128", SupportedType::Int128);
        add_wrapper_type("UInt8", SupportedType::UInt8);
        add_wrapper_type("UInt16", SupportedType::UInt16);
        add_wrapper_type("UInt32", SupportedType::UInt32);
        add_wrapper_type("UInt64", SupportedType::UInt64);
        add_wrapper_type("UInt128", SupportedType::UInt128);
        add_type::<PyFloat>(py, &dict, SupportedType::Float);
        add_wrapper_type("Float32", SupportedType::Float32);
        add_wrapper_type("Float64", SupportedType::Float64);
        add_type::<PyBool>(py, &dict, SupportedType::Bool);
        add_type::<PyList>(py, &dict, SupportedType::List);
        add_type::<PyTuple>(py, &dict, SupportedType::Tuple);
        add_type::<PyDict>(py, &dict, SupportedType::Dict);
        add_type::<PySet>(py, &dict, SupportedType::Set);
        add_type::<PyFrozenSet>(py, &dict, SupportedType::FrozenSet);
        dict.unbind()
    }

    fn from_int(int: u8) -> Self {
        match int {
            n if n == Self::ZBytes as u8 => Self::ZBytes,
            n if n == Self::ByteArray as u8 => Self::ByteArray,
            n if n == Self::Bytes as u8 => Self::Bytes,
            n if n == Self::Str as u8 => Self::Str,
            n if n == Self::Int as u8 => Self::Int,
            n if n == Self::Int8 as u8 => Self::Int8,
            n if n == Self::Int16 as u8 => Self::Int16,
            n if n == Self::Int32 as u8 => Self::Int32,
            n if n == Self::Int64 as u8 => Self::Int64,
            n if n == Self::Int128 as u8 => Self::Int128,
            n if n == Self::UInt8 as u8 => Self::UInt8,
            n if n == Self::UInt16 as u8 => Self::UInt16,
            n if n == Self::UInt32 as u8 => Self::UInt32,
            n if n == Self::UInt64 as u8 => Self::UInt64,
            n if n == Self::UInt128 as u8 => Self::UInt128,
            n if n == Self::Float as u8 => Self::Float,
            n if n == Self::Float32 as u8 => Self::Float32,
            n if n == Self::Float64 as u8 => Self::Float64,
            n if n == Self::Bool as u8 => Self::Bool,
            n if n == Self::List as u8 => Self::List,
            n if n == Self::Tuple as u8 => Self::Tuple,
            n if n == Self::Dict as u8 => Self::Dict,
            n if n == Self::Set as u8 => Self::Set,
            n if n == Self::FrozenSet as u8 => Self::FrozenSet,
            _ => unreachable!(),
        }
    }

    fn from_type(tp: &Bound<PyType>) -> Option<Self> {
        let py = tp.py();
        let dict = py_static!(py, PyDict, || Ok(Self::init_dict(py))).ok()?;
        Some(Self::from_int(dict.get_item(tp).ok()??.extract().unwrap()))
    }

    fn try_from_type(tp: &Bound<PyType>) -> PyResult<Self> {
        match Self::from_type(tp) {
            Some(res) => Ok(res),
            None => Err(PyTypeError::new_err(format!(
                "type {} is not supported",
                tp.get_type().name()?
            ))),
        }
    }
}

fn serialize(serializer: &mut ZSerializer, obj: &Bound<PyAny>) -> PyResult<()> {
    serialize_impl(
        serializer,
        obj,
        SupportedType::try_from_type(&obj.get_type())?,
    )
}

fn serialize_impl(
    serializer: &mut ZSerializer,
    obj: &Bound<PyAny>,
    tp: SupportedType,
) -> PyResult<()> {
    let item_type = |obj: &Bound<PyAny>| SupportedType::try_from_type(&obj.get_type());
    let serialize_item =
        |serializer: &mut ZSerializer, obj, tp| serialize_impl(serializer, &obj, tp);
    let pair_type = |kv: &(Bound<PyAny>, Bound<PyAny>)| {
        Ok((
            SupportedType::try_from_type(&kv.0.get_type())?,
            SupportedType::try_from_type(&kv.1.get_type())?,
        ))
    };
    let serialize_pair = |serializer: &mut ZSerializer, (k, v), (tp_k, tp_v)| {
        serialize_impl(serializer, &k, tp_k)?;
        serialize_impl(serializer, &v, tp_v)?;
        Ok(())
    };
    match tp {
        SupportedType::ZBytes => serializer.serialize(obj.extract::<ZBytes>()?.0),
        // SAFETY: bytes are immediately copied
        SupportedType::ByteArray => {
            serializer.serialize(unsafe { obj.downcast::<PyByteArray>()?.as_bytes() })
        }
        SupportedType::Bytes => serializer.serialize(obj.downcast::<PyBytes>()?.as_bytes()),
        SupportedType::Str => serializer.serialize(&obj.downcast::<PyString>()?.to_cow()?),
        SupportedType::Int8 => serializer.serialize(obj.extract::<i8>()?),
        SupportedType::Int16 => serializer.serialize(obj.extract::<i16>()?),
        SupportedType::Int | SupportedType::Int32 => serializer.serialize(obj.extract::<i32>()?),
        SupportedType::Int64 => serializer.serialize(obj.extract::<i64>()?),
        SupportedType::Int128 => serializer.serialize(obj.extract::<i128>()?),
        SupportedType::UInt8 => serializer.serialize(obj.extract::<u8>()?),
        SupportedType::UInt16 => serializer.serialize(obj.extract::<u16>()?),
        SupportedType::UInt32 => serializer.serialize(obj.extract::<u32>()?),
        SupportedType::UInt64 => serializer.serialize(obj.extract::<u64>()?),
        SupportedType::UInt128 => serializer.serialize(obj.extract::<u128>()?),
        SupportedType::Float | SupportedType::Float64 => {
            serializer.serialize(obj.extract::<f64>()?)
        }
        SupportedType::Float32 => serializer.serialize(obj.extract::<f64>()? as f32),
        SupportedType::Bool => serializer.serialize(obj.extract::<bool>()?),
        SupportedType::List => serialize_iter(
            serializer,
            obj.downcast::<PyList>()?,
            item_type,
            serialize_item,
        )?,
        SupportedType::Tuple => {
            let tuple = obj.downcast::<PyTuple>()?;
            for item in tuple {
                serialize(serializer, &item)?;
            }
        }
        SupportedType::Dict => serialize_iter(
            serializer,
            obj.downcast::<PyDict>()?,
            pair_type,
            serialize_pair,
        )?,
        SupportedType::Set => serialize_iter(
            serializer,
            obj.downcast::<PySet>()?,
            item_type,
            serialize_item,
        )?,
        SupportedType::FrozenSet => serialize_iter(
            serializer,
            obj.downcast::<PyFrozenSet>()?,
            item_type,
            serialize_item,
        )?,
    }
    Ok(())
}

fn serialize_iter<T, I: IntoIterator<Item = T>, Ty: Eq + Copy>(
    serializer: &mut ZSerializer,
    iter: I,
    get_type: impl Fn(&T) -> PyResult<Ty>,
    serialize: impl Fn(&mut ZSerializer, T, Ty) -> PyResult<()>,
) -> PyResult<()>
where
    I::IntoIter: ExactSizeIterator,
{
    let iter = iter.into_iter();
    serializer.serialize(VarInt(iter.len()));
    let mut tp = None;
    for item in iter {
        match &tp {
            Some(tp) if get_type(&item)? != *tp => {
                return Err(PyValueError::new_err(
                    "all items of serialized collections must have the same type",
                ))
            }
            Some(_) => {}
            None => tp = Some(get_type(&item)?),
        }
        serialize(serializer, item, tp.unwrap())?;
    }
    Ok(())
}

#[pyfunction]
pub(crate) fn z_serialize(obj: &Bound<PyAny>) -> PyResult<ZBytes> {
    let mut serializer = ZSerializer::new();
    serialize(&mut serializer, obj)?;
    Ok(serializer.finish().into())
}

struct DeserializationError(PyErr);

impl From<PyErr> for DeserializationError {
    fn from(value: PyErr) -> Self {
        Self(value)
    }
}

impl From<zenoh_ext::ZDeserializeError> for DeserializationError {
    fn from(_: zenoh_ext::ZDeserializeError) -> Self {
        Self(ZDeserializeError::new_err("deserialization error"))
    }
}

fn get_deserialization_type<'py>(
    tp: &Bound<'py, PyAny>,
) -> PyResult<(SupportedType, Option<Bound<'py, PyTuple>>)> {
    let py = tp.py();
    if try_import!(py, types.GenericAlias).is_ok_and(|alias| tp.is_instance(alias).unwrap_or(false))
    {
        let origin = import!(py, typing.get_origin)
            .call1((tp,))?
            .downcast_into::<PyType>()
            .map_err(PyErr::from)?;
        let args = import!(py, typing.get_args)
            .call1((tp,))?
            .downcast_into::<PyTuple>()
            .map_err(PyErr::from)?;
        Ok((SupportedType::try_from_type(&origin)?, Some(args)))
    } else {
        let tp = tp.downcast::<PyType>().map_err(PyErr::from)?;
        Ok((SupportedType::try_from_type(tp)?, None))
    }
}

fn deserialize(
    deserializer: &mut ZDeserializer,
    tp: &Bound<PyAny>,
) -> Result<PyObject, DeserializationError> {
    let (tp2, args) = get_deserialization_type(tp)?;
    deserialize_impl(deserializer, tp.py(), tp2, args)
}

fn deserialize_impl(
    deserializer: &mut ZDeserializer,
    py: Python,
    tp: SupportedType,
    args: Option<Bound<PyTuple>>,
) -> Result<PyObject, DeserializationError> {
    macro_rules! deserialize_wrapper {
        ($tp:ty, $wrapper:ident) => {
            import!(py, "zenoh.ext", $wrapper)
                .call1((deserializer.deserialize::<$tp>()?,))?
                .into_py_any(py)?
        };
    }
    let unwrap_args = || {
        let err = "collection types must be specialized with their generic parameter(s)";
        args.ok_or_else(|| PyValueError::new_err(err))
    };
    Ok(match tp {
        SupportedType::ZBytes => {
            ZBytes(deserializer.deserialize::<Vec<u8>>()?.into()).into_py_any(py)?
        }
        SupportedType::ByteArray => {
            PyByteArray::new(py, &deserializer.deserialize::<Vec<u8>>()?).into_py_any(py)?
        }
        SupportedType::Bytes => {
            PyBytes::new(py, &deserializer.deserialize::<Vec<u8>>()?).into_py_any(py)?
        }
        SupportedType::Str => deserializer.deserialize::<String>()?.into_py_any(py)?,
        SupportedType::Int => deserializer.deserialize::<i32>()?.into_py_any(py)?,
        SupportedType::Int8 => deserialize_wrapper!(i8, Int8),
        SupportedType::Int16 => deserialize_wrapper!(i16, Int16),
        SupportedType::Int32 => deserialize_wrapper!(i32, Int32),
        SupportedType::Int64 => deserialize_wrapper!(i64, Int64),
        SupportedType::Int128 => deserialize_wrapper!(i128, Int128),
        SupportedType::UInt8 => deserialize_wrapper!(u8, UInt8),
        SupportedType::UInt16 => deserialize_wrapper!(u16, UInt16),
        SupportedType::UInt32 => deserialize_wrapper!(u32, UInt32),
        SupportedType::UInt64 => deserialize_wrapper!(u64, UInt64),
        SupportedType::UInt128 => deserialize_wrapper!(u128, UInt128),
        SupportedType::Float => deserializer.deserialize::<f64>()?.into_py_any(py)?,
        SupportedType::Float32 => deserialize_wrapper!(f32, Float32),
        SupportedType::Float64 => deserialize_wrapper!(f64, Float64),
        SupportedType::Bool => deserializer.deserialize::<bool>()?.into_py_any(py)?,
        tp @ (SupportedType::List | SupportedType::Set | SupportedType::FrozenSet) => {
            deserialize_collection(deserializer, py, tp, unwrap_args()?)?
        }
        SupportedType::Tuple => {
            let args = unwrap_args()?;
            if args
                .get_item(1)
                .ok()
                .is_some_and(|arg| arg.is(py.Ellipsis()))
            {
                return Err(DeserializationError(PyTypeError::new_err(
                    "any size tuples are not supported",
                )));
            }
            let items = args
                .iter()
                .map(|arg| deserialize(deserializer, &arg))
                .collect::<Result<Vec<_>, _>>()?;
            PyTuple::new(py, items)?.into_py_any(py)?
        }
        SupportedType::Dict => {
            let dict = PyDict::new(py);
            let args = unwrap_args()?;
            let (k_tp, k_args) = get_deserialization_type(&args.get_item(0).expect("no key type"))?;
            let (v_tp, v_args) = get_deserialization_type(&args.get_item(1).expect("no key type"))?;
            let len = deserializer.deserialize::<VarInt<usize>>()?.0;
            for _ in 0..len {
                let k = deserialize_impl(deserializer, py, k_tp, k_args.clone())?;
                let v = deserialize_impl(deserializer, py, v_tp, v_args.clone())?;
                dict.set_item(k, v)?;
            }
            dict.into_py_any(py)?
        }
    })
}

fn deserialize_collection(
    deserializer: &mut ZDeserializer,
    py: Python,
    tp: SupportedType,
    args: Bound<PyTuple>,
) -> Result<PyObject, DeserializationError> {
    let item = args.get_item(0).expect("no item type");
    let (item_tp, item_args) = get_deserialization_type(&item)?;
    fn from_vec<'py, T: Deserialize + IntoPyObject<'py>>(
        deserializer: &mut ZDeserializer,
        py: Python<'py>,
        tp: SupportedType,
    ) -> Result<PyObject, DeserializationError> {
        let vec: Vec<T> = deserializer.deserialize()?;
        Ok(match tp {
            SupportedType::List => PyList::new(py, vec)?.into_py_any(py)?,
            SupportedType::Set => PySet::new(py, vec)?.into_py_any(py)?,
            SupportedType::FrozenSet => PySet::new(py, vec)?.into_py_any(py)?,
            _ => unreachable!(),
        })
    }
    match item_tp {
        SupportedType::Int8 => from_vec::<i8>(deserializer, py, tp),
        SupportedType::Int16 => from_vec::<i16>(deserializer, py, tp),
        SupportedType::Int32 => from_vec::<i32>(deserializer, py, tp),
        SupportedType::Int64 => from_vec::<i64>(deserializer, py, tp),
        SupportedType::Int128 => from_vec::<i128>(deserializer, py, tp),
        SupportedType::UInt8 => from_vec::<u8>(deserializer, py, tp),
        SupportedType::UInt16 => from_vec::<u16>(deserializer, py, tp),
        SupportedType::UInt32 => from_vec::<u32>(deserializer, py, tp),
        SupportedType::UInt64 => from_vec::<u64>(deserializer, py, tp),
        SupportedType::UInt128 => from_vec::<u128>(deserializer, py, tp),
        SupportedType::Float32 => from_vec::<f32>(deserializer, py, tp),
        SupportedType::Float64 => from_vec::<f64>(deserializer, py, tp),
        _ => {
            let list = PyList::empty(py);
            let len = deserializer.deserialize::<VarInt<usize>>()?.0;
            for _ in 0..len {
                list.append(deserialize_impl(
                    deserializer,
                    py,
                    item_tp,
                    item_args.clone(),
                )?)?;
            }
            Ok(match tp {
                SupportedType::List => list.into_py_any(py)?,
                SupportedType::Set => PySet::type_object(py).call1((list,))?.into_py_any(py)?,
                SupportedType::FrozenSet => PyFrozenSet::type_object(py)
                    .call1((list,))?
                    .into_py_any(py)?,
                _ => unreachable!(),
            })
        }
    }
}

#[pyfunction]
pub(crate) fn z_deserialize(tp: &Bound<PyAny>, zbytes: &ZBytes) -> PyResult<PyObject> {
    let mut deserializer = ZDeserializer::new(&zbytes.0);
    deserialize(&mut deserializer, tp).map_err(|err| err.0)
}

option_wrapper!(
    zenoh_ext::AdvancedPublisher<'static>,
    "Undeclared publisher"
);

#[pymethods]
impl AdvancedPublisher {
    fn __enter__<'a, 'py>(this: &'a Bound<'py, Self>) -> PyResult<&'a Bound<'py, Self>> {
        Self::check(this)
    }

    #[pyo3(signature = (*_args, **_kwargs))]
    fn __exit__(
        &mut self,
        py: Python,
        _args: &Bound<PyTuple>,
        _kwargs: Option<&Bound<PyDict>>,
    ) -> PyResult<PyObject> {
        self.undeclare(py)?;
        Ok(py.None())
    }

    #[getter]
    fn id(&self) -> PyResult<EntityGlobalId> {
        Ok(self.get_ref()?.id().into())
    }

    #[getter]
    fn key_expr(&self) -> PyResult<KeyExpr> {
        Ok(self.get_ref()?.key_expr().clone().into())
    }

    #[getter]
    fn encoding(&self) -> PyResult<Encoding> {
        Ok(self.get_ref()?.encoding().clone().into())
    }

    #[getter]
    fn congestion_control(&self) -> PyResult<CongestionControl> {
        Ok(self.get_ref()?.congestion_control().into())
    }

    #[getter]
    fn priority(&self) -> PyResult<Priority> {
        Ok(self.get_ref()?.priority().into())
    }

    #[pyo3(signature = (payload, *, encoding = None, attachment = None, timestamp = None))]
    fn put(
        &self,
        py: Python,
        #[pyo3(from_py_with = ZBytes::from_py)] payload: ZBytes,
        #[pyo3(from_py_with = Encoding::from_py_opt)] encoding: Option<Encoding>,
        #[pyo3(from_py_with = ZBytes::from_py_opt)] attachment: Option<ZBytes>,
        timestamp: Option<Timestamp>,
    ) -> PyResult<()> {
        let this = self.get_ref()?;
        wait(
            py,
            build!(this.put(payload), encoding, attachment, timestamp),
        )
    }

    #[pyo3(signature = (*, attachment = None, timestamp = None))]
    fn delete(
        &self,
        py: Python,
        #[pyo3(from_py_with = ZBytes::from_py_opt)] attachment: Option<ZBytes>,
        timestamp: Option<Timestamp>,
    ) -> PyResult<()> {
        wait(py, build!(self.get_ref()?.delete(), attachment, timestamp))
    }

    fn undeclare(&mut self, py: Python) -> PyResult<()> {
        wait(py, self.take()?.undeclare())
    }
}

option_wrapper!(
    zenoh_ext::AdvancedSubscriber<HandlerImpl<Sample>>,
    "Undeclared subscriber"
);

#[pymethods]
impl AdvancedSubscriber {
    #[classmethod]
    fn __class_getitem__(cls: &Bound<PyType>, args: &Bound<PyAny>) -> PyObject {
        generic(cls, args)
    }

    fn __enter__<'a, 'py>(this: &'a Bound<'py, Self>) -> &'a Bound<'py, Self> {
        this
    }

    #[pyo3(signature = (*_args, **_kwargs))]
    fn __exit__(
        &mut self,
        py: Python,
        _args: &Bound<PyTuple>,
        _kwargs: Option<&Bound<PyDict>>,
    ) -> PyResult<PyObject> {
        self.undeclare(py)?;
        Ok(py.None())
    }

    #[getter]
    fn id(&self) -> PyResult<EntityGlobalId> {
        Ok(self.get_ref()?.id().into())
    }

    #[getter]
    fn key_expr(&self) -> PyResult<KeyExpr> {
        Ok(self.get_ref()?.key_expr().clone().into())
    }

    #[getter]
    fn handler(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.handler().into_py_any(py)
    }

    #[pyo3(signature = (handler = None))]
    fn sample_miss_listener(
        &self,
        py: Python,
        handler: Option<&Bound<PyAny>>,
    ) -> PyResult<SampleMissListener> {
        let (handler, background) = into_handler(py, handler, None)?;
        let builder = self.get_ref()?.sample_miss_listener();
        let mut listener = wait(py, builder.with(handler))?;
        if background {
            listener.set_background(true);
        }
        Ok(listener.into())
    }

    #[pyo3(signature = (handler = None, *, history = None))]
    fn detect_publishers(
        &self,
        py: Python,
        handler: Option<&Bound<PyAny>>,
        history: Option<bool>,
    ) -> PyResult<Subscriber> {
        let (handler, background) = into_handler(py, handler, None)?;
        let builder = build!(self.get_ref()?.detect_publishers(), history);
        let mut subscriber = wait(py, builder.with(handler))?;
        if background {
            subscriber.set_background(true);
        }
        Ok(subscriber.into())
    }

    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.handler().try_recv(py)
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.handler().recv(py)
    }

    fn undeclare(&mut self, py: Python) -> PyResult<()> {
        wait(py, self.take()?.undeclare())
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        self.handler(py)?.bind(py).try_iter()
    }
}

wrapper!(zenoh_ext::CacheConfig: Clone);

#[pymethods]
impl CacheConfig {
    #[new]
    #[pyo3(signature = (max_samples = None, *, replies_config = None))]
    fn new(max_samples: Option<usize>, replies_config: Option<RepliesConfig>) -> Self {
        Self(build!(
            zenoh_ext::CacheConfig::default(),
            max_samples,
            replies_config,
        ))
    }
}

wrapper!(zenoh_ext::HistoryConfig: Clone);

#[pymethods]
impl HistoryConfig {
    #[new]
    #[pyo3(signature = (*, detect_late_publishers = None, max_samples = None, max_age = None))]
    fn new(
        detect_late_publishers: Option<bool>,
        max_samples: Option<usize>,
        max_age: Option<f64>,
    ) -> Self {
        let mut config = build!(zenoh_ext::HistoryConfig::default(), max_samples, max_age);
        if matches!(detect_late_publishers, Some(true)) {
            config = config.detect_late_publishers();
        }
        Self(config)
    }
}

wrapper!(zenoh_ext::Miss);

#[pymethods]
impl Miss {
    #[getter]
    fn source(&self) -> EntityGlobalId {
        self.0.source().into()
    }

    #[getter]
    fn nb(&self) -> u32 {
        self.0.nb()
    }
}

wrapper!(zenoh_ext::MissDetectionConfig: Clone);

#[pymethods]
impl MissDetectionConfig {
    #[new]
    #[pyo3(signature = (*, heartbeat = None, sporadic_heartbeat = None))]
    fn new(
        #[pyo3(from_py_with = duration)] heartbeat: Option<Duration>,
        #[pyo3(from_py_with = duration)] sporadic_heartbeat: Option<Duration>,
    ) -> Self {
        Self(build!(
            zenoh_ext::MissDetectionConfig::default(),
            heartbeat,
            sporadic_heartbeat,
        ))
    }
}

wrapper!(zenoh_ext::RecoveryConfig: Clone);

#[pymethods]
impl RecoveryConfig {
    #[new]
    #[pyo3(signature = (*, periodic_queries = None, heartbeat = None))]
    fn new(periodic_queries: Option<Duration>, heartbeat: Option<bool>) -> PyResult<Self> {
        let config = zenoh_ext::RecoveryConfig::default();
        Ok(Self(match (periodic_queries, heartbeat) {
            (Some(periodic_queries), None) => config.periodic_queries(periodic_queries),
            (None, Some(true)) => config.heartbeat(),
            _ => return Err(PyValueError::new_err("invalid parameters")),
        }))
    }
}

wrapper!(zenoh_ext::RepliesConfig: Clone);

#[pymethods]
impl RepliesConfig {
    #[new]
    #[pyo3(signature = (*, congestion_control = None, priority = None, express = None))]
    fn new(
        congestion_control: Option<CongestionControl>,
        priority: Option<Priority>,
        express: Option<bool>,
    ) -> Self {
        Self(build!(
            zenoh_ext::RepliesConfig::default(),
            congestion_control,
            priority,
            express,
            express,
        ))
    }
}

option_wrapper!(
    zenoh_ext::SampleMissListener<HandlerImpl<Miss>>,
    "Undeclared sample-miss listener"
);

#[pymethods]
impl SampleMissListener {
    #[classmethod]
    fn __class_getitem__(cls: &Bound<PyType>, args: &Bound<PyAny>) -> PyObject {
        generic(cls, args)
    }

    fn __enter__<'a, 'py>(this: &'a Bound<'py, Self>) -> &'a Bound<'py, Self> {
        this
    }

    #[pyo3(signature = (*_args, **_kwargs))]
    fn __exit__(
        &mut self,
        py: Python,
        _args: &Bound<PyTuple>,
        _kwargs: Option<&Bound<PyDict>>,
    ) -> PyResult<PyObject> {
        self.undeclare(py)?;
        Ok(py.None())
    }

    fn try_recv(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.try_recv(py)
    }

    fn recv(&self, py: Python) -> PyResult<PyObject> {
        self.get_ref()?.recv(py)
    }

    fn undeclare(&mut self, py: Python) -> PyResult<()> {
        wait(py, self.take()?.undeclare())
    }

    fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        (&**self.get_ref()?).into_pyobject(py)?.try_iter()
    }
}

#[allow(clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(signature = (session, key_expr, *, encoding = None, congestion_control = None, priority = None, express = None, reliability = None, allowed_destination = None, cache = None, sample_miss_detection = None, publisher_detection = None))]
pub(crate) fn declare_advanced_publisher(
    py: Python,
    session: &Session,
    #[pyo3(from_py_with = KeyExpr::from_py)] key_expr: KeyExpr,
    #[pyo3(from_py_with = Encoding::from_py_opt)] encoding: Option<Encoding>,
    congestion_control: Option<CongestionControl>,
    priority: Option<Priority>,
    express: Option<bool>,
    reliability: Option<Reliability>,
    allowed_destination: Option<Locality>,
    cache: Option<CacheConfig>,
    sample_miss_detection: Option<MissDetectionConfig>,
    publisher_detection: Option<bool>,
) -> PyResult<AdvancedPublisher> {
    let mut builder = build!(
        session.0.declare_publisher(key_expr).advanced(),
        encoding,
        congestion_control,
        priority,
        express,
        reliability,
        allowed_destination,
        cache,
        sample_miss_detection,
    );
    if matches!(publisher_detection, Some(true)) {
        builder = builder.publisher_detection();
    }
    wait(py, builder).map_into()
}

#[allow(clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(signature = (session, key_expr, handler = None, *, allowed_origin = None, history = None, recovery = None, subscriber_detection = None))]
pub(crate) fn declare_advanced_subscriber(
    session: &Session,
    py: Python,
    #[pyo3(from_py_with = KeyExpr::from_py)] key_expr: KeyExpr,
    handler: Option<&Bound<PyAny>>,
    allowed_origin: Option<Locality>,
    history: Option<HistoryConfig>,
    recovery: Option<RecoveryConfig>,
    subscriber_detection: Option<bool>,
) -> PyResult<AdvancedSubscriber> {
    let (handler, background) = into_handler(py, handler, None)?;
    let mut builder = build!(
        session.0.declare_subscriber(key_expr).advanced(),
        allowed_origin,
        history,
        recovery
    );
    if matches!(subscriber_detection, Some(true)) {
        builder = builder.subscriber_detection();
    }
    let mut subscriber = wait(py, builder.with(handler))?;
    if background {
        subscriber.set_background(true);
    }
    Ok(subscriber.into())
}
