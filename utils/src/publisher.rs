use crate::capturer;
use crate::common::Credentials;

use opentok::audio_device::{set_capture_callbacks, AudioDeviceCallbacks, AudioDeviceSettings};
use opentok::publisher::{Publisher as OpenTokPublisher, PublisherCallbacks};
use opentok::session::{Session, SessionCallbacks};
use opentok::video_capturer::{VideoCapturer, VideoCapturerCallbacks, VideoCapturerSettings};
use opentok::video_frame::VideoFrame;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Publisher {
    credentials: Credentials,
    main_loop: glib::MainLoop,
}

impl Publisher {
    pub fn new(credentials: Credentials) -> Self {
        Self {
            credentials,
            main_loop: glib::MainLoop::new(None, false),
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let audio_capture_thread_running = Arc::new(AtomicBool::new(false));
        let audio_capture_thread_running_ = audio_capture_thread_running.clone();

        set_capture_callbacks(
            AudioDeviceCallbacks::builder()
                .get_settings(move || -> AudioDeviceSettings { AudioDeviceSettings::default() })
                .start(move |device| {
                    let device = device.clone();
                    audio_capture_thread_running.store(true, Ordering::Relaxed);
                    let audio_capture_thread_running_ = audio_capture_thread_running.clone();
                    let audio_capturer =
                        capturer::AudioCapturer::new(&AudioDeviceSettings::default()).unwrap();

                    thread::spawn(move || loop {
                        if !audio_capture_thread_running_.load(Ordering::Relaxed) {
                            break;
                        }
                        if let Some(data) = audio_capturer.pull_buffer() {
                            device.write_capture_data(data);
                        }
                        thread::sleep(std::time::Duration::from_micros(10000));
                    });
                    Ok(())
                })
                .stop(move |_| {
                    audio_capture_thread_running_.store(false, Ordering::Relaxed);
                    Ok(())
                })
                .build(),
        )?;

        let render_thread_running = Arc::new(AtomicBool::new(false));
        let render_thread_running_ = render_thread_running.clone();
        let video_capturer_callbacks = VideoCapturerCallbacks::builder()
            .start(move |video_capturer| {
                let video_capturer = video_capturer.clone();
                render_thread_running.store(true, Ordering::Relaxed);
                let render_thread_running_ = render_thread_running.clone();
                std::thread::spawn(move || {
                    let settings = VideoCapturerSettings::default();
                    let capturer = capturer::Capturer::new(&settings).unwrap();
                    let mut buf: Vec<u8> = vec![];
                    loop {
                        if !render_thread_running_.load(Ordering::Relaxed) {
                            break;
                        }
                        if let Ok(buffer) = capturer.pull_buffer() {
                            buf.extend_from_slice((*buffer).as_ref());
                            let frame = VideoFrame::new(
                                settings.format,
                                settings.width,
                                settings.height,
                                buf.clone(),
                            );
                            video_capturer.provide_frame(0, &frame).unwrap();
                            buf.clear();
                        }
                        std::thread::sleep(std::time::Duration::from_micros(30 * 1_000));
                    }
                });
                Ok(())
            })
            .stop(move |_| {
                render_thread_running_.store(false, Ordering::Relaxed);
                Ok(())
            })
            .build();
        let video_capturer = VideoCapturer::new(Default::default(), video_capturer_callbacks);

        let publisher_callbacks = PublisherCallbacks::builder()
            .on_stream_created(|_, _| {
                println!("on_stream_created");
            })
            .on_error(|_, error, _| {
                println!("on_error {:?}", error);
            })
            .build();
        let _publisher = Arc::new(Mutex::new(OpenTokPublisher::new(
            "publisher",
            Some(video_capturer),
            publisher_callbacks,
        )));

        let session_callbacks = SessionCallbacks::builder()
            .on_connected(move |session| {
                let _ = session.publish(&*_publisher.lock().unwrap());
            })
            .on_error(|_, error, _| {
                eprintln!("on_error {:?}", error);
            })
            .build();
        let session = Session::new(
            &self.credentials.api_key,
            &self.credentials.session_id,
            session_callbacks,
        )?;
        session.connect(&self.credentials.token)?;

        self.main_loop.run();

        Ok(())
    }

    pub fn stop(&self) {
        self.main_loop.quit();
    }
}
