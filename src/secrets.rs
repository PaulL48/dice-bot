use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Secrets {
    #[serde(rename = "DISCORD_TOKEN")]
    discord_token: String,
}

impl Secrets {
    pub fn discord_token(&self) -> &str {
        &self.discord_token
    }
}
