use crate::enums::{IntoResult, OtcBool, OtcError, OtcResult};
use crate::stream::Stream;
use crate::video_frame::VideoFrame;

use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref INSTANCES: Arc<Mutex<HashMap<usize, Subscriber>>> = Default::default();
}

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
    on_connected: Option<Box<dyn Fn(&Subscriber, Stream) + Send + Sync + 'static>>,
    on_disconnected: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_reconnected: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_render_frame: Option<Box<dyn Fn(&Subscriber, VideoFrame) + Send + Sync + 'static>>,
    on_video_disabled: Option<Box<dyn Fn(&Subscriber, VideoReason) + Send + Sync + 'static>>,
    on_video_enabled: Option<Box<dyn Fn(&Subscriber, VideoReason) + Send + Sync + 'static>>,
    on_audio_disabled: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_audio_enabled: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_video_data_received: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_video_disable_warning: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_video_disable_warning_lifted: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_audio_level_updated: Option<Box<dyn Fn(&Subscriber, f32) + Send + Sync + 'static>>,
    on_error: Option<Box<dyn Fn(&Subscriber, &str, SubscriberError) + Send + Sync + 'static>>,
}

impl SubscriberCallbacks {
    pub fn builder() -> SubscriberCallbacksBuilder {
        SubscriberCallbacksBuilder::default()
    }

    callback!(on_connected, &Subscriber, Stream);
    callback!(on_disconnected, &Subscriber);
    callback!(on_reconnected, &Subscriber);
    callback!(on_render_frame, &Subscriber, VideoFrame);
    callback!(on_video_disabled, &Subscriber, VideoReason);
    callback!(on_video_enabled, &Subscriber, VideoReason);
    callback!(on_audio_disabled, &Subscriber);
    callback!(on_audio_enabled, &Subscriber);
    callback!(on_video_data_received, &Subscriber);
    callback!(on_video_disable_warning, &Subscriber);
    callback!(on_video_disable_warning_lifted, &Subscriber);
    callback!(on_audio_level_updated, &Subscriber, f32);
    callback!(on_error, &Subscriber, &str, SubscriberError);
}

#[derive(Default)]
#[allow(clippy::type_complexity)]
pub struct SubscriberCallbacksBuilder {
    on_connected: Option<Box<dyn Fn(&Subscriber, Stream) + Send + Sync + 'static>>,
    on_disconnected: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_reconnected: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_render_frame: Option<Box<dyn Fn(&Subscriber, VideoFrame) + Send + Sync + 'static>>,
    on_video_disabled: Option<Box<dyn Fn(&Subscriber, VideoReason) + Send + Sync + 'static>>,
    on_video_enabled: Option<Box<dyn Fn(&Subscriber, VideoReason) + Send + Sync + 'static>>,
    on_audio_disabled: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_audio_enabled: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_video_data_received: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_video_disable_warning: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_video_disable_warning_lifted: Option<Box<dyn Fn(&Subscriber) + Send + Sync + 'static>>,
    on_audio_level_updated: Option<Box<dyn Fn(&Subscriber, f32) + Send + Sync + 'static>>,
    on_error: Option<Box<dyn Fn(&Subscriber, &str, SubscriberError) + Send + Sync + 'static>>,
}

impl SubscriberCallbacksBuilder {
    callback_setter!(on_connected, &Subscriber, Stream);
    callback_setter!(on_disconnected, &Subscriber);
    callback_setter!(on_reconnected, &Subscriber);
    callback_setter!(on_render_frame, &Subscriber, VideoFrame);
    callback_setter!(on_video_disabled, &Subscriber, VideoReason);
    callback_setter!(on_video_enabled, &Subscriber, VideoReason);
    callback_setter!(on_audio_disabled, &Subscriber);
    callback_setter!(on_audio_enabled, &Subscriber);
    callback_setter!(on_video_data_received, &Subscriber);
    callback_setter!(on_video_disable_warning, &Subscriber);
    callback_setter!(on_video_disable_warning_lifted, &Subscriber);
    callback_setter!(on_audio_level_updated, &Subscriber, f32);
    callback_setter!(on_error, &Subscriber, &str, SubscriberError);

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
    ptr: Arc<Mutex<Option<*const ffi::otc_subscriber>>>,
    callbacks: Arc<Mutex<SubscriberCallbacks>>,
    stream: OnceCell<Stream>,
}

unsafe impl Send for Subscriber {}
unsafe impl Sync for Subscriber {}

impl Subscriber {
    pub fn new(callbacks: SubscriberCallbacks) -> Self {
        Self {
            ptr: Default::default(),
            callbacks: Arc::new(Mutex::new(callbacks)),
            stream: Default::default(),
        }
    }

    pub fn inner(&self) -> *const ffi::otc_subscriber {
        match *self.ptr.lock().unwrap() {
            Some(ptr) => ptr,
            None => std::ptr::null_mut(),
        }
    }

