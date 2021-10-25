extern crate opentok;

use opentok::log::{self, LogLevel};
use opentok_utils::publisher::Publisher;
use opentok_utils::subscriber::Subscriber;

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
    std::thread::spawn(move || {
        Subscriber::new(credentials_, duration).run().unwrap();
    });

    Publisher::new(credentials, None, duration).run()?;

    Ok(opentok::deinit()?)
}
