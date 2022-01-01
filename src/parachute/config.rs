use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct DiscordCommandPrefix(pub String);

impl Default for DiscordCommandPrefix {
    fn default() -> Self {
        DiscordCommandPrefix("~".into())
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct DiscordConfig {
    pub token: String,

    #[serde(default)]
    pub prefix: DiscordCommandPrefix,
}
