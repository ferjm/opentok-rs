extern crate lazy_static;

#[macro_use]
mod macros;

#[macro_use]
pub mod connection;
mod enums;
pub mod session;
pub mod stream;

use crate::enums::{IntoResult, OtcResult};

use std::ptr;

/// Initializes the library. You must call this function before
/// the execution of any other code using the library.
pub fn init() -> OtcResult {
    unsafe { ffi::otc_init(ptr::null_mut()).into_result() }
}

/// Destroys the library engine. You should call this function when you are done
/// executing code that uses the library.
pub fn deinit() -> OtcResult {
    unsafe { ffi::otc_destroy().into_result() }
}
