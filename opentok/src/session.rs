use crate::connection::Connection;
use crate::enums::{IntoResult, OtcBool, OtcError, OtcResult};
use crate::publisher::Publisher;
use crate::stream::{Stream, StreamVideoType};

use once_cell::sync::OnceCell;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::sync::{Arc, Mutex};
use thiserror::Error;

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

ffi_callback!(on_connected, *mut ffi::otc_session, Session);
ffi_callback!(on_reconnection_started, *mut ffi::otc_session, Session);
ffi_callback!(on_reconnected, *mut ffi::otc_session, Session);
ffi_callback!(on_disconnected, *mut ffi::otc_session, Session);
ffi_callback!(
    on_connection_created,
    *mut ffi::otc_session,
    Session,
    *const ffi::otc_connection
);
ffi_callback!(
    on_connection_dropped,
    *mut ffi::otc_session,
    Session,
    *const ffi::otc_connection
);
ffi_callback!(
    on_stream_received,
    *mut ffi::otc_session,
    Session,
    *const ffi::otc_stream
);
ffi_callback!(
    on_stream_dropped,
    *mut ffi::otc_session,
    Session,
    *const ffi::otc_stream
);
ffi_callback!(
    on_stream_has_audio_changed,
    *mut ffi::otc_session,
    Session,
    *const ffi::otc_stream,
    ffi::otc_bool
);
ffi_callback!(
    on_stream_has_video_changed,
    *mut ffi::otc_session,
    Session,
    *const ffi::otc_stream,
    ffi::otc_bool
);
ffi_callback!(
    on_stream_video_dimensions_changed,
    *mut ffi::otc_session,
    Session,
    *const ffi::otc_stream,
    i32,
    i32
);
ffi_callback!(
    on_stream_video_type_changed,
    *mut ffi::otc_session,
    Session,
    *const ffi::otc_stream,
    ffi::otc_stream_video_type
);
ffi_callback!(
    on_signal_received,
    *mut ffi::otc_session,
    Session,
    *const c_char,
    *const c_char,
    *const ffi::otc_connection
);
ffi_callback!(
    on_archive_started,
    *mut ffi::otc_session,
    Session,
    *const c_char,
    *const c_char
);
ffi_callback!(
    on_archive_stopped,
    *mut ffi::otc_session,
    Session,
    *const c_char
);
ffi_callback!(
    on_error,
    *mut ffi::otc_session,
    Session,
    *const c_char,
    ffi::otc_session_error_code
);

#[allow(clippy::type_complexity)]
pub struct SessionCallbacks {
    on_connected: Option<Box<dyn Fn()>>,
    on_reconnection_started: Option<Box<dyn Fn()>>,
    on_reconnected: Option<Box<dyn Fn()>>,
    on_disconnected: Option<Box<dyn Fn()>>,
    on_connection_created: Option<Box<dyn Fn(Connection)>>,
    on_connection_dropped: Option<Box<dyn Fn(Connection)>>,
    on_stream_received: Option<Box<dyn Fn(Stream)>>,
    on_stream_dropped: Option<Box<dyn Fn(Stream)>>,
    on_stream_has_audio_changed: Option<Box<dyn Fn(Stream, bool)>>,
    on_stream_has_video_changed: Option<Box<dyn Fn(Stream, bool)>>,
    on_stream_video_dimensions_changed: Option<Box<dyn Fn(Stream, i32, i32)>>,
    on_stream_video_type_changed: Option<Box<dyn Fn(Stream, StreamVideoType)>>,
    on_signal_received: Option<Box<dyn Fn(&str, &str, Connection)>>,
    on_archive_started: Option<Box<dyn Fn(&str, &str)>>,
    on_archive_stopped: Option<Box<dyn Fn(&str)>>,
    on_error: Option<Box<dyn Fn(&str, OtcSessionError)>>,
}

impl SessionCallbacks {
    pub fn builder() -> SessionCallbacksBuilder {
        SessionCallbacksBuilder {
            on_connected: None,
            on_reconnection_started: None,
            on_reconnected: None,
            on_disconnected: None,
            on_connection_created: None,
            on_connection_dropped: None,
            on_stream_received: None,
            on_stream_dropped: None,
            on_stream_has_audio_changed: None,
            on_stream_has_video_changed: None,
            on_stream_video_dimensions_changed: None,
            on_stream_video_type_changed: None,
            on_signal_received: None,
            on_archive_started: None,
            on_archive_stopped: None,
            on_error: None,
        }
    }

