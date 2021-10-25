use crate::common::Credentials;
use crate::renderer;

use opentok::audio_device::{set_render_callbacks, AudioDeviceCallbacks, AudioDeviceSettings};
use opentok::session::{Session, SessionCallbacks};
use opentok::subscriber::{Subscriber as OpenTokSubscriber, SubscriberCallbacks};
use opentok::video_frame::FramePlane;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Subscriber {
    credentials: Credentials,
    main_loop: glib::MainLoop,
    duration: Option<u64>,
}

impl Subscriber {
    pub fn new(credentials: Credentials, duration: Option<u64>) -> Self {
        Self {
            credentials,
            main_loop: glib::MainLoop::new(None, false),
            duration,
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let renderer: Arc<Mutex<Option<renderer::Renderer>>> = Arc::new(Mutex::new(None));
        let renderer_ = renderer.clone();
        let renderer__ = renderer.clone();

        let render_thread_running = Arc::new(AtomicBool::new(false));
        let render_thread_running_ = render_thread_running.clone();
        let render_thread_running__ = render_thread_running.clone();
        set_render_callbacks(
            AudioDeviceCallbacks::builder()
                .get_settings(|| -> AudioDeviceSettings {
                    AudioDeviceSettings {
                        sampling_rate: 44100,
                        number_of_channels: 1,
                    }
                })
                .start(move |device| {
                    let device = device.clone();
                    render_thread_running.store(true, Ordering::Relaxed);
                    let render_thread_running_ = render_thread_running.clone();
                    let renderer_ = renderer__.clone();
                    thread::spawn(move || loop {
                        if !render_thread_running_.load(Ordering::Relaxed) {
                            break;
                        }
                        if let Some(sample) = device.read_sample() {
                            if let Some(r) = renderer_.lock().unwrap().as_ref() {
                                r.render_audio_sample(sample);
                            }
                        }
                        thread::sleep(std::time::Duration::from_micros(10000));
                    });
                    Ok(())
                })
                .stop(move |_| {
                    render_thread_running_.store(false, Ordering::Relaxed);
                    Ok(())
                })
                .build(),
        )?;

        let subscriber_callbacks = SubscriberCallbacks::builder()
            .on_render_frame(move |_, frame| {
                let width = frame.get_width().unwrap() as u32;
                let height = frame.get_height().unwrap() as u32;

                let get_plane_size = |format, width: u32, height: u32| match format {
                    FramePlane::Y => width * height,
                    FramePlane::U | FramePlane::V => {
                        let pw = (width + 1) >> 1;
                        let ph = (height + 1) >> 1;
                        pw * ph
                    }
                    _ => unimplemented!(),
                };

                let offset = [
                    0,
                    get_plane_size(FramePlane::Y, width, height) as usize,
                    get_plane_size(FramePlane::Y, width, height) as usize
                        + get_plane_size(FramePlane::U, width, height) as usize,
                ];

                let stride = [
                    frame.get_plane_stride(FramePlane::Y).unwrap(),
                    frame.get_plane_stride(FramePlane::U).unwrap(),
                    frame.get_plane_stride(FramePlane::V).unwrap(),
                ];
                renderer
                    .lock()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .push_video_buffer(
                        frame.get_buffer().unwrap(),
                        frame.get_format().unwrap(),
                        width,
                        height,
                        &offset,
                        &stride,
                    );
            })
            .on_error(|_, error, _| {
                eprintln!("on_error {:?}", error);
            })
            .build();

        let subscriber = Arc::new(OpenTokSubscriber::new(subscriber_callbacks));

        let session_callbacks = SessionCallbacks::builder()
            .on_stream_received(move |session, stream| {
                *renderer_.lock().unwrap() = Some(renderer::Renderer::new().unwrap());
                println!(
                    "stream width {:?} height {:?}",
                    stream.get_video_width(),
                    stream.get_video_height()
                );
                if subscriber.set_stream(stream).is_ok() {
                    if let Err(e) = session.subscribe(&subscriber) {
                        eprintln!("Could not subscribe to session {:?}", e);
                    }
                }
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

        render_thread_running__.store(false, Ordering::Relaxed);

        Ok(())
    }

    pub fn stop(&self) {
        self.main_loop.quit();
    }
}
