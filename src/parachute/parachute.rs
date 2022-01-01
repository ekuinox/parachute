use anyhow::Result;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;

pub(crate) use super::DiscordConfig;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

pub(crate) struct Parachute {
    inner: Client,
}

impl Parachute {
    pub async fn build(conf: &DiscordConfig) -> Result<Parachute> {
        let framework = StandardFramework::new()
            .configure(|c| c.prefix(&conf.prefix.0)) // set the bot's prefix to "~"
            .group(&GENERAL_GROUP);

        let client = Client::builder(&conf.token)
            .event_handler(Handler)
            .framework(framework)
            .await
            .expect("Error creating client");

        Ok(Parachute { inner: client })
    }

    pub async fn start(&mut self) -> Result<()> {
        let _ = self.inner.start().await?;
        Ok(())
    }
}
