/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

extern crate serde_json;

use serenity::client::bridge::gateway::GatewayIntents;
use serenity::client::Client;
use serenity::prelude::SerenityError;

pub mod config;
//pub mod db;
pub mod error;
pub mod events;

#[tokio::main]
async fn main() -> Result<(), SerenityError> {
  Client::builder((*config::DISCORD_TOKEN).to_owned())
    .event_handler(events::Handler)
    .application_id((*config::APPLICATION_ID).parse()?)
    .intents(GatewayIntents::all())
    .await?
    .start()
    .await?;
  Ok(())
}
