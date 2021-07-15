use crate::video_capturer::VideoCapturer;

use once_cell::unsync::OnceCell;
use std::ffi::CString;
use std::ops::Deref;
use std::os::raw::{c_char, c_void};
use std::rc::Rc;

/// This enumeration represents all the possible error types
/// associated with a publisher.
enum PublisherError {
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

ffi_callback_proxy!(
    on_stream_created,
    *mut ffi::otc_publisher,
    Publisher,
    *const ffi::otc_stream
);

ffi_callback_proxy!(
    on_stream_destroyed,
    *mut ffi::otc_publisher,
    Publisher,
    *const ffi::otc_stream
);

ffi_callback_proxy!(
    on_render_frame,
    *mut ffi::otc_publisher,
    Publisher,
    *const ffi::otc_video_frame
);

ffi_callback_proxy!(
    on_audio_level_updated,
    *mut ffi::otc_publisher,
    Publisher,
    f32
);

ffi_callback_proxy!(
    on_audio_stats,
    *mut ffi::otc_publisher,
    Publisher,
    *mut ffi::otc_publisher_audio_stats,
    ffi::size_t
);

ffi_callback_proxy!(
    on_video_stats,
    *mut ffi::otc_publisher,
    Publisher,
    *mut ffi::otc_publisher_video_stats,
    ffi::size_t
);

ffi_callback_proxy!(
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
pub struct PublisherCallbacks {}

pub struct Publisher {
    ptr: OnceCell<*const ffi::otc_publisher>,
    capturer: Option<Rc<VideoCapturer>>,
    callbacks: PublisherCallbacks,
    ffi_callbacks: OnceCell<ffi::otc_publisher_callbacks>,
}

impl Publisher {
    pub fn new(
        name: &str,
        capturer: Option<Rc<VideoCapturer>>,
        callbacks: PublisherCallbacks,
    ) -> Self {
        let name = CString::new(name).unwrap_or_default();
        let capturer_callbacks = capturer.clone().map_or(std::ptr::null(), |capturer| {
            capturer.callbacks() as *const ffi::otc_video_capturer_callbacks
        });
        let mut publisher = Self {
            ptr: Default::default(),
            capturer,
            callbacks,
            ffi_callbacks: Default::default(),
        };
        let publisher_ptr: *mut c_void = &mut publisher as *mut _ as *mut c_void;
        let ffi_callbacks = ffi::otc_publisher_callbacks {
            on_stream_created: Some(on_stream_created),
            on_stream_destroyed: Some(on_stream_destroyed),
            on_render_frame: Some(on_render_frame),
            on_audio_level_updated: Some(on_audio_level_updated),
            on_audio_stats: Some(on_audio_stats),
            on_video_stats: Some(on_video_stats),
            on_error: Some(on_error),
            user_data: publisher_ptr,
            reserved: std::ptr::null_mut(),
        };
        let _ = publisher.ptr.set(unsafe {
            ffi::otc_publisher_new(name.as_ptr(), capturer_callbacks, &ffi_callbacks)
        });
        let _ = publisher.ffi_callbacks.set(ffi_callbacks);
        publisher
    }

    fn on_stream_created(&self, _stream: *const ffi::otc_stream) {}

    fn on_stream_destroyed(&self, _stream: *const ffi::otc_stream) {}

    fn on_render_frame(&self, _frame: *const ffi::otc_video_frame) {}

    fn on_audio_level_updated(&self, _audio_level: f32) {}

    fn on_audio_stats(
        &self,
        _audio_stats: *mut ffi::otc_publisher_audio_stats,
        _number_of_stats: ffi::size_t,
    ) {
    }

    fn on_video_stats(
        &self,
        _video_stats: *mut ffi::otc_publisher_video_stats,
        _number_of_stats: ffi::size_t,
    ) {
    }

    fn on_error(&self, _error_string: *const c_char, _error_code: ffi::otc_publisher_error_code) {}
}

impl Deref for Publisher {
    type Target = *const ffi::otc_publisher;

    fn deref(&self) -> &Self::Target {
        self.ptr.get().unwrap()
    }
}
