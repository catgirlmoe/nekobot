use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::Client;
use serenity::framework::standard::StandardFramework;

use std::env;

mod commands;
mod events;

use events::Handler;
use env::var;

#[tokio::main]
async fn main() {
    let fw = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&commands::GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = var("DISCORD_TOKEN").expect("No token was found aeee");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(fw)
        .intents(GatewayIntents::all())
        .await
        .expect("An error");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}