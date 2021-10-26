use crate::enums::{IntoResult, OtcError, OtcResult};
use crate::stream::Stream;
use crate::video_capturer::VideoCapturer;
use crate::video_frame::VideoFrame;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref INSTANCES: Arc<Mutex<HashMap<usize, Publisher>>> = Default::default();
}

/// This enumeration represents all the possible error types
/// associated with a publisher.
pub enum PublisherError {
    /// Internal error.
    Internal,
    /// Tried to publish on a disconnected session.
    SessionDisconnected,
    /// Timed out attempting to publish.
    TimedOut,
    /// Unable to publish.
    UnableToPublish,
    /// WebRTC error.
    WebRtcError,
    /// Unknown publisher error.
    __Unknown,
}

impl From<ffi::otc_publisher_error_code> for PublisherError {
    fn from(type_: ffi::otc_publisher_error_code) -> PublisherError {
        match type_ {
            ffi::otc_publisher_error_code_OTC_PUBLISHER_INTERNAL_ERROR => PublisherError::Internal,
            ffi::otc_publisher_error_code_OTC_PUBLISHER_SESSION_DISCONNECTED => {
                PublisherError::SessionDisconnected
            }
            ffi::otc_publisher_error_code_OTC_PUBLISHER_UNABLE_TO_PUBLISH => {
                PublisherError::UnableToPublish
            }
            ffi::otc_publisher_error_code_OTC_PUBLISHER_TIMED_OUT => PublisherError::TimedOut,
            ffi::otc_publisher_error_code_OTC_PUBLISHER_WEBRTC_ERROR => PublisherError::WebRtcError,
            _ => PublisherError::__Unknown,
        }
    }
}

/// Type of video being published by the Publisher.
enum VideoType {
    /// Camera video stream.
    Camera,
    /// Screen capture video stream.
    Screen,
}

/// Audio and video publisher statistics.
struct MediaStats {
    /// The unique identifier for the client connection.
    connection_id: String,
    /// The unique identifier of the subscriber.
    subscriber_id: String,
    /// The total number of audio or video packets that did not reach
    /// the subscriber (or the Opentok Media Router).
    packets_lost: i64,
    /// The total number of audio or video packets sent to the subscriber
    /// (or to the OpenTok Media Router).
    bytes_sent: i64,
    /// The audio level value, from 0 to 1.0.
    audio_level: f32,
    /// The time when these stats were gathered.
    timestamp: f64,
    /// The time when the cumulative totals started accumulating.
    start_time: f64,
}

/// This enum represents the publisher audio and video stats.
/// PublisherCallbacks includes a `on_audio_stats` and
/// `on_video_stats` callback that are periodically called to
/// report audio and video stats.
enum PublisherStats {
    Audio(MediaStats),
    Video(MediaStats),
}

ffi_callback!(
    on_stream_created,
    *mut ffi::otc_publisher,
    Publisher,
    *const ffi::otc_stream
);

ffi_callback!(
    on_stream_destroyed,
    *mut ffi::otc_publisher,
    Publisher,
    *const ffi::otc_stream
);

ffi_callback!(
    on_render_frame,
    *mut ffi::otc_publisher,
    Publisher,
    *const ffi::otc_video_frame
);

ffi_callback!(
    on_audio_level_updated,
    *mut ffi::otc_publisher,
    Publisher,
    f32
);

/*
TODO
ffi_callback!(
    on_audio_stats,
    *mut ffi::otc_publisher,
    Publisher,
    *mut ffi::otc_publisher_audio_stats,
    ffi::size_t
);

ffi_callback!(
    on_video_stats,
    *mut ffi::otc_publisher,
    Publisher,
    *mut ffi::otc_publisher_video_stats,
    ffi::size_t
);*/

ffi_callback!(
    on_error,
    *mut ffi::otc_publisher,
    Publisher,
    *const c_char,
    ffi::otc_publisher_error_code
);

