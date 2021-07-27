extern crate opentok;

use opentok::log::{self, LogLevel};
use opentok::publisher::{Publisher, PublisherCallbacks};
use opentok::session::{Session, SessionCallbacks};
use std::env;
use std::sync::{Arc, Mutex};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: subscriber_only <api key> <session ID> <token>");
        std::process::exit(-1);
    }

    let api_key: &str = args[1].as_ref();
    let session_id: &str = args[2].as_ref();
    let token: &str = args[3].as_ref();

    let _ = opentok::init();

    log::enable_log(LogLevel::Info);

    let publisher_callbacks = PublisherCallbacks::builder()
        .on_stream_created(|_, _| {
            println!("on_stream_created");
        })
        .on_error(|_, error, _| {
            println!("on_error {:?}", error);
        })
        .build();
    let _publisher = Arc::new(Mutex::new(Publisher::new(
        "basic_video_chat",
        None,
        publisher_callbacks,
    )));

    let session_callbacks = SessionCallbacks::builder()
        .on_connection_created(|_, _| {
            println!("on_connection_created");
        })
        .on_connected(move |session| {
            println!("on_connected");
            let _ = session.publish(&*_publisher.lock().unwrap());
        })
        .on_disconnected(|_| {
            println!("on_disconnected");
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
