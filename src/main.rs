/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::Client;

// pub mod config;
// pub mod consts;
// pub mod event;
// pub mod utils;

#[tokio::main]
async fn main() {
  let mut client = Client::builder((*config::PORT).to_owned())
    .event_handler(event::Handler)
    .intents(GatewayIntents::all())
    .await
    .expect("An error occured during startup");

  // start listening for events by starting a single shard
  if let Err(why) = client.start().await {
    println!("An error occurred while running the client: {:?}", why);
  }
}
