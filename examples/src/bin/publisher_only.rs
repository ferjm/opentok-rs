extern crate opentok;

use opentok::log::{self, LogLevel};
use opentok::publisher::{Publisher, PublisherCallbacks};
use opentok::session::{Session, SessionCallbacks};
use std::sync::{Arc, Mutex};

#[path = "../cli.rs"]
mod cli;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let credentials = cli::parse_cli().await?;

    opentok::init()?;

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
    let session = Session::new(
        &credentials.api_key,
        &credentials.session_id,
        session_callbacks,
    )?;
    session.connect(&credentials.token)?;

    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();

    Ok(opentok::deinit()?)
}
