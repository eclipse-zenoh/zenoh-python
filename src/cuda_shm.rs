//
// Copyright (c) 2026 ZettaScale Technology
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
    collections::BTreeMap,
    ffi::{CStr, CString},
    os::raw::{c_char, c_int, c_uint, c_void},
    ptr,
    sync::{Arc, Mutex},
};

use pyo3::{exceptions::PyRuntimeError, prelude::*};

type CuResult = c_int;
type CuDevice = c_int;
type CuContext = *mut c_void;

const CUDA_SUCCESS: CuResult = 0;
const RTLD_NOW: c_int = 2;

type CuInit = unsafe extern "C" fn(c_uint) -> CuResult;
type CuDeviceGet = unsafe extern "C" fn(*mut CuDevice, c_int) -> CuResult;
type CuDevicePrimaryCtxRetain = unsafe extern "C" fn(*mut CuContext, CuDevice) -> CuResult;
type CuDevicePrimaryCtxRelease = unsafe extern "C" fn(CuDevice) -> CuResult;
type CuCtxSetCurrent = unsafe extern "C" fn(CuContext) -> CuResult;
type CuMemHostRegister = unsafe extern "C" fn(*mut c_void, usize, c_uint) -> CuResult;
type CuMemHostUnregister = unsafe extern "C" fn(*mut c_void) -> CuResult;
type CuGetErrorString = unsafe extern "C" fn(CuResult, *mut *const c_char) -> CuResult;

#[cfg(target_os = "linux")]
#[link(name = "dl")]
unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn sysconf(name: c_int) -> isize;
}

pub(crate) struct CudaDriver {
    _lib: *mut c_void,
    device: CuDevice,
    context: CuContext,
    page_size: usize,
    registered_pages: Mutex<BTreeMap<usize, usize>>,
    cu_device_primary_ctx_release: CuDevicePrimaryCtxRelease,
    cu_ctx_set_current: CuCtxSetCurrent,
    cu_mem_host_register: CuMemHostRegister,
    cu_mem_host_unregister: CuMemHostUnregister,
    cu_get_error_string: CuGetErrorString,
}

unsafe impl Send for CudaDriver {}
unsafe impl Sync for CudaDriver {}

impl CudaDriver {
    pub(crate) fn new(device_index: c_int) -> PyResult<Arc<Self>> {
        if device_index < 0 {
            return Err(PyRuntimeError::new_err(
                "CUDA device index must be non-negative",
            ));
        }
        #[cfg(not(target_os = "linux"))]
        {
            let _ = device_index;
            return Err(PyRuntimeError::new_err(
                "CUDA pinned SHM currently requires Linux libcuda.so.1",
            ));
        }

        #[cfg(target_os = "linux")]
        unsafe {
            let lib_name = CString::new("libcuda.so.1").unwrap();
            let lib = dlopen(lib_name.as_ptr(), RTLD_NOW);
            if lib.is_null() {
                return Err(PyRuntimeError::new_err(
                    "CUDA driver library libcuda.so.1 is unavailable",
                ));
            }

            let cu_init: CuInit = symbol(lib, "cuInit")?;
            let cu_device_get: CuDeviceGet = symbol(lib, "cuDeviceGet")?;
            let cu_device_primary_ctx_retain: CuDevicePrimaryCtxRetain =
                symbol(lib, "cuDevicePrimaryCtxRetain")?;
            let cu_device_primary_ctx_release: CuDevicePrimaryCtxRelease =
                symbol(lib, "cuDevicePrimaryCtxRelease")?;
            let cu_ctx_set_current: CuCtxSetCurrent = symbol(lib, "cuCtxSetCurrent")?;
            let cu_mem_host_register: CuMemHostRegister = symbol(lib, "cuMemHostRegister")?;
            let cu_mem_host_unregister: CuMemHostUnregister = symbol(lib, "cuMemHostUnregister")?;
            let cu_get_error_string: CuGetErrorString = symbol(lib, "cuGetErrorString")?;

            check(cu_init(0), cu_get_error_string, "cuInit")?;

            let mut device = 0;
            check(
                cu_device_get(&mut device, device_index),
                cu_get_error_string,
                "cuDeviceGet",
            )?;

            let mut context = ptr::null_mut();
            check(
                cu_device_primary_ctx_retain(&mut context, device),
                cu_get_error_string,
                "cuDevicePrimaryCtxRetain",
            )?;
            check(
                cu_ctx_set_current(context),
                cu_get_error_string,
                "cuCtxSetCurrent",
            )?;

            Ok(Arc::new(Self {
                _lib: lib,
                device,
                context,
                page_size: page_size(),
                registered_pages: Mutex::new(BTreeMap::new()),
                cu_device_primary_ctx_release,
                cu_ctx_set_current,
                cu_mem_host_register,
                cu_mem_host_unregister,
                cu_get_error_string,
            }))
        }
    }

