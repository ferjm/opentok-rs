extern crate opentok;

use opentok::log::{self, LogLevel};
use opentok::publisher::{Publisher, PublisherCallbacks};
use opentok::session::{Session, SessionCallbacks};
use std::sync::{Arc, Mutex};

#[path = "../cli.rs"]
mod cli;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let (credentials, duration) = cli::parse_cli().await?;

    opentok::init()?;

    log::enable_log(LogLevel::Info);

    let credentials_ = credentials.clone();
    let publisher_callbacks = PublisherCallbacks::builder()
        .on_stream_created(move |_, stream| {
            println!("on_stream_created {}", stream.id());
            println!(
                "opentok url {}",
                format!(
                    "opentok://{}/{}?key={}&token={}",
                    credentials_.session_id,
                    stream.id(),
                    credentials_.api_key,
                    credentials_.token
                )
            );
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

    if let Some(duration) = duration {
        let main_loop = main_loop.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(duration));
            main_loop.quit();
        });
    }

    main_loop.run();

    Ok(opentok::deinit()?)
}
