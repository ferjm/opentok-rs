extern crate opentok;

use opentok::publisher::{Publisher, PublisherCallbacks};
use opentok::session::{Session, SessionCallbacks};
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

    let publisher_callbacks = PublisherCallbacks {};
    let _publisher = Arc::new(Publisher::new(
        "basic_video_chat",
        None,
        publisher_callbacks,
    ));

    let callbacks = SessionCallbacks::builder()
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
        .on_error(|_, _, _| {
            println!("on_error");
        });
    let session = Session::new(api_key, session_id, callbacks.build()).unwrap();
    let _ = session.connect(token);

    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();

    let _ = opentok::deinit();
}