    pub(crate) fn register(
        self: &Arc<Self>,
        ptr: *mut u8,
        len: usize,
    ) -> PyResult<CudaRegisteredMemory> {
        if len == 0 {
            return Err(PyRuntimeError::new_err(
                "cannot CUDA-register an empty SHM buffer",
            ));
        }
        if ptr.is_null() {
            return Err(PyRuntimeError::new_err(
                "cannot CUDA-register a null SHM buffer",
            ));
        }
        // The upstream ShmProvider/ZShmMut API used here does not expose the
        // full backing segment base/len. Register page-by-page and share
        // overlapping pages within this CUDA driver instead of registering
        // potentially overlapping allocation ranges.
        let page_size = self.page_size;
        let ptr_addr = ptr as usize;
        let start = ptr_addr - (ptr_addr % page_size);
        let last = ptr_addr
            .checked_add(len - 1)
            .ok_or_else(|| PyRuntimeError::new_err("CUDA registration range overflow"))?;
        let end = (last - (last % page_size))
            .checked_add(page_size)
            .ok_or_else(|| PyRuntimeError::new_err("CUDA registration range overflow"))?;
        let mut pages = Vec::new();
        let mut page = start;
        while page < end {
            if let Err(err) = self.acquire_page(page) {
                self.release_pages(&pages);
                return Err(err);
            }
            pages.push(page);
            page = page
                .checked_add(page_size)
                .ok_or_else(|| PyRuntimeError::new_err("CUDA registration range overflow"))?;
        }

        Ok(CudaRegisteredMemory {
            inner: Arc::new(CudaRegisteredMemoryInner {
                driver: self.clone(),
                pages,
            }),
        })
    }

    fn acquire_page(&self, page: usize) -> PyResult<()> {
        let mut registered_pages = self
            .registered_pages
            .lock()
            .map_err(|_| PyRuntimeError::new_err("CUDA registration registry is poisoned"))?;
        if let Some(count) = registered_pages.get_mut(&page) {
            *count += 1;
            return Ok(());
        }

        unsafe {
            check(
                (self.cu_ctx_set_current)(self.context),
                self.cu_get_error_string,
                "cuCtxSetCurrent",
            )?;
            check(
                (self.cu_mem_host_register)(page as *mut c_void, self.page_size, 0),
                self.cu_get_error_string,
                "cuMemHostRegister",
            )?;
        }
        registered_pages.insert(page, 1);
        Ok(())
    }

    fn release_pages(&self, pages: &[usize]) {
        let Ok(mut registered_pages) = self.registered_pages.lock() else {
            eprintln!("CUDA registration registry is poisoned during ZShmPool cleanup");
            return;
        };
        for page in pages {
            let Some(count) = registered_pages.get_mut(page) else {
                continue;
            };
            if *count > 1 {
                *count -= 1;
                continue;
            }
            registered_pages.remove(page);
            unsafe {
                let _ = (self.cu_ctx_set_current)(self.context);
                let rc = (self.cu_mem_host_unregister)(*page as *mut c_void);
                if rc != CUDA_SUCCESS {
                    eprintln!(
                        "CUDA cuMemHostUnregister failed during ZShmPool cleanup: {}",
                        cuda_error_message(rc, self.cu_get_error_string)
                    );
                }
            }
        }
    }
}

impl Drop for CudaDriver {
    fn drop(&mut self) {
        unsafe {
            let _ = (self.cu_ctx_set_current)(self.context);
            let _ = (self.cu_device_primary_ctx_release)(self.device);
        }
    }
}

#[derive(Clone)]
pub(crate) struct CudaRegisteredMemory {
    #[allow(dead_code)]
    inner: Arc<CudaRegisteredMemoryInner>,
}

struct CudaRegisteredMemoryInner {
    driver: Arc<CudaDriver>,
    pages: Vec<usize>,
}

unsafe impl Send for CudaRegisteredMemory {}
unsafe impl Sync for CudaRegisteredMemory {}

impl Drop for CudaRegisteredMemoryInner {
    fn drop(&mut self) {
        self.driver.release_pages(&self.pages);
    }
}

#[cfg(target_os = "linux")]
unsafe fn symbol<T: Copy>(lib: *mut c_void, name: &str) -> PyResult<T> {
    let c_name = CString::new(name).unwrap();
    let ptr = unsafe { dlsym(lib, c_name.as_ptr()) };
    if ptr.is_null() {
        return Err(PyRuntimeError::new_err(format!(
            "CUDA driver symbol {name} is unavailable"
        )));
    }
    Ok(unsafe { std::mem::transmute_copy(&ptr) })
}

