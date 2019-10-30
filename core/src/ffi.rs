/*
* Copyright 2019 Comcast Cable Communications Management, LLC
*
* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at
*
* http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*
* SPDX-License-Identifier: Apache-2.0
*/

pub use capsule_ffi::*;

use crate::dpdk::DpdkError;
use crate::{warn, Result};
use std::ffi::{CStr, CString};
use std::os::raw;
use std::ptr::NonNull;

/// Simplify `*const c_char` or [c_char] to `&str` conversion.
pub trait AsStr {
    fn as_str(&self) -> &str;
}

impl AsStr for *const raw::c_char {
    #[inline]
    fn as_str(&self) -> &str {
        unsafe {
            CStr::from_ptr(*self).to_str().unwrap_or_else(|_| {
                warn!("invalid UTF8 data");
                Default::default()
            })
        }
    }
}

impl AsStr for [raw::c_char] {
    #[inline]
    fn as_str(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.as_ptr()).to_str().unwrap_or_else(|_| {
                warn!("invalid UTF8 data");
                Default::default()
            })
        }
    }
}

/// Simplify `String` and `&str` to `CString` conversion.
pub trait ToCString {
    fn to_cstring(self) -> CString;
}

impl ToCString for String {
    #[inline]
    fn to_cstring(self) -> CString {
        CString::new(self).unwrap()
    }
}

impl ToCString for &str {
    #[inline]
    fn to_cstring(self) -> CString {
        CString::new(self).unwrap()
    }
}

/// Simplify FFI binding's return to `Result` conversion.
pub trait ToResult {
    type Ok;

    fn to_result(self) -> Result<Self::Ok>;
}

impl ToResult for raw::c_int {
    type Ok = u32;

    #[inline]
    fn to_result(self) -> Result<Self::Ok> {
        match self {
            -1 => Err(DpdkError::new().into()),
            err if err < 0 => Err(DpdkError::new_with_errno(-err).into()),
            _ => Ok(self as u32),
        }
    }
}

impl<T> ToResult for *mut T {
    type Ok = NonNull<T>;

    #[inline]
    fn to_result(self) -> Result<Self::Ok> {
        NonNull::new(self).ok_or_else(|| DpdkError::new().into())
    }
}