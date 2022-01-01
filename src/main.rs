mod config;

use config::*;
use anyhow::Result;
use dotenv::dotenv;
use std::env;

const CONFIG_ENV_KEY: &'static str = "PARACHUTE_CONFIG_PATH";

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv();
    let path = env::var(CONFIG_ENV_KEY)?;
    let conf = Config::try_from_path(&path)?;

    println!("token: {}", conf.discord.token);

    Ok(())
}