unsafe fn check(
    rc: CuResult,
    cu_get_error_string: CuGetErrorString,
    operation: &str,
) -> PyResult<()> {
    if rc == CUDA_SUCCESS {
        Ok(())
    } else {
        Err(PyRuntimeError::new_err(format!(
            "{operation} failed: {}",
            cuda_error_message(rc, cu_get_error_string)
        )))
    }
}

#[cfg(target_os = "linux")]
fn page_size() -> usize {
    const SC_PAGESIZE: c_int = 30;
    let value = unsafe { sysconf(SC_PAGESIZE) };
    if value > 0 {
        value as usize
    } else {
        4096
    }
}

#[cfg(not(target_os = "linux"))]
fn page_size() -> usize {
    4096
}

fn cuda_error_message(rc: CuResult, cu_get_error_string: CuGetErrorString) -> String {
    let mut ptr = ptr::null();
    let lookup = unsafe { cu_get_error_string(rc, &mut ptr) };
    if lookup == CUDA_SUCCESS && !ptr.is_null() {
        let msg = unsafe { CStr::from_ptr(ptr) }.to_string_lossy();
        format!("CUDA error {rc}: {msg}")
    } else {
        format!("CUDA error {rc}")
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use std::{
        os::raw::c_void,
        sync::{Arc, Mutex, OnceLock},
    };

    use super::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(crate) enum Event {
        Register(usize, usize),
        Unregister(usize),
    }

    static EVENTS: OnceLock<Mutex<Vec<Event>>> = OnceLock::new();
    static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn events() -> &'static Mutex<Vec<Event>> {
        EVENTS.get_or_init(|| Mutex::new(Vec::new()))
    }

    pub(crate) fn test_lock() -> std::sync::MutexGuard<'static, ()> {
        TEST_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    pub(crate) fn take_events() -> Vec<Event> {
        std::mem::take(&mut *events().lock().unwrap())
    }

    unsafe extern "C" fn fake_ctx_set_current(_context: CuContext) -> CuResult {
        CUDA_SUCCESS
    }

    unsafe extern "C" fn fake_ctx_release(_device: CuDevice) -> CuResult {
        CUDA_SUCCESS
    }

    unsafe extern "C" fn fake_register(
        ptr: *mut c_void,
        len: usize,
        _flags: c_uint,
    ) -> CuResult {
        events()
            .lock()
            .unwrap()
            .push(Event::Register(ptr as usize, len));
        CUDA_SUCCESS
    }

    unsafe extern "C" fn fake_unregister(ptr: *mut c_void) -> CuResult {
        events()
            .lock()
            .unwrap()
            .push(Event::Unregister(ptr as usize));
        CUDA_SUCCESS
    }

    unsafe extern "C" fn fake_error_string(
        _rc: CuResult,
        out: *mut *const c_char,
    ) -> CuResult {
        unsafe {
            *out = c"fake cuda error".as_ptr();
        }
        CUDA_SUCCESS
    }

    pub(crate) fn fake_driver() -> Arc<CudaDriver> {
        take_events();
        Arc::new(CudaDriver {
            _lib: ptr::null_mut(),
            device: 0,
            context: ptr::null_mut(),
            page_size: 4096,
            registered_pages: Mutex::new(BTreeMap::new()),
            cu_device_primary_ctx_release: fake_ctx_release,
            cu_ctx_set_current: fake_ctx_set_current,
            cu_mem_host_register: fake_register,
            cu_mem_host_unregister: fake_unregister,
            cu_get_error_string: fake_error_string,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::{fake_driver, take_events, test_lock, Event};

    #[test]
    fn overlapping_page_registrations_are_shared_until_last_owner_drops() {
        let _guard = test_lock();
        let driver = fake_driver();
        let first = driver.register(0x1008usize as *mut u8, 16).unwrap();
        let second = driver.register(0x1080usize as *mut u8, 16).unwrap();

        assert_eq!(take_events(), vec![Event::Register(0x1000, 4096)]);

        drop(first);
        assert_eq!(take_events(), Vec::<Event>::new());

        drop(second);
        assert_eq!(take_events(), vec![Event::Unregister(0x1000)]);
    }

    #[test]
    fn cloned_registration_keeps_pages_pinned_until_last_clone_drops() {
        let _guard = test_lock();
        let driver = fake_driver();
        let registration = driver.register(0x2008usize as *mut u8, 16).unwrap();
        assert_eq!(take_events(), vec![Event::Register(0x2000, 4096)]);

        let clone = registration.clone();
        drop(registration);
        assert_eq!(take_events(), Vec::<Event>::new());

        drop(clone);
        assert_eq!(take_events(), vec![Event::Unregister(0x2000)]);
    }
}
