use std::{
    num::NonZeroUsize,
    os::raw::c_int,
    slice, str,
    sync::Arc,
};

use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    ffi,
    prelude::*,
    types::{PyByteArray, PyBytes, PySlice, PyString, PyType},
};
use zenoh::shm::{ChunkAllocResult, PosixShmProviderBackend, ShmBuf};

use crate::{
    buffer::{fill_readonly_u8_buffer, release_u8_buffer},
    macros::{downcast_or_new, wrapper, zerror},
    utils::{wait, IntoPyResult, MapInto},
};

wrapper!(zenoh::shm::AllocAlignment: Clone);

#[pymethods]
impl AllocAlignment {
    #[classattr]
    const ALIGN_1_BYTE: Self = Self(zenoh::shm::AllocAlignment::ALIGN_1_BYTE);
    #[classattr]
    const ALIGN_2_BYTE: Self = Self(zenoh::shm::AllocAlignment::ALIGN_2_BYTES);
    #[classattr]
    const ALIGN_4_BYTE: Self = Self(zenoh::shm::AllocAlignment::ALIGN_4_BYTES);
    #[classattr]
    const ALIGN_8_BYTE: Self = Self(zenoh::shm::AllocAlignment::ALIGN_8_BYTES);

    #[new]
    fn new(pow: u8) -> PyResult<Self> {
        Ok(Self(zenoh::shm::AllocAlignment::new(pow).into_pyres()?))
    }

    fn get_alignment_value(&self) -> NonZeroUsize {
        self.0.get_alignment_value()
    }

    fn align_size(&self, size: NonZeroUsize) -> NonZeroUsize {
        self.0.align_size(size)
    }
}

#[derive(Clone)]
pub struct AllocPolicy(
    Option<Arc<dyn zenoh::shm::AllocPolicy<PosixShmProviderBackend> + Send + Sync>>,
);

impl FromPyObject<'_> for AllocPolicy {
    fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<Self> {
        if ob.is_none() || ob.is_exact_instance_of::<JustAlloc>() {
            Ok(Self(None))
        } else if let Ok(policy) = ob.extract::<BlockOn>() {
            Ok(Self(Some(Arc::new(policy.0))))
        } else if let Ok(policy) = ob.extract::<Deallocate>() {
            Ok(Self(Some(Arc::new(policy.0))))
        } else if let Ok(policy) = ob.extract::<Defragment>() {
            Ok(Self(Some(Arc::new(policy.0))))
        } else if let Ok(policy) = ob.extract::<GarbageCollect>() {
            Ok(Self(Some(Arc::new(policy.0))))
        } else {
            Err(PyTypeError::new_err("expected policy type"))
        }
    }
}

impl zenoh::shm::AllocPolicy<PosixShmProviderBackend> for AllocPolicy {
    fn alloc(
        &self,
        layout: &zenoh::shm::MemoryLayout,
        provider: &zenoh::shm::ShmProvider<PosixShmProviderBackend>,
    ) -> ChunkAllocResult {
        self.0
            .as_deref()
            .unwrap_or(&zenoh::shm::JustAlloc)
            .alloc(layout, provider)
    }
}

wrapper!(zenoh::shm::BlockOn<AllocPolicy>: Clone);

#[pymethods]
impl BlockOn {
    #[new]
    #[pyo3(signature = (inner_policy = AllocPolicy(None)))]
    fn new(inner_policy: AllocPolicy) -> Self {
        Self(zenoh::shm::BlockOn::new(inner_policy))
    }
}

wrapper!(zenoh::shm::Deallocate<usize, AllocPolicy, AllocPolicy>: Clone);

#[pymethods]
impl Deallocate {
    #[new]
    #[pyo3(signature = (limit, inner_policy = AllocPolicy(None), alt_policy = AllocPolicy(None))
    )]
    fn new(limit: usize, inner_policy: AllocPolicy, alt_policy: AllocPolicy) -> Self {
        Self(zenoh::shm::Deallocate::new(limit, inner_policy, alt_policy))
    }
}

wrapper!(zenoh::shm::Defragment<AllocPolicy, AllocPolicy>: Clone);

#[pymethods]
impl Defragment {
    #[new]
    #[pyo3(signature = (inner_policy = AllocPolicy(None), alt_policy = AllocPolicy(None)))]
    fn new(inner_policy: AllocPolicy, alt_policy: AllocPolicy) -> Self {
        Self(zenoh::shm::Defragment::new(inner_policy, alt_policy))
    }
}

wrapper!(zenoh::shm::GarbageCollect<AllocPolicy, AllocPolicy, bool>: Clone);

#[pymethods]
impl GarbageCollect {
    #[new]
    #[pyo3(signature = (inner_policy = AllocPolicy(None), alt_policy = AllocPolicy(None), safe = true)
    )]
    fn new(inner_policy: AllocPolicy, alt_policy: AllocPolicy, safe: bool) -> Self {
        Self(zenoh::shm::GarbageCollect::new(
            inner_policy,
            alt_policy,
            safe,
        ))
    }
}

wrapper!(zenoh::shm::JustAlloc:Clone);

#[pymethods]
impl JustAlloc {
    #[new]
    fn new() -> Self {
        Self(zenoh::shm::JustAlloc)
    }
}

wrapper!(zenoh::shm::MemoryLayout: Clone);
downcast_or_new!(MemoryLayout);

