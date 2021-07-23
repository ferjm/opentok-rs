extern crate opentok;

use opentok::log::{self, LogLevel};
use opentok::publisher::{Publisher, PublisherCallbacks};
use opentok::session::{Session, SessionCallbacks};
use opentok::video_capturer::{VideoCapturer, VideoCapturerCallbacks};
use std::env;
use std::sync::Arc;

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

    log::enable_log(LogLevel::Info);

    let video_capturer_callbacks = VideoCapturerCallbacks::builder()
        .init(|_| {
            println!("video capturer init");
            Ok(())
        })
        .destroy(|_| {
            println!("video capturer destroy");
            Ok(())
        })
        .start(|_| {
            println!("video capturer start");
            Ok(())
        })
        .stop(|_| Ok(()))
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
    let _publisher = Arc::new(Publisher::new(
        "basic_video_chat",
        Some(video_capturer),
        publisher_callbacks,
    ));

    let session_callbacks = SessionCallbacks::builder()
        .on_connection_created(|_, _| {
            println!("on_connection_created");
        })
        .on_connected(move |session| {
            println!("on_connected");
            let _ = session.publish(&*_publisher);
        })
        .on_disconnected(|_| {
            println!("on_disconnected");
        })
        .on_error(|_, error, _| {
            println!("on_error {:?}", error);
        })
        .build();
    let session = match Session::new(api_key, session_id, session_callbacks) {
        Ok(session) => session,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };
    let _ = session.connect(token);

    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();

    let _ = opentok::deinit();
}
