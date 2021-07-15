use crate::enums::{IntoResult, OtcBool, OtcError, OtcResult};
use crate::video_frame::{FrameFormat, VideoFrame};

use once_cell::unsync::OnceCell;
use std::os::raw::c_void;

/// Settings for a VideoCapturer.
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
            width: 800,
            height: 600,
            fps: 30,
            expected_delay: 0,
            mirror_on_local_render: false,
        }
    }
}

unsafe extern "C" fn init(
    capturer: *const ffi::otc_video_capturer,
    data: *mut c_void,
) -> ffi::otc_bool {
    let target = data as *mut VideoCapturer;
    let result: OtcBool = (*target).init(capturer).into();
    result.0
}

ffi_callback_proxy_with_return!(
    destroy,
    *const ffi::otc_video_capturer,
    VideoCapturer,
    ffi::otc_bool
);
ffi_callback_proxy_with_return!(
    start,
    *const ffi::otc_video_capturer,
    VideoCapturer,
    ffi::otc_bool
);
ffi_callback_proxy_with_return!(
    stop,
    *const ffi::otc_video_capturer,
    VideoCapturer,
    ffi::otc_bool
);
ffi_callback_proxy_with_return!(
    get_capture_settings,
    *const ffi::otc_video_capturer,
    VideoCapturer,
    *mut ffi::otc_video_capturer_settings,
    ffi::otc_bool
);

pub struct VideoCapturerCallbacks {
    init: Option<Box<dyn Fn()>>,
    destroy: Option<Box<dyn Fn()>>,
    start: Option<Box<dyn Fn()>>,
    stop: Option<Box<dyn Fn()>>,
    get_capture_settings: Option<Box<dyn Fn(VideoCapturerSettings)>>,
}

pub struct VideoCapturer {
    ptr: Option<*const ffi::otc_video_capturer>,
    settings: VideoCapturerSettings,
    callbacks: VideoCapturerCallbacks,
    ffi_callbacks: OnceCell<ffi::otc_video_capturer_callbacks>,
}

impl VideoCapturer {
    pub fn new(settings: VideoCapturerSettings, callbacks: VideoCapturerCallbacks) -> Self {
        let mut capturer = Self {
            ptr: None,
            settings,
            callbacks,
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

    fn init(&mut self, ptr: *const ffi::otc_video_capturer) -> OtcResult {
        self.ptr = Some(ptr);
        Ok(())
    }

    fn destroy(&mut self) -> OtcResult {
        Ok(())
    }

    fn start(&mut self) -> OtcResult {
        Ok(())
    }

    fn stop(&mut self) -> OtcResult {
        Ok(())
    }

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