    callback!(on_connected);
    callback!(on_reconnection_started);
    callback!(on_reconnected);
    callback!(on_disconnected);
    callback!(on_connection_created, connection, Connection);
    callback!(on_connection_dropped, connection, Connection);
    callback!(on_stream_received, stream, Stream);
    callback!(on_stream_dropped, stream, Stream);

    pub fn on_stream_has_audio_changed(&self, stream: Stream, has_audio: bool) {
        if let Some(ref callback) = self.on_stream_has_audio_changed {
            callback(stream, has_audio);
        }
    }

    pub fn on_stream_has_video_changed(&self, stream: Stream, has_video: bool) {
        if let Some(ref callback) = self.on_stream_has_video_changed {
            callback(stream, has_video);
        }
    }

    callback!(
        on_stream_video_dimensions_changed,
        stream,
        Stream,
        width,
        i32,
        height,
        i32
    );
    callback!(
        on_stream_video_type_changed,
        stream,
        Stream,
        type_,
        StreamVideoType
    );

    pub fn on_signal_received(&self, type_: &str, signal: &str, connection: Connection) {
        if let Some(ref callback) = self.on_signal_received {
            callback(type_, signal, connection)
        }
    }

    pub fn on_archive_started(&self, archive_id: &str, name: &str) {
        if let Some(ref callback) = self.on_archive_started {
            callback(archive_id, name);
        }
    }

    pub fn on_archive_stopped(&self, archive_id: &str) {
        if let Some(ref callback) = self.on_archive_stopped {
            callback(archive_id);
        }
    }

    pub fn on_error(&self, error_string: &str, error: OtcSessionError) {
        if let Some(ref callback) = self.on_error {
            callback(error_string, error);
        }
    }
}

#[allow(clippy::type_complexity)]
pub struct SessionCallbacksBuilder {
    on_connected: Option<Box<dyn Fn()>>,
    on_reconnection_started: Option<Box<dyn Fn()>>,
    on_reconnected: Option<Box<dyn Fn()>>,
    on_disconnected: Option<Box<dyn Fn()>>,
    on_connection_created: Option<Box<dyn Fn(Connection)>>,
    on_connection_dropped: Option<Box<dyn Fn(Connection)>>,
    on_stream_received: Option<Box<dyn Fn(Stream)>>,
    on_stream_dropped: Option<Box<dyn Fn(Stream)>>,
    on_stream_has_audio_changed: Option<Box<dyn Fn(Stream, bool)>>,
    on_stream_has_video_changed: Option<Box<dyn Fn(Stream, bool)>>,
    on_stream_video_dimensions_changed: Option<Box<dyn Fn(Stream, i32, i32)>>,
    on_stream_video_type_changed: Option<Box<dyn Fn(Stream, StreamVideoType)>>,
    on_signal_received: Option<Box<dyn Fn(&str, &str, Connection)>>,
    on_archive_started: Option<Box<dyn Fn(&str, &str)>>,
    on_archive_stopped: Option<Box<dyn Fn(&str)>>,
    on_error: Option<Box<dyn Fn(&str, OtcSessionError)>>,
}

impl SessionCallbacksBuilder {
    callback_setter!(on_connected);
    callback_setter!(on_reconnection_started);
    callback_setter!(on_reconnected);
    callback_setter!(on_disconnected);
    callback_setter!(on_connection_created, Connection);
    callback_setter!(on_connection_dropped, Connection);
    callback_setter!(on_stream_received, Stream);
    callback_setter!(on_stream_dropped, Stream);
    callback_setter!(on_stream_has_audio_changed, Stream, bool);
    callback_setter!(on_stream_has_video_changed, Stream, bool);
    callback_setter!(on_stream_video_dimensions_changed, Stream, i32, i32);
    callback_setter!(on_stream_video_type_changed, Stream, StreamVideoType);
    callback_setter!(on_signal_received, &str, &str, Connection);
    callback_setter!(on_archive_started, &str, &str);
    callback_setter!(on_archive_stopped, &str);
    callback_setter!(on_error, &str, OtcSessionError);

