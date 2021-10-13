use crate::connection::Connection;
use crate::enums::{IntoResult, OtcBool, OtcError, OtcResult};
use crate::publisher::Publisher;
use crate::stream::{Stream, StreamVideoType};
use crate::subscriber::Subscriber;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::{Arc, Mutex};
use thiserror::Error;

lazy_static! {
    pub static ref INSTANCES: Arc<Mutex<HashMap<usize, Session>>> = Default::default();
}

/// Errors associated with an OpenTok session.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Error)]
#[must_use]
#[repr(u32)]
pub enum SessionError {
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

impl From<ffi::otc_session_error_code> for SessionError {
    fn from(value: ffi::otc_session_error_code) -> SessionError {
        match value {
            ffi::otc_session_error_code_OTC_SESSION_AUTHORIZATION_FAILURE => {
                SessionError::AuthorizationFailure
            }
            ffi::otc_session_error_code_OTC_SESSION_BLOCKED_COUNTRY => SessionError::BlockedCountry,
            ffi::otc_session_error_code_OTC_SESSION_CONNECTION_DROPPED => {
                SessionError::ConnectionDropped
            }
            ffi::otc_session_error_code_OTC_SESSION_CONNECTION_FAILED => {
                SessionError::ConnectionFailed
            }
            ffi::otc_session_error_code_OTC_SESSION_CONNECTION_LIMIT_EXCEEDED => {
                SessionError::ConnectionLimitExceeded
            }
            ffi::otc_session_error_code_OTC_SESSION_CONNECTION_REFUSED => {
                SessionError::ConnectionRefused
            }
            ffi::otc_session_error_code_OTC_SESSION_CONNECTION_TIMED_OUT => {
                SessionError::ConnectionTimedOut
            }
            ffi::otc_session_error_code_OTC_SESSION_FORCE_UNPUBLISH_OR_INVALID_STREAM => {
                SessionError::ForceUnpublishOrInvalidStream
            }
            ffi::otc_session_error_code_OTC_SESSION_ILLEGAL_STATE => SessionError::IllegalState,
            ffi::otc_session_error_code_OTC_SESSION_INTERNAL_ERROR => SessionError::InternalError,
            ffi::otc_session_error_code_OTC_SESSION_INVALID_SESSION => SessionError::InvalidSession,
            ffi::otc_session_error_code_OTC_SESSION_INVALID_SIGNAL_TYPE => {
                SessionError::InvalidSignalType
            }
            ffi::otc_session_error_code_OTC_SESSION_NOT_CONNECTED => SessionError::NotConnected,
            ffi::otc_session_error_code_OTC_SESSION_NO_MESSAGING_SERVER => {
                SessionError::NoMessagingServer
            }
            ffi::otc_session_error_code_OTC_SESSION_NULL_OR_INVALID_PARAMETER => {
                SessionError::NullOrInvalidParameter
            }
            ffi::otc_session_error_code_OTC_SESSION_PUBLISHER_NOT_FOUND => {
                SessionError::PublisherNotFound
            }
            ffi::otc_session_error_code_OTC_SESSION_SIGNAL_DATA_TOO_LONG => {
                SessionError::SignalDataTooLong
            }
            ffi::otc_session_error_code_OTC_SESSION_SIGNAL_TYPE_TOO_LONG => {
                SessionError::SignalTypeTooLong
            }
            ffi::otc_session_error_code_OTC_SESSION_STATE_FAILED => SessionError::StateFailed,
            ffi::otc_session_error_code_OTC_SESSION_SUBSCRIBER_NOT_FOUND => {
                SessionError::SubscriberNotFound
            }
            ffi::otc_session_error_code_OTC_SESSION_UNEXPECTED_GET_SESSION_INFO_REPONSE => {
                SessionError::UnexpectedGetSessionInfoResponse
            }
            _ => SessionError::__Unknown,
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
    on_connected: Option<Box<dyn Fn(&Session) + Send + Sync + 'static>>,
    on_reconnection_started: Option<Box<dyn Fn(&Session) + Send + Sync + 'static>>,
    on_reconnected: Option<Box<dyn Fn(&Session) + Send + Sync + 'static>>,
    on_disconnected: Option<Box<dyn Fn(&Session) + Send + Sync + 'static>>,
    on_connection_created: Option<Box<dyn Fn(&Session, Connection) + Send + Sync + 'static>>,
    on_connection_dropped: Option<Box<dyn Fn(&Session, Connection) + Send + Sync + 'static>>,
    on_stream_received: Option<Box<dyn Fn(&Session, Stream) + Send + Sync + 'static>>,
    on_stream_dropped: Option<Box<dyn Fn(&Session, Stream) + Send + Sync + 'static>>,
    on_stream_has_audio_changed:
        Option<Box<dyn Fn(&Session, Stream, bool) + Send + Sync + 'static>>,
    on_stream_has_video_changed:
        Option<Box<dyn Fn(&Session, Stream, bool) + Send + Sync + 'static>>,
    on_stream_video_dimensions_changed:
        Option<Box<dyn Fn(&Session, Stream, i32, i32) + Send + Sync + 'static>>,
    on_stream_video_type_changed:
        Option<Box<dyn Fn(&Session, Stream, StreamVideoType) + Send + Sync + 'static>>,
    on_signal_received:
        Option<Box<dyn Fn(&Session, &str, &str, Connection) + Send + Sync + 'static>>,
    on_archive_started: Option<Box<dyn Fn(&Session, &str, &str) + Send + Sync + 'static>>,
    on_archive_stopped: Option<Box<dyn Fn(&Session, &str) + Send + Sync + 'static>>,
    on_error: Option<Box<dyn Fn(&Session, &str, SessionError) + Send + Sync + 'static>>,
}

impl SessionCallbacks {
    pub fn builder() -> SessionCallbacksBuilder {
        SessionCallbacksBuilder::default()
    }

    callback!(on_connected, &Session);
    callback!(on_reconnection_started, &Session);
    callback!(on_reconnected, &Session);
    callback!(on_disconnected, &Session);
    callback!(on_connection_created, &Session, Connection);
    callback!(on_connection_dropped, &Session, Connection);
    callback!(on_stream_received, &Session, Stream);
    callback!(on_stream_dropped, &Session, Stream);

    pub fn on_stream_has_audio_changed(&self, session: &Session, stream: Stream, has_audio: bool) {
        if let Some(ref callback) = self.on_stream_has_audio_changed {
            callback(session, stream, has_audio);
        }
    }

    pub fn on_stream_has_video_changed(&self, session: &Session, stream: Stream, has_video: bool) {
        if let Some(ref callback) = self.on_stream_has_video_changed {
            callback(session, stream, has_video);
        }
    }

    callback!(
        on_stream_video_dimensions_changed,
        &Session,
        Stream,
        i32,
        i32
    );
    callback!(
        on_stream_video_type_changed,
        &Session,
        Stream,
        StreamVideoType
    );

    pub fn on_signal_received(
        &self,
        session: &Session,
        type_: &str,
        signal: &str,
        connection: Connection,
    ) {
        if let Some(ref callback) = self.on_signal_received {
            callback(session, type_, signal, connection)
        }
    }

    pub fn on_archive_started(&self, session: &Session, archive_id: &str, name: &str) {
        if let Some(ref callback) = self.on_archive_started {
            callback(session, archive_id, name);
        }
    }

    pub fn on_archive_stopped(&self, session: &Session, archive_id: &str) {
        if let Some(ref callback) = self.on_archive_stopped {
            callback(session, archive_id);
        }
    }

    pub fn on_error(&self, session: &Session, error_string: &str, error: SessionError) {
        if let Some(ref callback) = self.on_error {
            callback(session, error_string, error);
        }
    }
}

#[derive(Default)]
#[allow(clippy::type_complexity)]
pub struct SessionCallbacksBuilder {
    on_connected: Option<Box<dyn Fn(&Session) + Send + Sync + 'static>>,
    on_reconnection_started: Option<Box<dyn Fn(&Session) + Send + Sync + 'static>>,
    on_reconnected: Option<Box<dyn Fn(&Session) + Send + Sync + 'static>>,
    on_disconnected: Option<Box<dyn Fn(&Session) + Send + Sync + 'static>>,
    on_connection_created: Option<Box<dyn Fn(&Session, Connection) + Send + Sync + 'static>>,
    on_connection_dropped: Option<Box<dyn Fn(&Session, Connection) + Send + Sync + 'static>>,
    on_stream_received: Option<Box<dyn Fn(&Session, Stream) + Send + Sync + 'static>>,
    on_stream_dropped: Option<Box<dyn Fn(&Session, Stream) + Send + Sync + 'static>>,
    on_stream_has_audio_changed:
        Option<Box<dyn Fn(&Session, Stream, bool) + Send + Sync + 'static>>,
    on_stream_has_video_changed:
        Option<Box<dyn Fn(&Session, Stream, bool) + Send + Sync + 'static>>,
    on_stream_video_dimensions_changed:
        Option<Box<dyn Fn(&Session, Stream, i32, i32) + Send + Sync + 'static>>,
    on_stream_video_type_changed:
        Option<Box<dyn Fn(&Session, Stream, StreamVideoType) + Send + Sync + 'static>>,
    on_signal_received:
        Option<Box<dyn Fn(&Session, &str, &str, Connection) + Send + Sync + 'static>>,
    on_archive_started: Option<Box<dyn Fn(&Session, &str, &str) + Send + Sync + 'static>>,
    on_archive_stopped: Option<Box<dyn Fn(&Session, &str) + Send + Sync + 'static>>,
    on_error: Option<Box<dyn Fn(&Session, &str, SessionError) + Send + Sync + 'static>>,
}

impl SessionCallbacksBuilder {
    callback_setter!(on_connected, &Session);
    callback_setter!(on_reconnection_started, &Session);
    callback_setter!(on_reconnected, &Session);
    callback_setter!(on_disconnected, &Session);
    callback_setter!(on_connection_created, &Session, Connection);
    callback_setter!(on_connection_dropped, &Session, Connection);
    callback_setter!(on_stream_received, &Session, Stream);
    callback_setter!(on_stream_dropped, &Session, Stream);
    callback_setter!(on_stream_has_audio_changed, &Session, Stream, bool);
    callback_setter!(on_stream_has_video_changed, &Session, Stream, bool);
    callback_setter!(
        on_stream_video_dimensions_changed,
        &Session,
        Stream,
        i32,
        i32
    );
    callback_setter!(
        on_stream_video_type_changed,
        &Session,
        Stream,
        StreamVideoType
    );
    callback_setter!(on_signal_received, &Session, &str, &str, Connection);
    callback_setter!(on_archive_started, &Session, &str, &str);
    callback_setter!(on_archive_stopped, &Session, &str);
    callback_setter!(on_error, &Session, &str, SessionError);

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
    ptr: Arc<AtomicPtr<*mut ffi::otc_session>>,
    callbacks: Arc<Mutex<SessionCallbacks>>,
}

unsafe impl Send for Session {}
unsafe impl Sync for Session {}

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
        let api_key = CString::new(api_key).map_err(|_| OtcError::InvalidParam("api_key"))?;
        let session_id =
            CString::new(session_id).map_err(|_| OtcError::InvalidParam("session_id"))?;
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
            user_data: std::ptr::null_mut(),
            reserved: std::ptr::null_mut(),
        };
        let session_ptr =
            unsafe { ffi::otc_session_new(api_key.as_ptr(), session_id.as_ptr(), &ffi_callbacks) };
        if session_ptr.is_null() {
            return Err(OtcError::NullError);
        }
        let session = Session {
            ptr: Arc::new(AtomicPtr::new(session_ptr as *mut _)),
            callbacks: Arc::new(Mutex::new(callbacks)),
        };
        INSTANCES
            .lock()
            .unwrap()
            .insert(session_ptr as usize, session.clone());
        Ok(session)
    }