#[pymethods]
impl MemoryLayout {
    #[new]
    fn new(obj: &Bound<PyAny>) -> PyResult<Self> {
        let layout = if let Ok(layout) = obj.extract::<usize>() {
            layout.try_into()
        } else if let Ok((size, layout)) = obj.extract::<(usize, AllocAlignment)>() {
            (size, layout.0).try_into()
        } else {
            return Err(PyTypeError::new_err(
                "expected int/tuple[int, AllocAlignment]",
            ));
        };
        Ok(Self(layout.into_pyres()?))
    }

    #[getter]
    fn size(&self) -> NonZeroUsize {
        self.0.size()
    }

    #[getter]
    fn alignment(&self) -> AllocAlignment {
        AllocAlignment(self.0.alignment())
    }
}

wrapper!(zenoh::shm::ShmProvider<PosixShmProviderBackend>);

#[pymethods]
impl ShmProvider {
    #[classmethod]
    fn default_backend(
        _cls: &Bound<PyType>,
        py: Python,
        #[pyo3(from_py_with = MemoryLayout::from_py)] layout: MemoryLayout,
    ) -> PyResult<Self> {
        let builder = zenoh::shm::ShmProviderBuilder::default_backend(layout.0);
        wait(py, builder).map_into()
    }

    #[pyo3(signature = (layout, policy = AllocPolicy(None)))]
    fn alloc(
        &self,
        py: Python,
        #[pyo3(from_py_with = MemoryLayout::from_py)] layout: MemoryLayout,
        policy: AllocPolicy,
    ) -> PyResult<ZShmMut> {
        // SAFETY: we are in Python...
        let builder = unsafe { self.0.alloc(layout.0).with_runtime_policy(policy) };
        wait(py, builder).map_into()
    }

    fn defragment(&self) {
        self.0.defragment();
    }

    fn garbage_collect(&self) -> usize {
        self.0.garbage_collect()
    }

    fn garbage_collect_unsafe(&self) -> usize {
        // SAFETY: we are in Python...
        unsafe { self.0.garbage_collect_unsafe() }
    }

    #[getter]
    fn available(&self) -> usize {
        self.0.available()
    }
}

wrapper!(zenoh::shm::ZShm);

#[pymethods]
impl ZShm {
    fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    fn __str__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyString>> {
        Ok(PyString::new(py, str::from_utf8(&self.0).into_pyres()?))
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0)
    }

    unsafe fn __getbuffer__(
        slf: Bound<'_, Self>,
        view: *mut ffi::Py_buffer,
        flags: c_int,
    ) -> PyResult<()> {
        let (ptr, len) = {
            let shm = slf.borrow();
            let bytes: &[u8] = shm.0.as_ref();
            (bytes.as_ptr(), bytes.len())
        };
        let bytes = if len == 0 {
            &[]
        } else {
            // SAFETY: `slf` owns the ZShm handle and keeps the mapped SHM
            // buffer alive for at least as long as the exported buffer.
            unsafe { slice::from_raw_parts(ptr, len) }
        };
        unsafe { fill_readonly_u8_buffer(slf.into_any(), bytes, view, flags) }
    }

    unsafe fn __releasebuffer__(&self, view: *mut ffi::Py_buffer) {
        unsafe { release_u8_buffer(view) }
    }
}

#[pyclass]
pub(crate) struct ZShmMut {
    buf: Option<zenoh::shm::ZShmMut>,
}

impl ZShmMut {
    pub(crate) fn get(&self) -> PyResult<&zenoh::shm::ZShmMut> {
        self.buf
            .as_ref()
            .ok_or_else(|| zerror!("ZShmMut has been consumed by ZBytes conversion"))
    }
    fn get_mut(&mut self) -> PyResult<&mut zenoh::shm::ZShmMut> {
        self.get()?;
        Ok(self.buf.as_mut().unwrap())
    }
    pub(crate) fn take(&mut self) -> PyResult<zenoh::shm::ZShmMut> {
        self.get()?;
        Ok(self.buf.take().unwrap())
    }
}

#[pymethods]
impl ZShmMut {
    fn __setitem__(this: &Bound<Self>, key: &Bound<PyAny>, value: &Bound<PyAny>) -> PyResult<()> {
        if let Ok(key) = key.extract::<usize>() {
            if let Ok(value) = value.extract::<u8>() {
                if let Some(entry) = this.borrow_mut().get_mut()?.get_mut(key) {
                    *entry = value;
                    return Ok(());
                }
            }
        } else if let Ok(key) = key.downcast::<PySlice>() {
            let mut buffer = this.borrow_mut();
            let slice = buffer.get_mut()?;
            let indices = key.indices(slice.len() as isize)?;
            let mut copy_bytes = |b: &[u8]| {
                if b.len() != indices.slicelength {
                    return Err(PyValueError::new_err(
                        "memoryview assignment: lvalue and rvalue have different structures",
                    ));
                }
                slice[indices.start as usize..indices.stop as usize].copy_from_slice(b);
                Ok(())
            };
            if let Ok(bytes) = value.downcast::<PyByteArray>() {
                return copy_bytes(unsafe { bytes.as_bytes() });
            } else if let Ok(bytes) = value.downcast::<PyBytes>() {
                return copy_bytes(bytes.as_bytes());
            }
        }
        Err(PyTypeError::new_err("expected bytes like argument"))
    }

    fn __str__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyString>> {
        Ok(PyString::new(py, str::from_utf8(self.get()?).into_pyres()?))
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        Ok(PyBytes::new(py, self.get()?))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.get()?))
    }
}

impl From<zenoh::shm::ZShmMut> for ZShmMut {
    fn from(value: zenoh::shm::ZShmMut) -> Self {
        Self { buf: Some(value) }
    }
}
