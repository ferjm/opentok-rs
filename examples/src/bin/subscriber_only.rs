extern crate opentok;

use opentok::log::{self, LogLevel};
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

    Subscriber::new(credentials, duration, None, None).run()?;

    Ok(opentok::deinit()?)
}
