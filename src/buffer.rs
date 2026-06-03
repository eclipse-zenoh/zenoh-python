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
    ffi::CString,
    os::raw::{c_int, c_void},
    ptr,
};

use pyo3::{exceptions::PyBufferError, ffi, prelude::*};

/// Populate `view` as a read-only, single-byte, C-contiguous buffer over
/// `data`, transferring ownership of `owner` into the exported view so the
/// backing storage stays alive while the consumer holds the buffer.
///
/// # Safety
/// `data` must remain valid for as long as `owner` keeps the buffer alive, and
/// `view` must point to a valid `Py_buffer` provided by the buffer protocol.
pub(crate) unsafe fn fill_readonly_u8_buffer(
    owner: Bound<'_, PyAny>,
    data: &[u8],
    view: *mut ffi::Py_buffer,
    flags: c_int,
) -> PyResult<()> {
    if view.is_null() {
        return Err(PyBufferError::new_err("view is null"));
    }
    if flags & ffi::PyBUF_WRITABLE == ffi::PyBUF_WRITABLE {
        return Err(PyBufferError::new_err("object is not writable"));
    }

    unsafe {
        (*view).obj = owner.into_ptr();
        (*view).buf = data.as_ptr() as *mut c_void;
        (*view).len = data.len() as ffi::Py_ssize_t;
        (*view).readonly = 1;
        (*view).itemsize = 1;
        (*view).format = if flags & ffi::PyBUF_FORMAT == ffi::PyBUF_FORMAT {
            CString::new("B").unwrap().into_raw()
        } else {
            ptr::null_mut()
        };
        (*view).ndim = 1;
        (*view).shape = if flags & ffi::PyBUF_ND == ffi::PyBUF_ND {
            &mut (*view).len
        } else {
            ptr::null_mut()
        };
        (*view).strides = if flags & ffi::PyBUF_STRIDES == ffi::PyBUF_STRIDES {
            &mut (*view).itemsize
        } else {
            ptr::null_mut()
        };
        (*view).suboffsets = ptr::null_mut();
        (*view).internal = ptr::null_mut();
    }
    Ok(())
}

/// Release the format string allocated by [`fill_readonly_u8_buffer`].
///
/// # Safety
/// `view` must be a `Py_buffer` previously populated by
/// [`fill_readonly_u8_buffer`].
pub(crate) unsafe fn release_u8_buffer(view: *mut ffi::Py_buffer) {
    unsafe {
        if !view.is_null() && !(*view).format.is_null() {
            drop(CString::from_raw((*view).format));
        }
    }
}
