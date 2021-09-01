use crate::enums::{IntoResult, OtcBool, OtcError, OtcResult};

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::os::raw::c_void;
use std::sync::{Arc, Mutex};
use thiserror::Error;

// TODO: Get rid of INSTANCES. We need this currently because of some of the
// macro calls remaining in this module.
lazy_static! {
    static ref INSTANCES: Arc<Mutex<HashMap<usize, AudioDevice>>> = Default::default();
    static ref SINGLETON: Mutex<Option<AudioDevice>> = Mutex::new(None);
}

#[derive(Error, Debug)]
pub enum AudioDeviceError {
    #[error("Unable to override render callbacks")]
    RenderCallbacksOverrideNotAllowed,
    #[error("Unable to override capture callbacks")]
    CaptureCallbacksOverrideNotAllowed,
    #[error("Audio device singleton not configured")]
    MissingAudioDevice,
    #[error("Audio device already initialized")]
    InitializationFailure,
}

/// Initialize the global AudioDevice. This is called immediately after the
/// library initialization. The AudioDevice has no associated callback by
/// default. The application call `set_render_callbacks()` and/or
/// `set_capture_callbacks()` to have the AudioDevice configured.
pub fn initialize() -> Result<(), AudioDeviceError> {
    if SINGLETON.lock().unwrap().is_some() {
        return Ok(());
    }
    let audio_device = AudioDevice::new().map_err(|_| AudioDeviceError::InitializationFailure)?;
    *SINGLETON.lock().unwrap() = Some(audio_device);
    Ok(())
}

/// Associate render callbacks to the AudioDevice singleton. This should be
/// called if the application wants to use a custom audio renderer.
pub fn set_render_callbacks(callbacks: AudioDeviceCallbacks) -> Result<(), AudioDeviceError> {
    if let Some(ref mut d) = *SINGLETON.lock().unwrap() {
        return d.set_render_callbacks(callbacks);
    }
    Err(AudioDeviceError::MissingAudioDevice)
}

/// Associate capture callbacks to the AudioDevice singleton. This should be
/// called if the application wants to use a custom audio capturer.
pub fn set_capture_callbacks(callbacks: AudioDeviceCallbacks) -> Result<(), AudioDeviceError> {
    if let Some(ref mut d) = *SINGLETON.lock().unwrap() {
        return d.set_capture_callbacks(callbacks);
    }
    Err(AudioDeviceError::MissingAudioDevice)
}

ffi_callback_with_return_user_data!(start_capturer, *const ffi::otc_audio_device, ffi::otc_bool);
ffi_callback_with_return_user_data!(stop_capturer, *const ffi::otc_audio_device, ffi::otc_bool);
ffi_callback_with_return_user_data!(start_renderer, *const ffi::otc_audio_device, ffi::otc_bool);
ffi_callback_with_return_user_data!(stop_renderer, *const ffi::otc_audio_device, ffi::otc_bool);

unsafe extern "C" fn get_capture_settings(
    _: *const ffi::otc_audio_device,
    user_data: *mut c_void,
    settings: *mut ffi::otc_audio_device_settings,
) -> ffi::otc_bool {
    if settings.is_null() {
        return false.into();
    }
    if let Some(callbacks) = &*INSTANCES
        .lock()
        .unwrap()
        .get(&(user_data as usize))
        .unwrap()
        .capture_callbacks
        .lock()
        .unwrap()
    {
        let s = callbacks.get_settings();
        (*settings).sampling_rate = s.sampling_rate;
        (*settings).number_of_channels = s.number_of_channels;
        true.into()
    } else {
        false.into()
    }
}

unsafe extern "C" fn get_render_settings(
    _: *const ffi::otc_audio_device,
    user_data: *mut c_void,
    settings: *mut ffi::otc_audio_device_settings,
) -> ffi::otc_bool {
    if settings.is_null() {
        return false.into();
    }
    if let Some(callbacks) = &*INSTANCES
        .lock()
        .unwrap()
        .get(&(user_data as usize))
        .unwrap()
        .render_callbacks
        .lock()
        .unwrap()
    {
        let s = callbacks.get_settings();
        (*settings).sampling_rate = s.sampling_rate;
        (*settings).number_of_channels = s.number_of_channels;
        true.into()
    } else {
        false.into()
    }
}

#[allow(clippy::type_complexity)]
pub struct AudioDeviceCallbacks {
    start: Option<Box<dyn Fn(&AudioDevice) -> OtcResult + Send + Sync + 'static>>,
    stop: Option<Box<dyn Fn(&AudioDevice) -> OtcResult + Send + Sync + 'static>>,
    get_settings: Option<Box<dyn Fn() -> AudioDeviceSettings + Send + Sync + 'static>>,
}

impl AudioDeviceCallbacks {
    pub fn builder() -> AudioDeviceCallbacksBuilder {
        AudioDeviceCallbacksBuilder::default()
    }

    callback_with_return!(start, &AudioDevice, OtcResult);
    callback_with_return!(stop, &AudioDevice, OtcResult);

    pub fn get_settings(&self) -> AudioDeviceSettings {
        let callback = self.get_settings.as_ref().unwrap();
        callback()
    }
}

#[derive(Default)]
#[allow(clippy::type_complexity)]
pub struct AudioDeviceCallbacksBuilder {
    start: Option<Box<dyn Fn(&AudioDevice) -> OtcResult + Send + Sync + 'static>>,
    stop: Option<Box<dyn Fn(&AudioDevice) -> OtcResult + Send + Sync + 'static>>,
    get_settings: Option<Box<dyn Fn() -> AudioDeviceSettings + Send + Sync + 'static>>,
}

impl AudioDeviceCallbacksBuilder {
    callback_setter_with_return!(start, &AudioDevice, OtcResult);
    callback_setter_with_return!(stop, &AudioDevice, OtcResult);

