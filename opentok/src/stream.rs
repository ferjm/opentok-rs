use crate::connection::Connection;

use std::ffi::CStr;
use std::sync::atomic::{AtomicPtr, Ordering};

/// Different type of video streams supported.
pub enum StreamVideoType {
    /// This is a video stream coming from a camera.
    Camera,
    /// This is a video stream coming from a screen capture.
    Screen,
    /// This is a custom video stream.
    Custom,
    /// Unknown video stream type.
    __Unknown,
}

impl From<ffi::otc_stream_video_type> for StreamVideoType {
    fn from(type_: ffi::otc_stream_video_type) -> StreamVideoType {
        match type_ {
            ffi::otc_stream_video_type_OTC_STREAM_VIDEO_TYPE_CAMERA => StreamVideoType::Camera,
            ffi::otc_stream_video_type_OTC_STREAM_VIDEO_TYPE_SCREEN => StreamVideoType::Screen,
            ffi::otc_stream_video_type_OTC_STREAM_VIDEO_TYPE_CUSTOM => StreamVideoType::Custom,
            _ => StreamVideoType::__Unknown,
        }
    }
}

pub struct Stream {
    ptr: AtomicPtr<*const ffi::otc_stream>,
}

impl Stream {
    pub fn inner(&self) -> *const ffi::otc_stream {
        self.ptr.load(Ordering::Relaxed) as *const _
    }

    string_getter!(
        /// Gets the uniquer identifier for this stream.
        => (id, otc_stream_get_id)
    );
    string_getter!(
        /// Gets the name of the stream. The publisher of the stream
        /// can set this name to identify the stream.
        => (name, otc_stream_get_name)
    );

    /// Checks whether this stream is currently publishing video or not.
    pub fn has_video(&self) -> bool {
        unsafe { ffi::otc_stream_has_video(self.ptr.load(Ordering::Relaxed) as *const _) != 0 }
    }

    /// Checks whether this stream contains a video track or not.
    pub fn has_video_track(&self) -> bool {
        unsafe {
            ffi::otc_stream_has_video_track(self.ptr.load(Ordering::Relaxed) as *const _) != 0
        }
    }

    /// Checks whether this stream is currently publishing audio or not.
    pub fn has_audio(&self) -> bool {
        unsafe { ffi::otc_stream_has_audio(self.ptr.load(Ordering::Relaxed) as *const _) != 0 }
    }

    /// Checks whether this stream contains an audio track or not.
    pub fn has_audio_track(&self) -> bool {
        unsafe {
            ffi::otc_stream_has_audio_track(self.ptr.load(Ordering::Relaxed) as *const _) != 0
        }
    }

    /// Return the width of the stream in pixels.
    pub fn get_video_width(&self) -> i32 {
        unsafe { ffi::otc_stream_get_video_width(self.ptr.load(Ordering::Relaxed) as *const _) }
    }

    /// Return the height of the stream in pixels.
    pub fn get_video_height(&self) -> i32 {
        unsafe { ffi::otc_stream_get_video_height(self.ptr.load(Ordering::Relaxed) as *const _) }
    }

    /// Get the creation time of the stream.
    pub fn get_creation_time(&self) -> i64 {
        unsafe { ffi::otc_stream_get_creation_time(self.ptr.load(Ordering::Relaxed) as *const _) }
    }

    pub fn get_video_type(&self) -> StreamVideoType {
        unsafe { ffi::otc_stream_get_video_type(self.ptr.load(Ordering::Relaxed) as *const _) }
            .into()
    }

    /// Get the Connection associated with the client publishing the stream.
    pub fn get_connection(&self) -> Connection {
        unsafe { ffi::otc_stream_get_connection(self.ptr.load(Ordering::Relaxed) as *const _) }
            .into()
    }
}

impl Clone for Stream {
    fn clone(&self) -> Self {
        (self.ptr.load(Ordering::Relaxed) as *const ffi::otc_stream).into()
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        let ptr = self.ptr.load(Ordering::Relaxed);

        if ptr.is_null() {
            return;
        }

        self.ptr.store(std::ptr::null_mut(), Ordering::Relaxed);

        unsafe {
            ffi::otc_stream_delete(ptr as *mut _);
        }
    }
}

impl From<*const ffi::otc_stream> for Stream {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn from(ptr: *const ffi::otc_stream) -> Stream {
        let ptr = unsafe { ffi::otc_stream_copy(ptr) };
        Stream {
            ptr: AtomicPtr::new(ptr as *mut _),
        }
    }
}