/// Callbacks triggered in response to events related to an OpenTok
/// publisher.
///
/// These callbacks are not executed on the application main thread but
/// on an internal thread. The application should return the callback as
/// quickly as possible to avoid blocking the internal thread.
///
/// Data passed into a callback (other than `publisher` and `user_data`)
/// is released after the callback finishes its execution.
#[allow(clippy::type_complexity)]
pub struct PublisherCallbacks {
    on_stream_created: Option<Box<dyn Fn(&Publisher, Stream) + Send + Sync + 'static>>,
    on_stream_destroyed: Option<Box<dyn Fn(&Publisher, Stream) + Send + Sync + 'static>>,
    on_render_frame: Option<Box<dyn Fn(&Publisher, VideoFrame) + Send + Sync + 'static>>,
    on_audio_level_updated: Option<Box<dyn Fn(&Publisher, f32) + Send + Sync + 'static>>,
    //TODO: on_audio_stats: Option<Box<dyn Fn(&Publisher, AudioStats)>>,
    //TODO: on_video_stats: Option<Box<dyn Fn(&Publisher, VideoStats)>>,
    on_error: Option<Box<dyn Fn(&Publisher, &str, PublisherError) + Send + Sync + 'static>>,
}

impl PublisherCallbacks {
    pub fn builder() -> PublisherCallbacksBuilder {
        PublisherCallbacksBuilder::default()
    }

    callback!(on_stream_created, &Publisher, Stream);
    callback!(on_stream_destroyed, &Publisher, Stream);
    callback!(on_render_frame, &Publisher, VideoFrame);
    callback!(on_audio_level_updated, &Publisher, f32);
    callback!(on_error, &Publisher, &str, PublisherError);
}

#[derive(Default)]
#[allow(clippy::type_complexity)]
pub struct PublisherCallbacksBuilder {
    on_stream_created: Option<Box<dyn Fn(&Publisher, Stream) + Send + Sync + 'static>>,
    on_stream_destroyed: Option<Box<dyn Fn(&Publisher, Stream) + Send + Sync + 'static>>,
    on_render_frame: Option<Box<dyn Fn(&Publisher, VideoFrame) + Send + Sync + 'static>>,
    on_audio_level_updated: Option<Box<dyn Fn(&Publisher, f32) + Send + Sync + 'static>>,
    //TODO: on_audio_stats: Option<Box<dyn Fn(&Publisher, AudioStats)>>,
    //TODO: on_video_stats: Option<Box<dyn Fn(&Publisher, VideoStats)>>,
    on_error: Option<Box<dyn Fn(&Publisher, &str, PublisherError) + Send + Sync + 'static>>,
}

impl PublisherCallbacksBuilder {
    callback_setter!(on_stream_created, &Publisher, Stream);
    callback_setter!(on_stream_destroyed, &Publisher, Stream);
    callback_setter!(on_render_frame, &Publisher, VideoFrame);
    callback_setter!(on_audio_level_updated, &Publisher, f32);
    callback_setter!(on_error, &Publisher, &str, PublisherError);

    pub fn build(self) -> PublisherCallbacks {
        PublisherCallbacks {
            on_stream_created: self.on_stream_created,
            on_stream_destroyed: self.on_stream_destroyed,
            on_render_frame: self.on_render_frame,
            on_audio_level_updated: self.on_audio_level_updated,
            on_error: self.on_error,
        }
    }
}

#[derive(Clone)]
pub struct Publisher {
    ptr: Arc<AtomicPtr<*const ffi::otc_publisher>>,
    capturer: Option<VideoCapturer>,
    callbacks: Arc<Mutex<PublisherCallbacks>>,
    publishing: Arc<AtomicBool>,
}

unsafe impl Sync for Publisher {}
unsafe impl Send for Publisher {}

