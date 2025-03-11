use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.as_str() {
            "img" => {},
            "mp3" => {},
            "dll" => {},
            &_ => {}
        }
        }
}


async fn login() -> Client {
    let token: String = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents: GatewayIntents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::MESSAGE_CONTENT;
    let client: Client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");
    return client;
}
    
#[tokio::main]
async fn main() {
    let mut client: Client = login().await;
    if let Err(e) = client.start().await {
        println!("Client error: {e:?}");
    }
}