    pub fn build(self) -> SessionCallbacks {
        SessionCallbacks {
            on_connected: self.on_connected,
            on_reconnection_started: self.on_reconnection_started,
            on_reconnected: self.on_reconnected,
            on_disconnected: self.on_disconnected,
            on_connection_created: self.on_connection_created,
            on_connection_dropped: self.on_connection_dropped,
            on_stream_received: self.on_stream_received,
            on_stream_dropped: self.on_stream_dropped,
            on_stream_has_audio_changed: self.on_stream_has_audio_changed,
            on_stream_has_video_changed: self.on_stream_has_video_changed,
            on_stream_video_dimensions_changed: self.on_stream_video_dimensions_changed,
            on_stream_video_type_changed: self.on_stream_video_type_changed,
            on_signal_received: self.on_signal_received,
            on_archive_started: self.on_archive_started,
            on_archive_stopped: self.on_archive_stopped,
            on_error: self.on_error,
        }
    }
}

#[derive(Clone)]
pub struct Session {
    ptr: OnceCell<*const ffi::otc_session>,
    callbacks: Arc<Mutex<SessionCallbacks>>,
    ffi_callbacks: OnceCell<ffi::otc_session_callbacks>,
}

impl Session {
    /// Creates a new OpenTok session.
    ///
    /// * api_key: Your OpenTok API key. You can get it from https://tokbox.com/account
    /// * session_id: The identifier of the session.
    /// * callbacks: An instance of SessionCallbacks containing the handlers for events
    /// related to the session.
    pub fn new(
        api_key: &str,
        session_id: &str,
        callbacks: SessionCallbacks,
    ) -> Result<Session, OtcError> {
        let mut session = Session {
            ptr: Default::default(),
            callbacks: Arc::new(Mutex::new(callbacks)),
            ffi_callbacks: Default::default(),
        };

        let session_ptr: *mut c_void = &mut session as *mut _ as *mut c_void;
        let ffi_callbacks = ffi::otc_session_callbacks {
            on_connected: Some(on_connected),
            on_reconnection_started: Some(on_reconnection_started),
            on_reconnected: Some(on_reconnected),
            on_disconnected: Some(on_disconnected),
            on_connection_created: Some(on_connection_created),
            on_connection_dropped: Some(on_connection_dropped),
            on_stream_received: Some(on_stream_received),
            on_stream_dropped: Some(on_stream_dropped),
            on_stream_has_audio_changed: Some(on_stream_has_audio_changed),
            on_stream_has_video_changed: Some(on_stream_has_video_changed),
            on_stream_video_dimensions_changed: Some(on_stream_video_dimensions_changed),
            on_stream_video_type_changed: Some(on_stream_video_type_changed),
            on_signal_received: Some(on_signal_received),
            on_archive_started: Some(on_archive_started),
            on_archive_stopped: Some(on_archive_stopped),
            on_error: Some(on_error),
            user_data: session_ptr,
            reserved: std::ptr::null_mut(),
        };

        let api_key = CString::new(api_key).map_err(|_| OtcError::InvalidParam("api_key"))?;
        let session_id =
            CString::new(session_id).map_err(|_| OtcError::InvalidParam("session_id"))?;
        let session_ptr =
            unsafe { ffi::otc_session_new(api_key.as_ptr(), session_id.as_ptr(), &ffi_callbacks) };
        if session_ptr.is_null() {
            return Err(OtcError::Fatal);
        }

        session.ptr.set(session_ptr).map_err(|_| OtcError::Fatal)?;
        session
            .ffi_callbacks
            .set(ffi_callbacks)
            .map_err(|_| OtcError::Fatal)?;

        Ok(session)
    }

    /// Connects a client to an OpenTok session.
    ///
    /// * token - The client token for connecting to the session. Check
    /// https://tokbox.com/developer/guides/create-token/
    pub fn connect(&self, token: &str) -> OtcResult {
        let token = std::ffi::CString::new(token).map_err(|_| OtcError::InvalidParam("token"))?;
        unsafe { ffi::otc_session_connect(*self.ptr.get().unwrap() as *mut _, token.as_ptr()) }
            .into_result()
    }

