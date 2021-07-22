extern crate opentok;

use opentok::log::{self, LogLevel};
use opentok::publisher::{Publisher, PublisherCallbacks};
use opentok::session::{Session, SessionCallbacks};
use opentok::subscriber::{Subscriber, SubscriberCallbacks};
use opentok::video_frame::FramePlane;
use std::env;
use std::sync::{Arc, Mutex};

#[path = "../renderer.rs"]
mod renderer;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: basic_video_chat <api key> <session ID> <token>");
        std::process::exit(-1);
    }

    let api_key: &str = args[1].as_ref();
    let session_id: &str = args[2].as_ref();
    let token: &str = args[3].as_ref();

    let _ = opentok::init();

    log::enable_log(LogLevel::Disabled);

    let renderer: Arc<Mutex<Option<renderer::Renderer>>> = Arc::new(Mutex::new(None));
    let renderer_ = renderer.clone();

    let publisher_callbacks = PublisherCallbacks::builder()
        .on_error(|_, error, _| {
            println!("on_error {:?}", error);
        })
        .build();
    let _publisher = Arc::new(Publisher::new(
        "basic_video_chat",
        None,
        publisher_callbacks,
    ));

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
            renderer.lock().unwrap().as_ref().unwrap().push_buffer(
                frame.get_buffer().unwrap(),
                frame.get_format().unwrap(),
                width,
                height,
                &offset,
                &stride,
            );
        })
        .on_error(|_, error, _| {
            println!("on_error {:?}", error);
        })
        .build();

    let subscriber = Arc::new(Subscriber::new(subscriber_callbacks));

    let session_callbacks = SessionCallbacks::builder()
        .on_connected(move |session| {
            let _ = session.publish(&*_publisher);
        })
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
            println!("on_error {:?}", error);
        })
        .build();
    let session = Session::new(api_key, session_id, session_callbacks).unwrap();
    let _ = session.connect(token);

    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();

    let _ = opentok::deinit();
}
