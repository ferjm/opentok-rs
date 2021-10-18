extern crate opentok;

use opentok::log::{self, LogLevel};
use opentok_utils::publisher::Publisher;

#[path = "../cli.rs"]
mod cli;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let credentials = cli::parse_cli().await?;

    opentok::init()?;

    log::enable_log(LogLevel::Error);

    Publisher::new(credentials, None).run()?;

    Ok(opentok::deinit()?)
}