    /// Disconnects the client from this session. All of the client's subscribers
    /// and publishers will also be will be disconnected from the session.
    pub fn disconnect(&self) -> OtcResult {
        unsafe { ffi::otc_session_disconnect(*self.ptr.get().unwrap() as *mut _) }.into_result()
    }

    /// Releases resources associated with the session.
    pub fn delete(&self) -> OtcResult {
        unsafe { ffi::otc_session_delete(*self.ptr.get().unwrap() as *mut _) }.into_result()
    }

    pub fn publish(&self, publisher: Publisher) -> OtcResult {
        unsafe {
            ffi::otc_session_publish(*self.ptr.get().unwrap() as *mut _, *publisher as *mut _)
        }
        .into_result()
    }

    callback_call!(on_connected);
    callback_call!(on_reconnection_started);
    callback_call!(on_reconnected);
    callback_call!(on_disconnected);
    callback_call!(on_stream_received, *const ffi::otc_stream);
    callback_call!(on_stream_dropped, *const ffi::otc_stream);

    fn on_connection_created(&self, connection: *const ffi::otc_connection) {
        let connection = unsafe { ffi::otc_connection_copy(connection) };
        if connection.is_null() {
            return;
        }
        self.callbacks
            .lock()
            .unwrap()
            .on_connection_created((connection as *const ffi::otc_connection).into())
    }

    fn on_connection_dropped(&self, connection: *const ffi::otc_connection) {
        let connection = unsafe { ffi::otc_connection_copy(connection) };
        if connection.is_null() {
            return;
        }
        self.callbacks
            .lock()
            .unwrap()
            .on_connection_dropped((connection as *const ffi::otc_connection).into())
       
    }

    fn on_stream_has_audio_changed(
        &self,
        stream: *const ffi::otc_stream,
        has_audio: ffi::otc_bool,
    ) {
        self.callbacks
            .lock()
            .unwrap()
            .on_stream_has_audio_changed(stream.into(), *OtcBool(has_audio))
    }

    fn on_stream_has_video_changed(
        &self,
        stream: *const ffi::otc_stream,
        has_video: ffi::otc_bool,
    ) {
        self.callbacks
            .lock()
            .unwrap()
            .on_stream_has_video_changed(stream.into(), *OtcBool(has_video))
    }

    callback_call!(
        on_stream_video_dimensions_changed,
        *const ffi::otc_stream,
        i32,
        i32
    );
    callback_call!(
        on_stream_video_type_changed,
        *const ffi::otc_stream,
        ffi::otc_stream_video_type
    );

    fn on_signal_received(
        &self,
        type_: *const c_char,
        signal: *const c_char,
        connection: *const ffi::otc_connection,
    ) {
        let type_ = unsafe { CStr::from_ptr(type_) };
        let signal = unsafe { CStr::from_ptr(signal) };
        let connection = unsafe { ffi::otc_connection_copy(connection) };
        if connection.is_null() {
            return;
        }
        self.callbacks.lock().unwrap().on_signal_received(
            type_.to_str().unwrap_or_default(),
            signal.to_str().unwrap_or_default(),
            (connection as *const ffi::otc_connection).into(),
        );
    }

    fn on_archive_started(&self, archive_id: *const c_char, name: *const c_char) {
        let archive_id = unsafe { CStr::from_ptr(archive_id) };
        let name = unsafe { CStr::from_ptr(name) };
        self.callbacks.lock().unwrap().on_archive_started(
            archive_id.to_str().unwrap_or_default(),
            name.to_str().unwrap_or_default(),
        );
    }

    fn on_archive_stopped(&self, archive_id: *const c_char) {
        let archive_id = unsafe { CStr::from_ptr(archive_id) };
        self.callbacks
            .lock()
            .unwrap()
            .on_archive_stopped(archive_id.to_str().unwrap_or_default());
    }

    fn on_error(&self, error_string: *const c_char, error: ffi::otc_session_error_code) {
        let error_string = unsafe { CStr::from_ptr(error_string) };
        self.callbacks
            .lock()
            .unwrap()
            .on_error(error_string.to_str().unwrap_or_default(), error.into());
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { ffi::otc_session_delete(*self.ptr.get().unwrap() as *mut _) };
    }
}