    /// Connects a client to an OpenTok session.
    ///
    /// * token - The client token for connecting to the session. Check
    /// https://tokbox.com/developer/guides/create-token/
    pub fn connect(&self, token: &str) -> OtcResult {
        if self.ptr.load(Ordering::Relaxed).is_null() {
            return Err(OtcError::NullError);
        }
        let token = std::ffi::CString::new(token).map_err(|_| OtcError::InvalidParam("token"))?;
        unsafe {
            ffi::otc_session_connect(self.ptr.load(Ordering::Relaxed) as *mut _, token.as_ptr())
        }
        .into_result()
    }

    /// Disconnects the client from this session. All of the client's subscribers
    /// and publishers will also be will be disconnected from the session.
    pub fn disconnect(&self) -> OtcResult {
        if self.ptr.load(Ordering::Relaxed).is_null() {
            return Err(OtcError::NullError);
        }
        unsafe { ffi::otc_session_disconnect(self.ptr.load(Ordering::Relaxed) as *mut _) }
            .into_result()
    }

    pub fn publish(&self, publisher: &Publisher) -> OtcResult {
        if self.ptr.load(Ordering::Relaxed).is_null() {
            return Err(OtcError::NullError);
        }
        unsafe {
            ffi::otc_session_publish(
                self.ptr.load(Ordering::Relaxed) as *mut _,
                publisher.inner() as *mut _,
            )
        }
        .into_result()
    }

