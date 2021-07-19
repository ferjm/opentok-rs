use crate::enums::{IntoResult, OtcBool, OtcError, OtcResult};
use crate::video_frame::{FrameFormat, VideoFrame};

use once_cell::unsync::OnceCell;
use std::os::raw::c_void;
use std::sync::{Arc, Mutex};

/// Settings for a VideoCapturer.
#[derive(Clone)]
pub struct VideoCapturerSettings {
    /// The pixel format.
    format: FrameFormat,
    /// The width of the video in pixels.
    width: i32,
    /// The height of the video in pixels.
    height: i32,
    /// The estimated number of frames per second of video.
    fps: i32,
    /// The estimated capture delay, in milliseconds.
    expected_delay: i32,
    /// Whether the frame should appear mirrored on the x-axis
    /// in the local renderer.
    mirror_on_local_render: bool,
}

impl Default for VideoCapturerSettings {
    fn default() -> Self {
        Self {
            format: FrameFormat::Rgba32,
            width: 1280,
            height: 720,
            fps: 30,
            expected_delay: 0,
            mirror_on_local_render: false,
        }
    }
}

ffi_callback_with_return!(
    init,
    *const ffi::otc_video_capturer,
    VideoCapturer,
    ffi::otc_bool
);

ffi_callback_with_return!(
    destroy,
    *const ffi::otc_video_capturer,
    VideoCapturer,
    ffi::otc_bool
);
ffi_callback_with_return!(
    start,
    *const ffi::otc_video_capturer,
    VideoCapturer,
    ffi::otc_bool
);
ffi_callback_with_return!(
    stop,
    *const ffi::otc_video_capturer,
    VideoCapturer,
    ffi::otc_bool
);
ffi_callback_with_return!(
    get_capture_settings,
    *const ffi::otc_video_capturer,
    VideoCapturer,
    *mut ffi::otc_video_capturer_settings,
    ffi::otc_bool
);

#[derive(Default)]
pub struct VideoCapturerCallbacks {
    init: Option<Box<dyn Fn(VideoCapturer) -> OtcResult>>,
    destroy: Option<Box<dyn Fn(VideoCapturer) -> OtcResult>>,
    start: Option<Box<dyn Fn(VideoCapturer) -> OtcResult>>,
    stop: Option<Box<dyn Fn(VideoCapturer) -> OtcResult>>,
}

impl VideoCapturerCallbacks {
    pub fn builder() -> VideoCapturerCallbacksBuilder {
        VideoCapturerCallbacksBuilder::default()
    }

    callback_with_return!(init, VideoCapturer, OtcResult);
    callback_with_return!(destroy, VideoCapturer, OtcResult);
    callback_with_return!(start, VideoCapturer, OtcResult);
    callback_with_return!(stop, VideoCapturer, OtcResult);
}

#[derive(Default)]
pub struct VideoCapturerCallbacksBuilder {
    init: Option<Box<dyn Fn(VideoCapturer) -> OtcResult>>,
    destroy: Option<Box<dyn Fn(VideoCapturer) -> OtcResult>>,
    start: Option<Box<dyn Fn(VideoCapturer) -> OtcResult>>,
    stop: Option<Box<dyn Fn(VideoCapturer) -> OtcResult>>,
}

impl VideoCapturerCallbacksBuilder {
    callback_setter_with_return!(init, VideoCapturer, OtcResult);
    callback_setter_with_return!(destroy, VideoCapturer, OtcResult);
    callback_setter_with_return!(start, VideoCapturer, OtcResult);
    callback_setter_with_return!(stop, VideoCapturer, OtcResult);

    pub fn build(self) -> VideoCapturerCallbacks {
        VideoCapturerCallbacks {
            init: self.init,
            destroy: self.destroy,
            start: self.start,
            stop: self.stop,
        }
    }
}

#[derive(Clone)]
pub struct VideoCapturer {
    ptr: Option<*const ffi::otc_video_capturer>,
    settings: VideoCapturerSettings,
    callbacks: Arc<Mutex<VideoCapturerCallbacks>>,
    ffi_callbacks: OnceCell<ffi::otc_video_capturer_callbacks>,
}

impl VideoCapturer {
    pub fn new(settings: VideoCapturerSettings, callbacks: VideoCapturerCallbacks) -> Self {
        let mut capturer = Self {
            ptr: None,
            settings,
            callbacks: Arc::new(Mutex::new(callbacks)),
            ffi_callbacks: Default::default(),
        };
        let capturer_ptr: *mut c_void = &mut capturer as *mut _ as *mut c_void;
        let ffi_callbacks = ffi::otc_video_capturer_callbacks {
            init: Some(init),
            destroy: Some(destroy),
            start: Some(start),
            stop: Some(stop),
            get_capture_settings: Some(get_capture_settings),
            user_data: capturer_ptr,
            reserved: std::ptr::null_mut(),
        };
        let _ = capturer.ffi_callbacks.set(ffi_callbacks);
        capturer
    }

    pub fn callbacks(&self) -> &ffi::otc_video_capturer_callbacks {
        self.ffi_callbacks.get().unwrap()
    }

    fn provide_frame(&self, rotation: i32, frame: &VideoFrame) -> OtcResult {
        match self.ptr {
            Some(ptr) => unsafe { ffi::otc_video_capturer_provide_frame(ptr, rotation, **frame) }
                .into_result(),
            None => Err(OtcError::NullError),
        }
    }

    callback_call_with_return!(init, OtcResult);
    callback_call_with_return!(destroy, OtcResult);
    callback_call_with_return!(start, OtcResult);
    callback_call_with_return!(stop, OtcResult);

    fn get_capture_settings(
        &mut self,
        settings: *mut ffi::otc_video_capturer_settings,
    ) -> OtcResult {
        if settings.is_null() {
            return Err(OtcError::NullError);
        }
        unsafe {
            (*settings).format = self.settings.format as i32;
            (*settings).width = self.settings.width;
            (*settings).height = self.settings.height;
            (*settings).fps = self.settings.fps;
            (*settings).expected_delay = self.settings.expected_delay;
            (*settings).mirror_on_local_render = self.settings.mirror_on_local_render as i32;
        }
        Ok(())
    }
}
