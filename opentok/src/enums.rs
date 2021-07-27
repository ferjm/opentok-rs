use std::ops::Deref;
use thiserror::Error;

/// OpenTok error status codes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
#[must_use]
#[allow(clippy::manual_non_exhaustive)]
pub enum OtcError {
    /// An argument used in a function call is not valid
    #[error("The argument {0} used in a function call is not valid")]
    InvalidParam(&'static str),
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
    /// Double initilization error.
    #[error("{0} has already been initialized")]
    AlreadyInitialized(&'static str),
    /// Initialization error.
    #[error("Could not initialize {0}: {1}")]
    Initialization(&'static str, &'static str),
    /// Unknown error
    #[doc(hidden)]
    #[error("Unknown error. Life is hard sometimes")]
    __Unknown,
}

pub type OtcResult = Result<(), OtcError>;

pub trait IntoResult {
    fn into_result(self) -> Result<(), OtcError>;
}

impl IntoResult for ffi::otc_status {
    fn into_result(self) -> Result<(), OtcError> {
        match self as u32 {
            ffi::otc_constant_OTC_SUCCESS => Ok(()),
            ffi::otc_error_code_OTC_INVALID_PARAM => Err(OtcError::InvalidParam("")),
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

impl From<OtcResult> for OtcBool {
    fn from(result: OtcResult) -> OtcBool {
        match result {
            Ok(_) => OtcBool(1),
            Err(_) => OtcBool(0),
        }
    }
}

impl From<bool> for OtcBool {
    fn from(value: bool) -> OtcBool {
        match value {
            true => OtcBool(1),
            false => OtcBool(0),
        }
    }
}

impl From<OtcBool> for ffi::otc_bool {
    fn from(value: OtcBool) -> ffi::otc_bool {
        value.0
    }
}
