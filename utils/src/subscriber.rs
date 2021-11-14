use crate::common::Credentials;
use crate::renderer;

use opentok::audio_device::AudioDevice;
use opentok::session::{Session, SessionCallbacks};
use opentok::subscriber::{Subscriber as OpenTokSubscriber, SubscriberCallbacks};
use opentok::video_frame::FramePlane;
use std::sync::{Arc, Mutex};

pub struct Subscriber {
    credentials: Credentials,
    main_loop: glib::MainLoop,
    duration: Option<u64>,
    stream_id: Arc<Mutex<Option<String>>>,
    ignored_stream_ids: Arc<Mutex<Vec<String>>>,
}

impl Subscriber {
    pub fn new(
        credentials: Credentials,
        duration: Option<u64>,
        stream_id: Option<String>,
        ignored_stream_ids: Option<Vec<String>>,
    ) -> Self {
        Self {
            credentials,
            main_loop: glib::MainLoop::new(None, false),
            duration,
            stream_id: Arc::new(Mutex::new(stream_id)),
            ignored_stream_ids: Arc::new(Mutex::new(ignored_stream_ids.unwrap_or_default())),
        }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let renderer: Arc<Mutex<Option<renderer::Renderer>>> = Arc::new(Mutex::new(None));
        let renderer_ = renderer.clone();
        let renderer__ = renderer.clone();

        let audio_device = AudioDevice::get_instance();
        audio_device
            .lock()
            .unwrap()
            .set_on_audio_sample_callback(Box::new(move |sample| {
                if let Some(renderer) = renderer.lock().unwrap().as_ref() {
                    renderer.render_audio_sample(sample);
                }
            }));

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
                renderer_
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

        let stream_id = self.stream_id.clone();
        let ignored_stream_ids = self.ignored_stream_ids.clone();
        let session_callbacks = SessionCallbacks::builder()
            .on_stream_received(move |session, stream| {
                *renderer__.lock().unwrap() = Some(renderer::Renderer::new().unwrap());
                println!(
                    "stream width {:?} height {:?}",
                    stream.get_video_width(),
                    stream.get_video_height()
                );

                if let Some(ref stream_id) = *stream_id.lock().unwrap() {
                    if stream.id() != *stream_id {
                        println!("{} is not the stream we want to susbscribe to", stream_id);
                        return;
                    }
                }

                if ignored_stream_ids.lock().unwrap().contains(&stream.id()) {
                    println!("Ignoring stream {}", stream.id());
                    return;
                }

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

        session.disconnect().unwrap();

        Ok(())
    }

    pub fn stop(&self) {
        self.main_loop.quit();
    }
}
