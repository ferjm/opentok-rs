use crate::enums::{IntoResult, OtcBool, OtcError, OtcResult};
use crate::stream::Stream;
use crate::video_frame::VideoFrame;

use once_cell::unsync::OnceCell;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::sync::{Arc, Mutex};

/// All possible Subscriber errors.
pub enum SubscriberError {
    /// Internal error.
    Internal,
    /// Tried to subscribe to a disconnected session.
    SessionDisconnected,
    /// The subscriber failed because the stream is missing. This can happen
    /// if the subscriber is created at the same time the stream is removed
    /// from the session.
    ServerCannotFindStream,
    /// The client tired to subscribe to a stream in a session that has
    /// exceeded the limit for simultaneous streams.
    StreamLimitExceeded,
    /// Timed out attempting to subscribe to a stream.
    TimedOut,
    /// WebRTC error.
    WebRtcError,
    /// Unknown subscriber error.
    __Unknown,
}

impl From<ffi::otc_subscriber_error_code> for SubscriberError {
    fn from(type_: ffi::otc_subscriber_error_code) -> SubscriberError {
        match type_ {
            ffi::otc_subscriber_error_code_OTC_SUBSCRIBER_INTERNAL_ERROR => {
                SubscriberError::Internal
            }
            ffi::otc_subscriber_error_code_OTC_SUBSCRIBER_SESSION_DISCONNECTED => {
                SubscriberError::SessionDisconnected
            }
            ffi::otc_subscriber_error_code_OTC_SUBSCRIBER_SERVER_CANNOT_FIND_STREAM => {
                SubscriberError::ServerCannotFindStream
            }
            ffi::otc_subscriber_error_code_OTC_SUBSCRIBER_STREAM_LIMIT_EXCEEDED => {
                SubscriberError::StreamLimitExceeded
            }
            ffi::otc_subscriber_error_code_OTC_SUBSCRIBER_TIMED_OUT => SubscriberError::TimedOut,
            ffi::otc_subscriber_error_code_OTC_SUBSCRIBER_WEBRTC_ERROR => {
                SubscriberError::WebRtcError
            }
            _ => SubscriberError::__Unknown,
        }
    }
}

/// Reasons for a video to be started, stopped, resumed, etc.
pub enum VideoReason {
    Publish,
    Subscribe,
    Quality,
    CodecNotSupported,
    __Unknown,
}

impl From<ffi::otc_video_reason> for VideoReason {
    fn from(reason: ffi::otc_video_reason) -> VideoReason {
        match reason {
            ffi::otc_video_reason_OTC_VIDEO_REASON_PUBLISH_VIDEO => VideoReason::Publish,
            ffi::otc_video_reason_OTC_VIDEO_REASON_SUBSCRIBE_TO_VIDEO => VideoReason::Subscribe,
            ffi::otc_video_reason_OTC_VIDEO_REASON_QUALITY => VideoReason::Quality,
            ffi::otc_video_reason_OTC_VIDEO_REASON_CODEC_NOT_SUPPORTED => {
                VideoReason::CodecNotSupported
            }
            _ => VideoReason::__Unknown,
        }
    }
}

ffi_callback!(
    on_connected,
    *mut ffi::otc_subscriber,
    Subscriber,
    *const ffi::otc_stream
);

ffi_callback!(on_disconnected, *mut ffi::otc_subscriber, Subscriber);

ffi_callback!(on_reconnected, *mut ffi::otc_subscriber, Subscriber);

ffi_callback!(
    on_render_frame,
    *mut ffi::otc_subscriber,
    Subscriber,
    *const ffi::otc_video_frame
);

ffi_callback!(
    on_video_disabled,
    *mut ffi::otc_subscriber,
    Subscriber,
    ffi::otc_video_reason
);

ffi_callback!(
    on_video_enabled,
    *mut ffi::otc_subscriber,
    Subscriber,
    ffi::otc_video_reason
);

ffi_callback!(on_audio_disabled, *mut ffi::otc_subscriber, Subscriber);

ffi_callback!(on_audio_enabled, *mut ffi::otc_subscriber, Subscriber);

ffi_callback!(on_video_data_received, *mut ffi::otc_subscriber, Subscriber);

