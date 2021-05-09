/*
//  Copyright 2021 catgirl.moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::Client;
use std::env;
use env::var;

pub mod consts;
pub mod event;
pub mod utils;

#[tokio::main]
async fn main() {
  let token = var("DISCORD_TOKEN").expect("No token was found");
  let mut client = Client::builder(token)
    .event_handler(event::Handler)
    .intents(GatewayIntents::all())
    .await
    .expect("An error occured during startup");

  // start listening for events by starting a single shard
  if let Err(why) = client.start().await {
    println!("An error occurred while running the client: {:?}", why);
  }
}