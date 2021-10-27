/// The OpenTok SDK handles all audio related work through a singleton instance
/// of `otc_audio_device` which persists throughout the lifetime of the OpenTok
/// library.
///
/// Unfortunately, the SDK expose no way of having a dedicated audio device
/// per session, which makes many use cases that differs from the basic video
/// chat demo pretty hard to implement. In summary, if you want independent
/// audio devices, you likely need to have a multiprocess application, where
/// an independent opentok::init is executed per process.
///
/// Likewise, there is currently no way to get independent audio samples
/// per participant in a session. The SDK exposes an audio stream which is
/// a mix of all participants' audio streams. Check
/// <https://github.com/opentok/opentok-linux-sdk-samples/issues/25>
use crate::enums::{IntoResult, OtcBool, OtcResult};

use lazy_static::lazy_static;
use log::warn;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use thiserror::Error;

lazy_static! {
    static ref SINGLETON: Arc<Mutex<AudioDevice>> = Arc::new(Mutex::new(AudioDevice::new()));
}

#[derive(Error, Debug)]
pub enum AudioDeviceError {
    #[error("Audio device singleton not configured")]
    MissingAudioDevice,
    #[error("Audio device already initialized")]
    InitializationFailure,
}

ffi_callback_with_return_singleton!(start_capturer, *const ffi::otc_audio_device, ffi::otc_bool);
ffi_callback_with_return_singleton!(stop_capturer, *const ffi::otc_audio_device, ffi::otc_bool);
ffi_callback_with_return_singleton!(start_renderer, *const ffi::otc_audio_device, ffi::otc_bool);
ffi_callback_with_return_singleton!(stop_renderer, *const ffi::otc_audio_device, ffi::otc_bool);

unsafe extern "C" fn get_capture_settings(
    _: *const ffi::otc_audio_device,
    _: *mut c_void,
    settings: *mut ffi::otc_audio_device_settings,
) -> ffi::otc_bool {
    if settings.is_null() {
        return false.into();
    }
    if let Ok(singleton) = SINGLETON.try_lock() {
        let capture_settings = singleton.capture_settings();
        (*settings).sampling_rate = capture_settings.sampling_rate;
        (*settings).number_of_channels = capture_settings.number_of_channels;
        true
    } else {
        false
    }
    .into()
}

unsafe extern "C" fn get_render_settings(
    _: *const ffi::otc_audio_device,
    _: *mut c_void,
    settings: *mut ffi::otc_audio_device_settings,
) -> ffi::otc_bool {
    if settings.is_null() {
        return false.into();
    }
    if let Ok(singleton) = SINGLETON.try_lock() {
        let render_settings = singleton.render_settings();
        (*settings).sampling_rate = render_settings.sampling_rate;
        (*settings).number_of_channels = render_settings.number_of_channels;
        true
    } else {
        false
    }
    .into()
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

impl Default for AudioDeviceSettings {
    fn default() -> AudioDeviceSettings {
        AudioDeviceSettings {
            sampling_rate: 44100,
            number_of_channels: 1,
        }
    }
}

type OnAudioSampleCallback = Box<dyn Fn(AudioSample) + Send + Sync + 'static>;

#[derive(Clone)]
pub struct AudioDevice {
    ffi_callbacks: Arc<Mutex<ffi::otc_audio_device_callbacks>>,
    capturer_ready: Arc<AtomicBool>,
    capture_settings: Arc<Mutex<AudioDeviceSettings>>,
    render_thread_running: Arc<AtomicBool>,
    render_settings: Arc<Mutex<AudioDeviceSettings>>,
    on_audio_sample_callbacks: Arc<Mutex<Vec<OnAudioSampleCallback>>>,
}

unsafe impl Send for AudioDevice {}
unsafe impl Sync for AudioDevice {}

impl AudioDevice {
    fn new() -> Self {
        let device = Self {
            capturer_ready: Default::default(),
            capture_settings: Default::default(),
            render_thread_running: Default::default(),
            render_settings: Default::default(),
            on_audio_sample_callbacks: Default::default(),
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
                user_data: std::ptr::null_mut(),
                reserved: std::ptr::null_mut(),
            })),
        };

        unsafe {
            if let Err(e) =
                ffi::otc_set_audio_device(&*device.ffi_callbacks.lock().unwrap()
                    as *const ffi::otc_audio_device_callbacks)
                .into_result()
            {
                warn!("Could not set audio device callbacks. {}", e);
            }
        }

        device
    }

    pub fn get_instance() -> Arc<Mutex<AudioDevice>> {
        SINGLETON.clone()
    }

    pub fn stop() {
        let _ = SINGLETON.lock().unwrap().stop_renderer();
    }

    pub fn override_capture_settings(&self, settings: AudioDeviceSettings) {
        *self.capture_settings.lock().unwrap() = settings;
    }

    pub fn override_render_settings(&self, settings: AudioDeviceSettings) {
        *self.render_settings.lock().unwrap() = settings;
    }

    fn capture_settings(&self) -> AudioDeviceSettings {
        *self.capture_settings.lock().unwrap()
    }

    fn render_settings(&self) -> AudioDeviceSettings {
        *self.render_settings.lock().unwrap()
    }

    pub fn set_on_audio_sample_callback(&self, callback: OnAudioSampleCallback) {
        self.on_audio_sample_callbacks
            .lock()
            .unwrap()
            .push(callback);
    }

    pub fn push_audio_sample(&self, data: AudioSampleData) {
        if !self.capturer_ready.load(Ordering::Relaxed) {
            warn!("Audio capturer is not ready yet. Dropping audio sample");
            return;
        }
        let size = data.0.len();
        unsafe { ffi::otc_audio_device_write_capture_data(data.0.as_ptr(), size as u64) };
    }

    fn start_capturer(&self) -> OtcResult {
        self.capturer_ready.store(true, Ordering::Relaxed);
        Ok(())
    }

    fn stop_capturer(&self) -> OtcResult {
        self.capturer_ready.store(false, Ordering::Relaxed);
        Ok(())
    }

    fn start_renderer(&self) -> OtcResult {
        self.render_thread_running.store(true, Ordering::Relaxed);
        let render_thread_running = self.render_thread_running.clone();
        let sampling_rate = self.render_settings.lock().unwrap().sampling_rate;
        let number_of_channels = self.render_settings.lock().unwrap().number_of_channels;
        let size = (sampling_rate / 100) * number_of_channels;
        let on_audio_sample_callbacks = self.on_audio_sample_callbacks.clone();
        std::thread::spawn(move || loop {
            if !render_thread_running.load(Ordering::Relaxed) {
                break;
            }
            let data = unsafe {
                let mut data = Vec::with_capacity(size as usize);
                let size = ffi::otc_audio_device_read_render_data(data.as_mut_ptr(), size as u64);
                data.set_len(size as usize);
                data
            };
            if data.is_empty() {
                continue;
            }
            if let Ok(callbacks) = on_audio_sample_callbacks.try_lock() {
                for ref callback in callbacks.iter() {
                    callback(AudioSample {
                        data: AudioSampleData(data.clone()),
                        sampling_rate,
                        number_of_channels,
                    });
                }
            }
            std::thread::sleep(std::time::Duration::from_micros(10000));
        });
        Ok(())
    }

    fn stop_renderer(&self) -> OtcResult {
        self.render_thread_running.store(false, Ordering::Relaxed);
        Ok(())
    }
}
