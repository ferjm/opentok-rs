use crate::enums::{IntoResult, OtcBool, OtcError, OtcResult};
use crate::video_frame::{FrameFormat, VideoFrame};

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::os::raw::c_void;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref INSTANCES: Arc<Mutex<HashMap<usize, VideoCapturer>>> = Default::default();
}

/// Settings for a VideoCapturer.
#[derive(Clone, Copy)]
pub struct VideoCapturerSettings {
    /// The pixel format.
    pub format: FrameFormat,
    /// The width of the video in pixels.
    pub width: i32,
    /// The height of the video in pixels.
    pub height: i32,
    /// The estimated number of frames per second of video.
    pub fps: i32,
    /// The estimated capture delay, in milliseconds.
    pub expected_delay: i32,
    /// Whether the frame should appear mirrored on the x-axis
    /// in the local renderer.
    pub mirror_on_local_render: bool,
}

impl Default for VideoCapturerSettings {
    fn default() -> Self {
        Self {
            format: FrameFormat::Yuv420P,
            width: 1280,
            height: 720,
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
    let instance_id = data as usize;
    let result: OtcBool = INSTANCES
        .lock()
        .unwrap()
        .get(&instance_id)
        .unwrap()
        .init(capturer)
        .into();
    result.0
}

ffi_callback_with_return_user_data!(destroy, *const ffi::otc_video_capturer, ffi::otc_bool);
ffi_callback_with_return_user_data!(start, *const ffi::otc_video_capturer, ffi::otc_bool);
ffi_callback_with_return_user_data!(stop, *const ffi::otc_video_capturer, ffi::otc_bool);
ffi_callback_with_return_user_data!(
    get_capture_settings,
    *const ffi::otc_video_capturer,
    *mut ffi::otc_video_capturer_settings,
    ffi::otc_bool
);

#[derive(Default)]
pub struct VideoCapturerCallbacks {
    init: Option<Box<dyn Fn(&VideoCapturer) -> OtcResult + Send + Sync + 'static>>,
    destroy: Option<Box<dyn Fn(&VideoCapturer) -> OtcResult + Send + Sync + 'static>>,
    start: Option<Box<dyn Fn(&VideoCapturer) -> OtcResult + Send + Sync + 'static>>,
    stop: Option<Box<dyn Fn(&VideoCapturer) -> OtcResult + Send + Sync + 'static>>,
}

impl VideoCapturerCallbacks {
    pub fn builder() -> VideoCapturerCallbacksBuilder {
        VideoCapturerCallbacksBuilder::default()
    }

    callback_with_return!(init, &VideoCapturer, OtcResult);
    callback_with_return!(destroy, &VideoCapturer, OtcResult);
    callback_with_return!(start, &VideoCapturer, OtcResult);
    callback_with_return!(stop, &VideoCapturer, OtcResult);
}

#[derive(Default)]
pub struct VideoCapturerCallbacksBuilder {
    init: Option<Box<dyn Fn(&VideoCapturer) -> OtcResult + Send + Sync + 'static>>,
    destroy: Option<Box<dyn Fn(&VideoCapturer) -> OtcResult + Send + Sync + 'static>>,
    start: Option<Box<dyn Fn(&VideoCapturer) -> OtcResult + Send + Sync + 'static>>,
    stop: Option<Box<dyn Fn(&VideoCapturer) -> OtcResult + Send + Sync + 'static>>,
}

impl VideoCapturerCallbacksBuilder {
    callback_setter_with_return!(init, &VideoCapturer, OtcResult);
    callback_setter_with_return!(destroy, &VideoCapturer, OtcResult);
    callback_setter_with_return!(start, &VideoCapturer, OtcResult);
    callback_setter_with_return!(stop, &VideoCapturer, OtcResult);

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
    instance_id: usize,
    ptr: Arc<Mutex<Option<*const ffi::otc_video_capturer>>>,
    settings: VideoCapturerSettings,
    callbacks: Arc<Mutex<VideoCapturerCallbacks>>,
    ffi_callbacks: Arc<Mutex<ffi::otc_video_capturer_callbacks>>,
}

unsafe impl Send for VideoCapturer {}
unsafe impl Sync for VideoCapturer {}

impl VideoCapturer {
    pub fn new(settings: VideoCapturerSettings, callbacks: VideoCapturerCallbacks) -> Self {
        let instance_id = INSTANCES.lock().unwrap().len() + 1;
        let capturer = Self {
            instance_id,
            ptr: Default::default(),
            settings,
            callbacks: Arc::new(Mutex::new(callbacks)),
            ffi_callbacks: Arc::new(Mutex::new(ffi::otc_video_capturer_callbacks {
                init: None,
                destroy: None,
                start: None,
                stop: None,
                get_capture_settings: None,
                user_data: std::ptr::null_mut(),
                reserved: std::ptr::null_mut(),
            })),
        };
        INSTANCES
            .lock()
            .unwrap()
            .insert(instance_id, capturer.clone());
        capturer
    }

    pub fn callbacks(&mut self) -> Arc<Mutex<ffi::otc_video_capturer_callbacks>> {
        {
            *self.ffi_callbacks.lock().unwrap() = ffi::otc_video_capturer_callbacks {
                init: Some(init),
                destroy: Some(destroy),
                start: Some(start),
                stop: Some(stop),
                get_capture_settings: None, //Some(get_capture_settings),
                user_data: self.instance_id as *mut c_void,
                reserved: std::ptr::null_mut(),
            };
        }
        self.ffi_callbacks.clone()
    }

    pub fn provide_frame(&self, rotation: i32, frame: &VideoFrame) -> OtcResult {
        let ptr = self.ptr.lock().unwrap();
        if ptr.is_none() || ptr.as_ref().unwrap().is_null() {
            return Err(OtcError::NullError);
        }
        unsafe { ffi::otc_video_capturer_provide_frame(*ptr.as_ref().unwrap(), rotation, **frame) }
            .into_result()
    }

    fn init(&self, capturer: *const ffi::otc_video_capturer) -> OtcResult {
        *self.ptr.lock().unwrap() = Some(capturer);
        self.callbacks.lock().unwrap().init(self)
    }

    callback_call_with_return!(destroy, OtcResult);
    callback_call_with_return!(start, OtcResult);
    callback_call_with_return!(stop, OtcResult);

    fn get_capture_settings(&self, settings: *mut ffi::otc_video_capturer_settings) -> OtcResult {
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
