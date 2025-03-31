mod common_parse;
mod roll;
mod roll_command;
mod secrets;

use log::{error, warn};
use roll_command::parse_roll_command;
use secrets::Secrets;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use simplelog::*;

struct Bot;

const SECRETS_PATH: &str = "./Secrets.toml";
const DISCORD_MESSAGE_LENGTH_MAX: usize = 2000;
const PREFIXES: [&str; 3] = ["!roll ", "!r ", "!plsrollformedicechan "];

async fn process_roll_command(command: &str, ctx: &Context, msg: &Message) {
    let (output, batches) = match parse_roll_command(command) {
        Ok((remainder, result)) => {
            if remainder.trim().is_empty() {
                (result.evaluate(), result.batch_count())
            } else {
                (format!("Error, unexpected character: {}", remainder), 0)
            }
        }
        Err(nom::Err::Error(err)) => (format!("Error parsing roll command: {}", err.input), 0),
        Err(nom::Err::Incomplete(_)) => (
            "Error, failed to parse roll command: Incomplete expression".to_string(),
            0,
        ),
        Err(nom::Err::Failure(err)) => (format!("Failure to parse roll command: {}", err.input), 0),
    };

    let mut message = if let Some(nickname) = msg.author_nick(&ctx.http).await {
        format!("{} requested `[{}]` ", nickname, command)
    } else {
        format!("{} requested `[{}]` ", msg.author.name, command)
    };

    if batches > 1 {
        message.push_str("Rolls:\n");
    } else {
        message.push_str("Roll: ");
    }

    message.push_str(&output);

    if message.len() > DISCORD_MESSAGE_LENGTH_MAX {
        message = format!(
            "Error, output length exceeds {} characters",
            DISCORD_MESSAGE_LENGTH_MAX
        );
    }

    if let Err(e) = msg.channel_id.say(&ctx.http, message).await {
        error!("Error sending message: {:?}", e)
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        for prefix in PREFIXES {
            if let Some(text) = msg.content.strip_prefix(prefix) {
                process_roll_command(text, &ctx, &msg).await;
                return;
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        warn!("{} is connected!", ready.user.name);
    }
}

fn read_discord_token(secrets_path: &str) -> Result<Secrets, String> {
    let secrets_contents = std::fs::read_to_string(secrets_path)
        .map_err(|e| format!("Could not read secrets: {}", e))?;

    let secrets =
        toml::from_str(&secrets_contents).map_err(|e| format!("Could not read secrets: {}", e))?;

    Ok(secrets)
}

#[tokio::main]
async fn main() {
    TermLogger::init(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .expect("Could not initialize logging");

    // Get the discord token set in `Secrets.toml`
    let token = match read_discord_token(SECRETS_PATH) {
        Ok(secrets) => secrets.discord_token().to_string(),
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    if let Err(err) = client.start().await {
        println!("Could not start discord bot: {}", err);
    }
}