    callback_call!(on_connected, *const ffi::otc_stream);
    callback_call!(on_disconnected);
    callback_call!(on_reconnected);
    callback_call!(on_render_frame, *const ffi::otc_video_frame);
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
            self,
            error_string.to_str().unwrap_or_default(),
            error_code.into(),
        );
    }

    pub fn set_stream(&self, stream: Stream) -> OtcResult {
        if self.stream.get().is_some() || self.stream.set(stream.clone()).is_err() {
            return Err(OtcError::AlreadyInitialized("stream"));
        }

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
            user_data: std::ptr::null_mut(),
            reserved: std::ptr::null_mut(),
        };
        assert!(self.ptr.lock().unwrap().is_none());
        let ptr = unsafe { ffi::otc_subscriber_new(stream.inner(), &ffi_callbacks) };
        INSTANCES.lock().unwrap().insert(ptr as usize, self.clone());
        *self.ptr.lock().unwrap() = Some(ptr);
        Ok(())
    }

    pub fn get_stream(&self) -> Option<Stream> {
        self.stream.get().cloned()
    }

    pub fn set_subscribe_to_video(&self, subscribe_to_video: bool) -> OtcResult {
        if self.ptr.lock().unwrap().is_none() {
            return Err(OtcError::NullError);
        }

        let subscribe_to_video: OtcBool = subscribe_to_video.into();
        unsafe {
            ffi::otc_subscriber_set_subscribe_to_video(
                *self.ptr.lock().unwrap().as_ref().unwrap() as *mut ffi::otc_subscriber,
                subscribe_to_video.into(),
            )
        }
        .into_result()
    }

    pub fn set_subscribe_to_audio(&self, subscribe_to_audio: bool) -> OtcResult {
        if self.ptr.lock().unwrap().is_none() {
            return Err(OtcError::NullError);
        }

        let subscribe_to_video: OtcBool = subscribe_to_audio.into();
        unsafe {
            ffi::otc_subscriber_set_subscribe_to_video(
                *self.ptr.lock().unwrap().as_ref().unwrap() as *mut ffi::otc_subscriber,
                subscribe_to_video.into(),
            )
        }
        .into_result()
    }

    pub fn get_subscribe_to_video(&self) -> Result<bool, OtcError> {
        if self.ptr.lock().unwrap().is_none() {
            return Err(OtcError::NullError);
        }
        Ok(*OtcBool(unsafe {
            ffi::otc_subscriber_get_subscribe_to_video(
                *self.ptr.lock().unwrap().as_ref().unwrap() as *mut ffi::otc_subscriber
            )
        }))
    }

    pub fn get_subscribe_to_audio(&self) -> Result<bool, OtcError> {
        if self.ptr.lock().unwrap().is_none() {
            return Err(OtcError::NullError);
        }
        Ok(*OtcBool(unsafe {
            ffi::otc_subscriber_get_subscribe_to_audio(
                *self.ptr.lock().unwrap().as_ref().unwrap() as *mut ffi::otc_subscriber
            )
        }))
    }

    pub fn set_preferred_resolution(&self, width: u32, height: u32) -> OtcResult {
        if self.ptr.lock().unwrap().is_none() {
            return Err(OtcError::NullError);
        }

        unsafe {
            ffi::otc_subscriber_set_preferred_resolution(
                *self.ptr.lock().unwrap().as_ref().unwrap() as *mut ffi::otc_subscriber,
                width,
                height,
            )
        }
        .into_result()
    }

    pub fn get_preferred_resolution(&self) -> Result<(u32, u32), OtcError> {
        if self.ptr.lock().unwrap().is_none() {
            return Err(OtcError::NullError);
        }
        let mut width = 0;
        let mut height = 0;
        unsafe {
            ffi::otc_subscriber_get_preferred_resolution(
                *self.ptr.lock().unwrap().as_ref().unwrap() as *mut ffi::otc_subscriber,
                &mut width as *mut u32,
                &mut height as *mut u32,
            )
        }
        .into_result()
        .map(|_| (width, height))
    }

    pub fn set_preferred_framerate(&self, framerate: f32) -> OtcResult {
        if self.ptr.lock().unwrap().is_none() {
            return Err(OtcError::NullError);
        }
        unsafe {
            ffi::otc_subscriber_set_preferred_framerate(
                *self.ptr.lock().unwrap().as_ref().unwrap() as *mut ffi::otc_subscriber,
                framerate,
            )
        }
        .into_result()
    }

    pub fn get_preferred_framerate(&self) -> Result<f32, OtcError> {
        if self.ptr.lock().unwrap().is_none() {
            return Err(OtcError::NullError);
        }
        let mut framerate = 0.0;
        unsafe {
            ffi::otc_subscriber_get_preferred_framerate(
                *self.ptr.lock().unwrap().as_ref().unwrap() as *mut ffi::otc_subscriber,
                &mut framerate as *mut f32,
            )
        }
        .into_result()
        .map(|_| framerate)
    }
}

impl Drop for Subscriber {
    fn drop(&mut self) {
        match self.ptr.try_lock() {
            Ok(ptr) => {
                if ptr.is_none() {
                    return;
                }
            }
            Err(_) => return,
        }

        if Arc::strong_count(&self.ptr) != 2 {
            return;
        }

        let ptr = self.ptr.lock().unwrap().take().unwrap();
        unsafe {
            let session = ffi::otc_subscriber_get_session(ptr as *const _);
            if !session.is_null() {
                ffi::otc_session_unsubscribe(session, ptr as *mut _);
            }
            ffi::otc_subscriber_delete(ptr as *mut _);
        }

        if let Ok(ref mut instances) = INSTANCES.try_lock() {
            instances.remove(&(ptr as usize));
        }
    }
}