ffi_callback!(
    on_video_disable_warning,
    *mut ffi::otc_subscriber,
    Subscriber
);

ffi_callback!(
    on_video_disable_warning_lifted,
    *mut ffi::otc_subscriber,
    Subscriber
);

ffi_callback!(
    on_audio_level_updated,
    *mut ffi::otc_subscriber,
    Subscriber,
    f32
);

ffi_callback!(
    on_error,
    *mut ffi::otc_subscriber,
    Subscriber,
    *const c_char,
    ffi::otc_subscriber_error_code
);

#[allow(clippy::type_complexity)]
pub struct SubscriberCallbacks {
    on_connected: Option<Box<dyn Fn(Subscriber, Stream)>>,
    on_disconnected: Option<Box<dyn Fn(Subscriber)>>,
    on_reconnected: Option<Box<dyn Fn(Subscriber)>>,
    on_render_frame: Option<Box<dyn Fn(Subscriber, VideoFrame)>>,
    on_video_disabled: Option<Box<dyn Fn(Subscriber, VideoReason)>>,
    on_video_enabled: Option<Box<dyn Fn(Subscriber, VideoReason)>>,
    on_audio_disabled: Option<Box<dyn Fn(Subscriber)>>,
    on_audio_enabled: Option<Box<dyn Fn(Subscriber)>>,
    on_video_data_received: Option<Box<dyn Fn(Subscriber)>>,
    on_video_disable_warning: Option<Box<dyn Fn(Subscriber)>>,
    on_video_disable_warning_lifted: Option<Box<dyn Fn(Subscriber)>>,
    on_audio_level_updated: Option<Box<dyn Fn(Subscriber, f32)>>,
    on_error: Option<Box<dyn Fn(Subscriber, &str, SubscriberError)>>,
}

impl SubscriberCallbacks {
    pub fn builder() -> SubscriberCallbacksBuilder {
        SubscriberCallbacksBuilder::default()
    }

    callback!(on_connected, Subscriber, Stream);
    callback!(on_disconnected, Subscriber);
    callback!(on_reconnected, Subscriber);
    callback!(on_render_frame, Subscriber, VideoFrame);
    callback!(on_video_disabled, Subscriber, VideoReason);
    callback!(on_video_enabled, Subscriber, VideoReason);
    callback!(on_audio_disabled, Subscriber);
    callback!(on_audio_enabled, Subscriber);
    callback!(on_video_data_received, Subscriber);
    callback!(on_video_disable_warning, Subscriber);
    callback!(on_video_disable_warning_lifted, Subscriber);
    callback!(on_audio_level_updated, Subscriber, f32);
    callback!(on_error, Subscriber, &str, SubscriberError);
}

#[derive(Default)]
#[allow(clippy::type_complexity)]
pub struct SubscriberCallbacksBuilder {
    on_connected: Option<Box<dyn Fn(Subscriber, Stream)>>,
    on_disconnected: Option<Box<dyn Fn(Subscriber)>>,
    on_reconnected: Option<Box<dyn Fn(Subscriber)>>,
    on_render_frame: Option<Box<dyn Fn(Subscriber, VideoFrame)>>,
    on_video_disabled: Option<Box<dyn Fn(Subscriber, VideoReason)>>,
    on_video_enabled: Option<Box<dyn Fn(Subscriber, VideoReason)>>,
    on_audio_disabled: Option<Box<dyn Fn(Subscriber)>>,
    on_audio_enabled: Option<Box<dyn Fn(Subscriber)>>,
    on_video_data_received: Option<Box<dyn Fn(Subscriber)>>,
    on_video_disable_warning: Option<Box<dyn Fn(Subscriber)>>,
    on_video_disable_warning_lifted: Option<Box<dyn Fn(Subscriber)>>,
    on_audio_level_updated: Option<Box<dyn Fn(Subscriber, f32)>>,
    on_error: Option<Box<dyn Fn(Subscriber, &str, SubscriberError)>>,
}

