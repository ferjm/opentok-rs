use std::ops::Deref;
use thiserror::Error;

/// Log level enumeration.
///
/// This enumeration represents the different log levels supported.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum OtcLogLevel {
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

impl From<ffi::otc_log_level> for OtcLogLevel {
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

/// OpenTok error status codes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
#[must_use]
pub enum OtcError {
    /// An argument used in a function call is not valid
    #[error("An argument used in a function call is not valid")]
    InvalidParam,
    /// Generic error
    #[error("Generic error")]
    Fatal,
    /// The connection to the OpenTok messaging server was dropped. Check the network connection"
    #[error(
        "The connection to the OpenTok messaging server was dropped. Check the network connection"
    )]
    ConnectionDropped,
    /// Timeout while performing a connect action
    #[error("Timeout while performing a connect action")]
    TimedOut,
    /// An unknown Publisher instance was used as a function argument
    #[error("An unknown Publisher instance was used as a function argument")]
    UnknownPublisherInstance,
    /// An unknown Subscriber instance was used as a function argument
    #[error("An unknown Subscriber instance was used as a function argument")]
    UnknownSubscriberInstance,
    /// There was an error with video capturer
    #[error("There was an error with video capturer")]
    VideoCaptureFailed,
    /// There was an error while acquiring video from the camera
    #[error("There was an error while acquiring video from the camera")]
    CameraFailed,
    /// There was an error while rendering video
    #[error("There was an error while rendering video")]
    VideoRenderFailed,
    /// There was an error when trying to get the list of supported video codecs
    #[error("There was an error when trying to get the list of supported video codecs.")]
    UnableToAccessMediaEngine,
    /// Unexpected null pointer
    #[error("Unexpected null pointer")]
    NullError,
    /// Unknown error
    #[doc(hidden)]
    #[error("Unknown error. Life is hard sometimes")]
    __Unknown,
}

pub type OtcResult = Result<(), OtcError>;

pub trait IntoResult {
    fn into_result(&self) -> Result<(), OtcError>;
}

impl IntoResult for ffi::otc_status {
    fn into_result(&self) -> Result<(), OtcError> {
        match *self as u32 {
            ffi::otc_constant_OTC_SUCCESS => Ok(()),
            ffi::otc_error_code_OTC_INVALID_PARAM => Err(OtcError::InvalidParam),
            ffi::otc_error_code_OTC_FATAL => Err(OtcError::Fatal),
            ffi::otc_error_code_OTC_CONNECTION_DROPPED => Err(OtcError::ConnectionDropped),
            ffi::otc_error_code_OTC_CONNECTION_TIMED_OUT => Err(OtcError::TimedOut),
            ffi::otc_error_code_OTC_UNKNOWN_PUBLISHER_INSTANCE => {
                Err(OtcError::UnknownPublisherInstance)
            }
            ffi::otc_error_code_OTC_UNKNOWN_SUBSCRIBER_INSTANCE => {
                Err(OtcError::UnknownSubscriberInstance)
            }
            ffi::otc_error_code_OTC_VIDEO_CAPTURE_FAILED => Err(OtcError::VideoCaptureFailed),
            ffi::otc_error_code_OTC_CAMERA_FAILED => Err(OtcError::CameraFailed),
            ffi::otc_error_code_OTC_VIDEO_RENDER_FAILED => Err(OtcError::VideoRenderFailed),
            ffi::otc_error_code_OT_UNABLE_TO_ACCESS_MEDIA_ENGINE => {
                Err(OtcError::UnableToAccessMediaEngine)
            }
            _ => Err(OtcError::__Unknown),
        }
    }
}

pub struct OtcBool(pub ffi::otc_bool);

impl Deref for OtcBool {
    type Target = bool;

    fn deref(&self) -> &bool {
        match self.0 {
            0 => &false,
            _ => &true,
        }
    }
}