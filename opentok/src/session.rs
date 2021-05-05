use crate::enums::{IntoResult, OtcError, OtcResult};

extern "C" fn on_session_connected(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
) {
    println!("on_session_connected");
}

extern "C" fn on_session_reconnection_started(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
) {
    println!("on_session_reconnection_started");
}

extern "C" fn on_session_reconnected(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
) {
    println!("on_session_reconnected");
}

extern "C" fn on_session_disconnected(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
) {
    println!("on_session_disconnected");
}

extern "C" fn on_session_connection_created(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
    connection: *const ffi::otc_connection,
) {
    println!("on_session_connection_created");
}

extern "C" fn on_session_connection_dropped(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
    connection: *const ffi::otc_connection,
) {
    println!("on_session_connection_dropped");
}

extern "C" fn on_session_stream_received(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
    stream: *const ffi::otc_stream,
) {
}

extern "C" fn on_session_stream_dropped(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
    stream: *const ffi::otc_stream,
) {
}

extern "C" fn on_session_stream_has_audio_changed(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
    stream: *const ffi::otc_stream,
    has_audio: ffi::otc_bool,
) {
}

extern "C" fn on_session_stream_has_video_changed(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
    stream: *const ffi::otc_stream,
    has_video: ffi::otc_bool,
) {
}

extern "C" fn on_session_stream_video_dimensions_changed(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
    stream: *const ffi::otc_stream,
    width: ::std::os::raw::c_int,
    height: ::std::os::raw::c_int,
) {

}

extern "C" fn on_session_stream_video_type_changed(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
    stream: *const ffi::otc_stream,
    type_: ffi::otc_stream_video_type,

) {
}

extern "C" fn on_session_signal_received(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
    type_: *const ::std::os::raw::c_char,
    signal: *const ::std::os::raw::c_char,
    connection: *const ffi::otc_connection,
) {
}

extern "C" fn on_session_archive_started(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
    archive_id: *const ::std::os::raw::c_char,
    name: *const ::std::os::raw::c_char,
) {
}

extern "C" fn on_session_archive_stopped(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
    archive_id: *const ::std::os::raw::c_char,
) {
}

extern "C" fn on_session_error(
    session: *mut ffi::otc_session,
    user_data: *mut ::std::os::raw::c_void,
    error_string: *const ::std::os::raw::c_char,
    error: ffi::otc_session_error_code,
) {
    println!("on_session_error");
}

pub struct Session {
    session_ptr: *mut ffi::otc_session,
}

impl Session {
    pub fn new(api_key: &str, session_id: &str) -> Result<Session, OtcError> {
        let api_key = std::ffi::CString::new(api_key).map_err(|_| OtcError::NullError)?;
        let session_id = std::ffi::CString::new(session_id).map_err(|_| OtcError::NullError)?;
        let callbacks = ffi::otc_session_callbacks {
            on_connected: Some(on_session_connected),
            on_reconnection_started: Some(on_session_reconnection_started),
            on_reconnected: Some(on_session_reconnected),
            on_disconnected: Some(on_session_disconnected),
            on_connection_created: Some(on_session_connection_created),
            on_connection_dropped: Some(on_session_connection_dropped),
            on_stream_received: Some(on_session_stream_received),
            on_stream_dropped: Some(on_session_stream_dropped),
            on_stream_has_audio_changed: Some(on_session_stream_has_audio_changed),
            on_stream_has_video_changed: Some(on_session_stream_has_video_changed),
            on_stream_video_dimensions_changed: Some(on_session_stream_video_dimensions_changed),
            on_stream_video_type_changed: Some(on_session_stream_video_type_changed),
            on_signal_received: Some(on_session_signal_received),
            on_archive_started: Some(on_session_archive_started),
            on_archive_stopped: Some(on_session_archive_stopped),
            on_error: Some(on_session_error),
            user_data: std::ptr::null_mut(),
            reserved: std::ptr::null_mut(),
        };
        let session_ptr = unsafe {
            ffi::otc_session_new(api_key.as_ptr(), session_id.as_ptr(), &callbacks)
        };
        if session_ptr.is_null() {
            return Err(OtcError::Fatal);
        }
        Ok(Session { session_ptr })
    }

    pub fn connect(&self, token: &str) -> OtcResult {
        let token = std::ffi::CString::new(token).map_err(|_| OtcError::NullError)?;
        unsafe { ffi::otc_session_connect(self.session_ptr, token.as_ptr()) }.into_result()
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { ffi::otc_session_delete(self.session_ptr) };
    }
}