impl Publisher {
    pub fn new(name: &str, capturer: Option<VideoCapturer>, callbacks: PublisherCallbacks) -> Self {
        let name = CString::new(name).unwrap_or_default();
        let capturer_callbacks = capturer.clone().map_or(std::ptr::null(), |mut capturer| {
            &*capturer.callbacks().lock().unwrap() as *const ffi::otc_video_capturer_callbacks
        });

        let ffi_callbacks = ffi::otc_publisher_callbacks {
            on_stream_created: Some(on_stream_created),
            on_stream_destroyed: Some(on_stream_destroyed),
            on_render_frame: Some(on_render_frame),
            on_audio_level_updated: Some(on_audio_level_updated),
            on_audio_stats: None,
            on_video_stats: None,
            on_error: Some(on_error),
            user_data: std::ptr::null_mut(),
            reserved: std::ptr::null_mut(),
        };
        let ptr =
            unsafe { ffi::otc_publisher_new(name.as_ptr(), capturer_callbacks, &ffi_callbacks) };
        let publisher = Self {
            ptr: Arc::new(AtomicPtr::new(ptr as *mut _)),
            capturer,
            callbacks: Arc::new(Mutex::new(callbacks)),
            publishing: Default::default(),
        };
        INSTANCES
            .lock()
            .unwrap()
            .insert(ptr as usize, publisher.clone());
        publisher
    }

    pub fn inner(&self) -> *const ffi::otc_publisher {
        self.ptr.load(Ordering::Relaxed) as *const _
    }

    callback_call!(on_render_frame, *const ffi::otc_video_frame);
    callback_call!(on_audio_level_updated, f32);

    fn on_stream_created(&self, stream: *const ffi::otc_stream) {
        self.publishing.store(true, Ordering::Relaxed);
        if let Ok(callbacks) = self.callbacks.try_lock() {
            callbacks.on_stream_created(self, stream.into());
        }
    }

    fn on_stream_destroyed(&self, stream: *const ffi::otc_stream) {
        self.publishing.store(false, Ordering::Relaxed);
        if let Ok(callbacks) = self.callbacks.try_lock() {
            callbacks.on_stream_destroyed(self, stream.into());
        }
    }

    fn on_error(&self, error_string: *const c_char, error_code: ffi::otc_publisher_error_code) {
        if error_string.is_null() {
            return;
        }
        let error_string = unsafe { CStr::from_ptr(error_string) };
        if let Ok(callbacks) = self.callbacks.try_lock() {
            callbacks.on_error(
                self,
                error_string.to_str().unwrap_or_default(),
                error_code.into(),
            );
        }
    }

    pub fn toggle_audio(&self, audio_enabled: bool) -> OtcResult {
        if self.ptr.load(Ordering::Relaxed).is_null() {
            return Err(OtcError::NullError);
        }
        unsafe {
            ffi::otc_publisher_set_publish_audio(
                self.ptr.load(Ordering::Relaxed) as *mut _,
                audio_enabled.into(),
            )
        }
        .into_result()
    }

    pub fn toggle_video(&self, video_enabled: bool) -> OtcResult {
        if self.ptr.load(Ordering::Relaxed).is_null() {
            return Err(OtcError::NullError);
        }
        unsafe {
            ffi::otc_publisher_set_publish_video(
                self.ptr.load(Ordering::Relaxed) as *mut _,
                video_enabled.into(),
            )
        }
        .into_result()
    }

    pub fn stream(&self) -> Option<Stream> {
        if self.ptr.load(Ordering::Relaxed).is_null() {
            return None;
        }
        let stream_ptr =
            unsafe { ffi::otc_publisher_get_stream(self.ptr.load(Ordering::Relaxed) as *mut _) };
        if stream_ptr.is_null() {
            return None;
        }
        Some((stream_ptr as *const ffi::otc_stream).into())
    }
}

impl Drop for Publisher {
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
            let session = ffi::otc_publisher_get_session(ptr as *const _);
            if !session.is_null() {
                ffi::otc_session_unpublish(session, ptr as *mut _);
            }
            ffi::otc_publisher_delete(ptr as *mut _);
        }

        if let Ok(ref mut instances) = INSTANCES.try_lock() {
            instances.remove(&(ptr as usize));
        }
    }
}