    pub fn subscribe(&self, subscriber: &Subscriber) -> OtcResult {
        let ptr = self.ptr.load(Ordering::Relaxed);
        if ptr.is_null() {
            return Err(OtcError::NullError);
        }
        unsafe { ffi::otc_session_subscribe(ptr as *mut _, subscriber.inner() as *mut _) }
            .into_result()
    }

    callback_call!(on_connected);
    callback_call!(on_connection_created, *const ffi::otc_connection);
    callback_call!(on_connection_dropped, *const ffi::otc_connection);
    callback_call!(on_reconnection_started);
    callback_call!(on_reconnected);
    callback_call!(on_disconnected);
    callback_call!(on_stream_received, *const ffi::otc_stream);
    callback_call!(on_stream_dropped, *const ffi::otc_stream);
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

    fn on_stream_has_audio_changed(
        &self,
        stream: *const ffi::otc_stream,
        has_audio: ffi::otc_bool,
    ) {
        if stream.is_null() {
            return;
        }
        let stream = unsafe { ffi::otc_stream_copy(stream) };
        if let Ok(callbacks) = self.callbacks.try_lock() {
            callbacks.on_stream_has_audio_changed(
                self,
                (stream as *const ffi::otc_stream).into(),
                *OtcBool(has_audio),
            )
        }
    }

