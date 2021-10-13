use lazy_static::lazy_static;
use std::ffi::CStr;
use std::sync::{Arc, Mutex};

/// Log level enumeration.
///
/// This enumeration represents the different log levels supported.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum LogLevel {
    /// No messages
    Disabled,
    /// Fatal level messages
    Fatal,
    /// Error level messages
    Error,
    /// Warn level messages
    Warn,
    /// Info level messages
    Info,
    /// Debug level messages
    Debug,
    /// Message level messages
    Message,
    /// Trace level messages
    Trace,
    /// All messages
    All,
    #[doc(hidden)]
    __Unknown(u32),
}

impl From<ffi::otc_log_level> for LogLevel {
    fn from(value: ffi::otc_log_level) -> Self {
        match value {
            ffi::otc_log_level_OTC_LOG_LEVEL_DISABLED => Self::Disabled,
            ffi::otc_log_level_OTC_LOG_LEVEL_FATAL => Self::Fatal,
            ffi::otc_log_level_OTC_LOG_LEVEL_ERROR => Self::Error,
            ffi::otc_log_level_OTC_LOG_LEVEL_WARN => Self::Warn,
            ffi::otc_log_level_OTC_LOG_LEVEL_INFO => Self::Info,
            ffi::otc_log_level_OTC_LOG_LEVEL_DEBUG => Self::Debug,
            ffi::otc_log_level_OTC_LOG_LEVEL_MSG => Self::Message,
            ffi::otc_log_level_OTC_LOG_LEVEL_TRACE => Self::Trace,
            ffi::otc_log_level_OTC_LOG_LEVEL_ALL => Self::All,
            _ => Self::__Unknown(value),
        }
    }
}

impl From<LogLevel> for ffi::otc_log_level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Disabled => ffi::otc_log_level_OTC_LOG_LEVEL_DISABLED,
            LogLevel::Fatal => ffi::otc_log_level_OTC_LOG_LEVEL_FATAL,
            LogLevel::Error => ffi::otc_log_level_OTC_LOG_LEVEL_ERROR,
            LogLevel::Warn => ffi::otc_log_level_OTC_LOG_LEVEL_WARN,
            LogLevel::Info => ffi::otc_log_level_OTC_LOG_LEVEL_INFO,
            LogLevel::Debug => ffi::otc_log_level_OTC_LOG_LEVEL_DEBUG,
            LogLevel::Message => ffi::otc_log_level_OTC_LOG_LEVEL_MSG,
            LogLevel::Trace => ffi::otc_log_level_OTC_LOG_LEVEL_TRACE,
            LogLevel::All => ffi::otc_log_level_OTC_LOG_LEVEL_ALL,
            LogLevel::__Unknown(_) => ffi::otc_log_level_OTC_LOG_LEVEL_DISABLED,
        }
    }
}

pub fn enable_log(level: LogLevel) {
    unsafe { ffi::otc_log_enable(level.into()) };
}

pub type LoggerCallback = Box<dyn Fn(&str) + Send + Sync + 'static>;

lazy_static! {
    pub static ref LOGGER_CALLBACKS: Arc<Mutex<Vec<LoggerCallback>>> = Default::default();
}

unsafe extern "C" fn ffi_logger_callback(message: *const ::std::os::raw::c_char) {
    let message: &CStr = CStr::from_ptr(message);
    let message: &str = message.to_str().unwrap();
    for ref callback in LOGGER_CALLBACKS.lock().unwrap().iter() {
        callback(message);
    }
}

pub fn logger_callback(callback: LoggerCallback) {
    LOGGER_CALLBACKS.lock().unwrap().push(callback);
    unsafe { ffi::otc_log_set_logger_callback(Some(ffi_logger_callback)) }
}