    pub fn get_settings<F: Fn() -> AudioDeviceSettings + Send + Sync + 'static>(
        self,
        callback: F,
    ) -> Self {
        Self {
            get_settings: Some(Box::new(callback)),
            ..self
        }
    }

    pub fn build(self) -> AudioDeviceCallbacks {
        AudioDeviceCallbacks {
            start: self.start,
            stop: self.stop,
            get_settings: self.get_settings,
        }
    }
}

/// Raw data holder for audio samples.
pub struct AudioSampleData(pub Vec<i16>);

/// High level storage for audio samples. The data size should correspond with
/// the sampling_rate and number_of_channels (size = (sampling_rate / 1000) *
/// number_of_channels).
pub struct AudioSample {
    pub data: AudioSampleData,
    pub sampling_rate: i32,
    pub number_of_channels: i32,
}

/// Settings for a AudioDevice.
#[derive(Clone, Debug, Copy)]
pub struct AudioDeviceSettings {
    pub sampling_rate: i32,
    pub number_of_channels: i32,
}

#[derive(Clone)]
pub struct AudioDevice {
    instance_id: usize,
    render_callbacks: Arc<Mutex<Option<AudioDeviceCallbacks>>>,
    capture_callbacks: Arc<Mutex<Option<AudioDeviceCallbacks>>>,
    ffi_callbacks: Arc<Mutex<ffi::otc_audio_device_callbacks>>,
}

unsafe impl Send for AudioDevice {}
unsafe impl Sync for AudioDevice {}

impl AudioDevice {
    fn new() -> Result<Self, OtcError> {
        let instance_id = INSTANCES.lock().unwrap().len() + 1;
        let device = Self {
            instance_id,
            render_callbacks: Arc::new(Mutex::new(None)),
            capture_callbacks: Arc::new(Mutex::new(None)),
            ffi_callbacks: Arc::new(Mutex::new(ffi::otc_audio_device_callbacks {
                init: None,
                destroy: None,
                init_capturer: None,
                destroy_capturer: None,
                start_capturer: Some(start_capturer),
                stop_capturer: Some(stop_capturer),
                is_capturer_initialized: None,
                is_capturer_started: None,
                get_estimated_capture_delay: None,
                get_capture_settings: Some(get_capture_settings),
                init_renderer: None,
                destroy_renderer: None,
                start_renderer: Some(start_renderer),
                stop_renderer: Some(stop_renderer),
                is_renderer_initialized: None,
                is_renderer_started: None,
                get_estimated_render_delay: None,
                get_render_settings: Some(get_render_settings),
                user_data: instance_id as *mut c_void,
                reserved: std::ptr::null_mut(),
            })),
        };
        INSTANCES
            .lock()
            .unwrap()
            .insert(instance_id, device.clone());

        let r = unsafe {
            ffi::otc_set_audio_device(
                &*device.ffi_callbacks.lock().unwrap() as *const ffi::otc_audio_device_callbacks
            )
            .into_result()
        };

        r.map(|_| device)
    }

    pub fn read_sample(&self) -> Option<AudioSample> {
        match &*self.render_callbacks.lock().unwrap() {
            Some(c) => {
                let settings = c.get_settings();
                let size = (settings.sampling_rate / 100) * settings.number_of_channels;
                let d = unsafe {
                    let mut data = Vec::with_capacity(size as usize);
                    let size =
                        ffi::otc_audio_device_read_render_data(data.as_mut_ptr(), size as u64);
                    data.set_len(size as usize);
                    data
                };
                if !d.is_empty() {
                    let sample = AudioSample {
                        data: AudioSampleData(d),
                        sampling_rate: settings.sampling_rate,
                        number_of_channels: settings.number_of_channels,
                    };
                    return Some(sample);
                }
                None
            }
            None => None,
        }
    }

    pub fn write_capture_data(&self, data: AudioSampleData) {
        let size = data.0.len();
        unsafe { ffi::otc_audio_device_write_capture_data(data.0.as_ptr(), size as u64) };
    }

    fn set_render_callbacks(
        &mut self,
        callbacks: AudioDeviceCallbacks,
    ) -> Result<(), AudioDeviceError> {
        let render_callbacks = self.render_callbacks.lock();
        if render_callbacks.as_ref().unwrap().is_some() {
            return Err(AudioDeviceError::RenderCallbacksOverrideNotAllowed);
        }
        *render_callbacks.unwrap() = Some(callbacks);
        Ok(())
    }

    fn set_capture_callbacks(
        &mut self,
        callbacks: AudioDeviceCallbacks,
    ) -> Result<(), AudioDeviceError> {
        let capture_callbacks = self.capture_callbacks.lock();
        if capture_callbacks.as_ref().unwrap().is_some() {
            return Err(AudioDeviceError::CaptureCallbacksOverrideNotAllowed);
        }
        *capture_callbacks.unwrap() = Some(callbacks);
        Ok(())
    }

    fn start_capturer(&self) -> OtcResult {
        if let Some(c) = &*self.capture_callbacks.lock().unwrap() {
            return c.start(self);
        }
        Ok(())
    }
    fn stop_capturer(&self) -> OtcResult {
        if let Some(c) = &*self.capture_callbacks.lock().unwrap() {
            return c.stop(self);
        }
        Ok(())
    }
    fn start_renderer(&self) -> OtcResult {
        if let Some(c) = &*self.render_callbacks.lock().unwrap() {
            return c.start(self);
        }
        Ok(())
    }
    fn stop_renderer(&self) -> OtcResult {
        if let Some(c) = &*self.render_callbacks.lock().unwrap() {
            return c.stop(self);
        }
        Ok(())
    }
}
