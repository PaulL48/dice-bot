mod common_parse;
mod roll;
mod roll_command;

use anyhow::anyhow;
use roll_command::parse_roll_command;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let text = &msg.content;

        const PREFIX: &str = "!roll ";
        if let Some(text) = text.strip_prefix(PREFIX) {
            let (output, batches) = match parse_roll_command(text) {
                Ok((remainder, result)) => {
                    if remainder.trim().is_empty() {
                        (result.evaluate(), result.batch_count())
                    } else {
                        (format!("Error, unexpected character: {}", remainder), 0)
                    }
                }
                Err(nom::Err::Error(err)) => {
                    (format!("Error parsing roll command: {}", err.input), 0)
                }
                Err(nom::Err::Incomplete(_)) => (
                    "Error, failed to parse roll command: Incomplete expression".to_string(),
                    0,
                ),
                Err(nom::Err::Failure(err)) => {
                    (format!("Failure to parse roll command: {}", err.input), 0)
                }
            };

            let mut message = if let Some(nickname) = msg.author_nick(&ctx.http).await {
                format!("{} requested `[{}]` ", nickname, text)
            } else {
                format!("{} requested `[{}]` ", msg.author.name, text)
            };

            if batches > 1 {
                message.push_str("Rolls:\n");
            } else {
                message.push_str("Roll: ");
            }

            message.push_str(&output);

            if let Err(e) = msg.channel_id.say(&ctx.http, message).await {
                error!("Error sending message: {:?}", e)
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
