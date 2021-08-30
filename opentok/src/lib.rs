#![allow(dead_code)]

extern crate lazy_static;

#[macro_use]
mod macros;

#[macro_use]
pub mod connection;
pub mod audio_device;
mod enums;
pub mod log;
pub mod publisher;
pub mod session;
pub mod stream;
pub mod subscriber;
pub mod video_capturer;
pub mod video_frame;

pub use crate::enums::{IntoResult, OtcError, OtcResult};

use std::ptr;

/// Initializes the library. You must call this function before
/// the execution of any other code using the library.
pub fn init() -> OtcResult {
    let result = unsafe { ffi::otc_init(ptr::null_mut()).into_result() };
    audio_device::initialize().map_err(|_| OtcError::Fatal)?;
    result
}

/// Destroys the library engine. You should call this function when you are done
/// executing code that uses the library.
pub fn deinit() -> OtcResult {
    unsafe { ffi::otc_destroy().into_result() }
}
