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

/// Errors associated with an OpenTok session.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
#[must_use]
#[repr(u32)]
pub enum OtcSessionError {
    #[error("Authorization failure")]
    AuthorizationFailure,
    #[error("Block country")]
    BlockedCountry,
    #[error("Connection dropped")]
    ConnectionDropped,
    #[error("Connection failed")]
    ConnectionFailed,
    #[error("Connection limit exceeded")]
    ConnectionLimitExceeded,
    #[error("Connection refused")]
    ConnectionRefused,
    #[error("Connection timed out")]
    ConnectionTimedOut,
    #[error("Force unpublish or invalid stream")]
    ForceUnpublishOrInvalidStream,
    #[error("Illegal state")]
    IllegalState,
    #[error("Internal error")]
    InternalError,
    #[error("Invalid session")]
    InvalidSession,
    #[error("Invalid signal type")]
    InvalidSignalType,
    #[error("Not connected")]
    NotConnected,
    #[error("No messaging server")]
    NoMessagingServer,
    #[error("Null or invalid parameter")]
    NullOrInvalidParameter,
    #[error("Publisher not found")]
    PublisherNotFound,
    #[error("Signal data too long")]
    SignalDataTooLong,
    #[error("Singal type too long")]
    SignalTypeTooLong,
    #[error("State failed")]
    StateFailed,
    #[error("Subscriber not found")]
    SubscriberNotFound,
    #[error("Unexpected get session info response")]
    UnexpectedGetSessionInfoResponse,
    #[error("Unknown error")]
    __Unknown,
}

impl From<ffi::otc_session_error_code> for OtcSessionError {
    fn from(value: ffi::otc_session_error_code) -> OtcSessionError {
        match value {
            ffi::otc_session_error_code_OTC_SESSION_AUTHORIZATION_FAILURE => {
                OtcSessionError::AuthorizationFailure
            }
            ffi::otc_session_error_code_OTC_SESSION_BLOCKED_COUNTRY => {
                OtcSessionError::BlockedCountry
            }
            ffi::otc_session_error_code_OTC_SESSION_CONNECTION_DROPPED => {
                OtcSessionError::ConnectionDropped
            }
            ffi::otc_session_error_code_OTC_SESSION_CONNECTION_FAILED => {
                OtcSessionError::ConnectionFailed
            }
            ffi::otc_session_error_code_OTC_SESSION_CONNECTION_LIMIT_EXCEEDED => {
                OtcSessionError::ConnectionLimitExceeded
            }
            ffi::otc_session_error_code_OTC_SESSION_CONNECTION_REFUSED => {
                OtcSessionError::ConnectionRefused
            }
            ffi::otc_session_error_code_OTC_SESSION_CONNECTION_TIMED_OUT => {
                OtcSessionError::ConnectionTimedOut
            }
            ffi::otc_session_error_code_OTC_SESSION_FORCE_UNPUBLISH_OR_INVALID_STREAM => {
                OtcSessionError::ForceUnpublishOrInvalidStream
            }
            ffi::otc_session_error_code_OTC_SESSION_ILLEGAL_STATE => OtcSessionError::IllegalState,
            ffi::otc_session_error_code_OTC_SESSION_INTERNAL_ERROR => {
                OtcSessionError::InternalError
            }
            ffi::otc_session_error_code_OTC_SESSION_INVALID_SESSION => {
                OtcSessionError::InvalidSession
            }
            ffi::otc_session_error_code_OTC_SESSION_INVALID_SIGNAL_TYPE => {
                OtcSessionError::InvalidSignalType
            }
            ffi::otc_session_error_code_OTC_SESSION_NOT_CONNECTED => OtcSessionError::NotConnected,
            ffi::otc_session_error_code_OTC_SESSION_NO_MESSAGING_SERVER => {
                OtcSessionError::NoMessagingServer
            }
            ffi::otc_session_error_code_OTC_SESSION_NULL_OR_INVALID_PARAMETER => {
                OtcSessionError::NullOrInvalidParameter
            }
            ffi::otc_session_error_code_OTC_SESSION_PUBLISHER_NOT_FOUND => {
                OtcSessionError::PublisherNotFound
            }
            ffi::otc_session_error_code_OTC_SESSION_SIGNAL_DATA_TOO_LONG => {
                OtcSessionError::SignalDataTooLong
            }
            ffi::otc_session_error_code_OTC_SESSION_SIGNAL_TYPE_TOO_LONG => {
                OtcSessionError::SignalTypeTooLong
            }
            ffi::otc_session_error_code_OTC_SESSION_STATE_FAILED => OtcSessionError::StateFailed,
            ffi::otc_session_error_code_OTC_SESSION_SUBSCRIBER_NOT_FOUND => {
                OtcSessionError::SubscriberNotFound
            }
            ffi::otc_session_error_code_OTC_SESSION_UNEXPECTED_GET_SESSION_INFO_REPONSE => {
                OtcSessionError::UnexpectedGetSessionInfoResponse
            }
            _ => OtcSessionError::__Unknown,
        }
    }
}
