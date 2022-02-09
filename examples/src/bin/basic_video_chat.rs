extern crate opentok;

use opentok::log::{self, LogLevel};
use opentok::utils::publisher::Publisher;
use opentok::utils::subscriber::Subscriber;

#[path = "../cli.rs"]
mod cli;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let (credentials, duration) = cli::parse_cli().await?;

    opentok::init()?;

    log::enable_log(LogLevel::Error);
    log::logger_callback(Box::new(|msg| {
        println!("{:?}", msg);
    }));

    let credentials_ = credentials.clone();
    let on_stream_created = Box::new(move |_: &Publisher, stream_id: String| {
        let credentials = credentials_.clone();
        std::thread::spawn(move || {
            Subscriber::new(credentials, duration, None, Some(vec![stream_id]))
                .run()
                .unwrap();
        });
    });

    Publisher::new(credentials, Some(on_stream_created), duration).run()?;

    Ok(opentok::deinit()?)
}