impl SubscriberCallbacksBuilder {
    callback_setter!(on_connected, Subscriber, Stream);
    callback_setter!(on_disconnected, Subscriber);
    callback_setter!(on_reconnected, Subscriber);
    callback_setter!(on_render_frame, Subscriber, VideoFrame);
    callback_setter!(on_video_disabled, Subscriber, VideoReason);
    callback_setter!(on_video_enabled, Subscriber, VideoReason);
    callback_setter!(on_audio_disabled, Subscriber);
    callback_setter!(on_audio_enabled, Subscriber);
    callback_setter!(on_video_data_received, Subscriber);
    callback_setter!(on_video_disable_warning, Subscriber);
    callback_setter!(on_video_disable_warning_lifted, Subscriber);
    callback_setter!(on_audio_level_updated, Subscriber, f32);
    callback_setter!(on_error, Subscriber, &str, SubscriberError);

    pub fn build(self) -> SubscriberCallbacks {
        SubscriberCallbacks {
            on_connected: self.on_connected,
            on_disconnected: self.on_disconnected,
            on_reconnected: self.on_reconnected,
            on_render_frame: self.on_render_frame,
            on_video_disabled: self.on_video_disabled,
            on_video_enabled: self.on_video_enabled,
            on_audio_disabled: self.on_audio_disabled,
            on_audio_enabled: self.on_audio_enabled,
            on_video_data_received: self.on_video_data_received,
            on_video_disable_warning: self.on_video_disable_warning,
            on_video_disable_warning_lifted: self.on_video_disable_warning_lifted,
            on_audio_level_updated: self.on_audio_level_updated,
            on_error: self.on_error,
        }
    }
}

#[derive(Clone)]
pub struct Subscriber {
    ptr: OnceCell<*const ffi::otc_subscriber>,
    callbacks: Arc<Mutex<SubscriberCallbacks>>,
    ffi_callbacks: OnceCell<ffi::otc_subscriber_callbacks>,
    stream: Stream,
}

impl Subscriber {
    pub fn new(stream: Stream, callbacks: SubscriberCallbacks) -> Self {
        let mut subscriber = Self {
            ptr: Default::default(),
            callbacks: Arc::new(Mutex::new(callbacks)),
            ffi_callbacks: Default::default(),
            stream: stream.clone(),
        };
        let subscriber_ptr: *mut c_void = &mut subscriber as *mut _ as *mut c_void;
        let ffi_callbacks = ffi::otc_subscriber_callbacks {
            on_connected: Some(on_connected),
            on_disconnected: Some(on_disconnected),
            on_reconnected: Some(on_reconnected),
            on_render_frame: Some(on_render_frame),
            on_video_disabled: Some(on_video_disabled),
            on_video_enabled: Some(on_video_enabled),
            on_audio_disabled: Some(on_audio_disabled),
            on_audio_enabled: Some(on_audio_enabled),
            on_video_data_received: Some(on_video_data_received),
            on_video_disable_warning: Some(on_video_disable_warning),
            on_video_disable_warning_lifted: Some(on_video_disable_warning_lifted),
            on_audio_level_updated: Some(on_audio_level_updated),
            on_error: Some(on_error),
            on_audio_stats: None, // TODO
            on_video_stats: None, // TODO
            user_data: subscriber_ptr,
            reserved: std::ptr::null_mut(),
        };
        let _ = subscriber
            .ptr
            .set(unsafe { ffi::otc_subscriber_new(*stream, &ffi_callbacks) });
        let _ = subscriber.ffi_callbacks.set(ffi_callbacks);
        subscriber
    }

    callback_call_with_copy!(on_connected, *const ffi::otc_stream, ffi::otc_stream_copy);
    callback_call!(on_disconnected);
    callback_call!(on_reconnected);
    callback_call_with_copy!(
        on_render_frame,
        *const ffi::otc_video_frame,
        ffi::otc_video_frame_copy
    );
    callback_call!(on_video_disabled, ffi::otc_video_reason);
    callback_call!(on_video_enabled, ffi::otc_video_reason);
    callback_call!(on_audio_disabled);
    callback_call!(on_audio_enabled);
    callback_call!(on_video_data_received);
    callback_call!(on_video_disable_warning);
    callback_call!(on_video_disable_warning_lifted);
    callback_call!(on_audio_level_updated, f32);

