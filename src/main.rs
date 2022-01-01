mod config;

use config::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let conf = Config::try_from_path("./parachute.toml")?;

    println!("token: {}", conf.discord.token);

    Ok(())
}