    fn on_stream_has_video_changed(
        &self,
        stream: *const ffi::otc_stream,
        has_video: ffi::otc_bool,
    ) {
        if stream.is_null() {
            return;
        }
        let stream = unsafe { ffi::otc_stream_copy(stream) };
        if let Ok(callbacks) = self.callbacks.try_lock() {
            callbacks.on_stream_has_video_changed(
                self,
                (stream as *const ffi::otc_stream).into(),
                *OtcBool(has_video),
            )
        }
    }

    fn on_signal_received(
        &self,
        type_: *const c_char,
        signal: *const c_char,
        connection: *const ffi::otc_connection,
    ) {
        if type_.is_null() || signal.is_null() || connection.is_null() {
            return;
        }
        let type_ = unsafe { CStr::from_ptr(type_) };
        let signal = unsafe { CStr::from_ptr(signal) };
        if let Ok(callbacks) = self.callbacks.try_lock() {
            callbacks.on_signal_received(
                self,
                type_.to_str().unwrap_or_default(),
                signal.to_str().unwrap_or_default(),
                (connection as *const ffi::otc_connection).into(),
            );
        }
    }

    fn on_archive_started(&self, archive_id: *const c_char, name: *const c_char) {
        if archive_id.is_null() || name.is_null() {
            return;
        }
        let archive_id = unsafe { CStr::from_ptr(archive_id) };
        let name = unsafe { CStr::from_ptr(name) };
        if let Ok(callbacks) = self.callbacks.try_lock() {
            callbacks.on_archive_started(
                self,
                archive_id.to_str().unwrap_or_default(),
                name.to_str().unwrap_or_default(),
            );
        }
    }

    fn on_archive_stopped(&self, archive_id: *const c_char) {
        if archive_id.is_null() {
            return;
        }
        let archive_id = unsafe { CStr::from_ptr(archive_id) };
        if let Ok(callbacks) = self.callbacks.try_lock() {
            callbacks.on_archive_stopped(self, archive_id.to_str().unwrap_or_default());
        }
    }

    fn on_error(&self, error_string: *const c_char, error: ffi::otc_session_error_code) {
        if error_string.is_null() {
            return;
        }
        let error_string = unsafe { CStr::from_ptr(error_string) };
        if let Ok(callbacks) = self.callbacks.try_lock() {
            callbacks.on_error(
                self,
                error_string.to_str().unwrap_or_default(),
                error.into(),
            );
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let ptr = self.ptr.load(Ordering::Relaxed);

        // 2 because we keep a reference in INSTANCES.
        if Arc::strong_count(&self.ptr) > 2 {
            return;
        }

        if ptr.is_null() {
            return;
        }

        self.ptr.store(std::ptr::null_mut(), Ordering::Relaxed);

        unsafe {
            ffi::otc_session_delete(ptr as *mut _);
        }

        if let Ok(ref mut instances) = INSTANCES.try_lock() {
            instances.remove(&(ptr as usize));
        }
    }
}