    fn on_error(&self, error_string: *const c_char, error_code: ffi::otc_subscriber_error_code) {
        if error_string.is_null() {
            return;
        }
        let error_string = unsafe { CStr::from_ptr(error_string) };
        self.callbacks.lock().unwrap().on_error(
            self.clone(),
            error_string.to_str().unwrap_or_default(),
            error_code.into(),
        );
    }

    pub fn delete(&self) {
        let ptr = self.ptr.get().unwrap();
        if ptr.is_null() {
            return;
        }
        unsafe { ffi::otc_subscriber_delete(*ptr as *mut ffi::otc_subscriber) };
    }

    pub fn get_stream(&self) -> Stream {
        self.stream.clone()
    }

    pub fn set_subscribe_to_video(&self, subscribe_to_video: bool) -> OtcResult {
        let ptr = self.ptr.get().unwrap();
        if ptr.is_null() {
            return Err(OtcError::NullError);
        }

        let subscribe_to_video: OtcBool = subscribe_to_video.into();
        unsafe {
            ffi::otc_subscriber_set_subscribe_to_video(
                *ptr as *mut ffi::otc_subscriber,
                subscribe_to_video.into(),
            )
        }
        .into_result()
    }

    pub fn set_subscribe_to_audio(&self, subscribe_to_audio: bool) -> OtcResult {
        let ptr = self.ptr.get().unwrap();
        if ptr.is_null() {
            return Err(OtcError::NullError);
        }

        let subscribe_to_video: OtcBool = subscribe_to_audio.into();
        unsafe {
            ffi::otc_subscriber_set_subscribe_to_video(
                *ptr as *mut ffi::otc_subscriber,
                subscribe_to_video.into(),
            )
        }
        .into_result()
    }

    pub fn get_subscribe_to_video(&self) -> Result<bool, OtcError> {
        let ptr = self.ptr.get().unwrap();
        if ptr.is_null() {
            return Err(OtcError::NullError);
        }
        Ok(*OtcBool(unsafe {
            ffi::otc_subscriber_get_subscribe_to_video(*ptr as *mut ffi::otc_subscriber)
        }))
    }

    pub fn get_subscribe_to_audio(&self) -> Result<bool, OtcError> {
        let ptr = self.ptr.get().unwrap();
        if ptr.is_null() {
            return Err(OtcError::NullError);
        }
        Ok(*OtcBool(unsafe {
            ffi::otc_subscriber_get_subscribe_to_audio(*ptr as *mut ffi::otc_subscriber)
        }))
    }

    pub fn set_preferred_resolution(&self, width: u32, height: u32) -> OtcResult {
        let ptr = self.ptr.get().unwrap();
        if ptr.is_null() {
            return Err(OtcError::NullError);
        }

        unsafe {
            ffi::otc_subscriber_set_preferred_resolution(
                *ptr as *mut ffi::otc_subscriber,
                width,
                height,
            )
        }
        .into_result()
    }

    pub fn get_preferred_resolution(&self) -> Result<(u32, u32), OtcError> {
        let ptr = self.ptr.get().unwrap();
        if ptr.is_null() {
            return Err(OtcError::NullError);
        }
        let mut width = 0;
        let mut height = 0;
        unsafe {
            ffi::otc_subscriber_get_preferred_resolution(
                *ptr as *mut ffi::otc_subscriber,
                &mut width as *mut u32,
                &mut height as *mut u32,
            )
        }
        .into_result()
        .map(|_| (width, height))
    }

    pub fn set_preferred_framerate(&self, framerate: f32) -> OtcResult {
        let ptr = self.ptr.get().unwrap();
        if ptr.is_null() {
            return Err(OtcError::NullError);
        }
        unsafe {
            ffi::otc_subscriber_set_preferred_framerate(*ptr as *mut ffi::otc_subscriber, framerate)
        }
        .into_result()
    }

    pub fn get_preferred_framerate(&self) -> Result<f32, OtcError> {
        let ptr = self.ptr.get().unwrap();
        if ptr.is_null() {
            return Err(OtcError::NullError);
        }
        let mut framerate = 0.0;
        unsafe {
            ffi::otc_subscriber_get_preferred_framerate(
                *ptr as *mut ffi::otc_subscriber,
                &mut framerate as *mut f32,
            )
        }
        .into_result()
        .map(|_| framerate)
    }
}
