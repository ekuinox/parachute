mod config;
mod parachute;

use config::*;
use parachute::Parachute;
use anyhow::Result;
use dotenv::dotenv;
use std::env;

const CONFIG_ENV_KEY: &'static str = "PARACHUTE_CONFIG_PATH";

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv();
    let path = env::var(CONFIG_ENV_KEY)?;
    let conf = Config::try_from_path(&path)?;

    dbg!(&conf);

    let mut parachute = Parachute::build(&conf.discord).await?;

    let _ = parachute.start().await?;

    Ok(())
}
