use crate::capturer;
use crate::common::Credentials;

use opentok::audio_device::{AudioDevice, AudioDeviceSettings};
use opentok::publisher::{Publisher as OpenTokPublisher, PublisherCallbacks};
use opentok::session::{Session, SessionCallbacks};
use opentok::video_capturer::{VideoCapturer, VideoCapturerCallbacks, VideoCapturerSettings};
use opentok::video_frame::VideoFrame;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

type Callback = Box<dyn Fn(&Publisher, String) + Send + Sync + 'static>;

#[derive(Clone)]
pub struct Publisher {
    credentials: Credentials,
    main_loop: glib::MainLoop,
    on_stream_created: Arc<Mutex<Option<Callback>>>,
    duration: Option<u64>,
}

impl Publisher {
    pub fn new(
        credentials: Credentials,
        on_stream_created: Option<Callback>,
        duration: Option<u64>,
    ) -> Self {
        Self {
            credentials,
            main_loop: glib::MainLoop::new(None, false),
            on_stream_created: Arc::new(Mutex::new(on_stream_created)),
            duration,
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let audio_capture_thread_running = Arc::new(AtomicBool::new(true));
        let audio_capture_thread_running_ = audio_capture_thread_running.clone();

        let audio_device = AudioDevice::get_instance();

        thread::spawn(move || {
            let audio_capturer =
                capturer::AudioCapturer::new(&AudioDeviceSettings::default()).unwrap();
            loop {
                if !audio_capture_thread_running.load(Ordering::Relaxed) {
                    break;
                }
                if let Some(sample) = audio_capturer.pull_buffer() {
                    audio_device.lock().unwrap().push_audio_sample(sample);
                }
                thread::sleep(std::time::Duration::from_micros(10000));
            }
        });

        let render_thread_running = Arc::new(AtomicBool::new(false));
        let render_thread_running_ = render_thread_running.clone();
        let render_thread_running__ = render_thread_running.clone();
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

        let on_stream_created = self.on_stream_created.clone();
        let this = self.clone();
        let publisher_callbacks = PublisherCallbacks::builder()
            .on_stream_created(move |_, stream| {
                println!("Publishing stream with ID {}", stream.id());
                println!(
                    "opentok url {}",
                    format!(
                        "opentok://{}/{}?key={}&token={}",
                        this.credentials.session_id,
                        stream.id(),
                        this.credentials.api_key,
                        this.credentials.token
                    )
                );

                if let Some(ref callback) = *on_stream_created.lock().unwrap() {
                    callback(&this, stream.id());
                }
            })
            .on_error(|_, error, _| {
                println!("on_error {:?}", error);
            })
            .build();
        let publisher = Arc::new(Mutex::new(OpenTokPublisher::new(
            "publisher",
            Some(video_capturer),
            publisher_callbacks,
        )));

        let publisher_ = publisher.clone();
        let session_callbacks = SessionCallbacks::builder()
            .on_connected(move |session| {
                let _ = session.publish(&*publisher_.lock().unwrap());
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

        if let Some(duration) = self.duration {
            let main_loop = self.main_loop.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(duration));
                main_loop.quit();
            });
        }

        self.main_loop.run();

        audio_capture_thread_running_.store(false, Ordering::Relaxed);
        render_thread_running__.store(false, Ordering::Relaxed);

        publisher.lock().unwrap().unpublish().unwrap();

        session.disconnect().unwrap();

        Ok(())
    }

    pub fn stop(&self) {
        self.main_loop.quit();
    }
}
