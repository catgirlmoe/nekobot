use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::prelude::Interaction;

pub struct Handler;

use std::fmt::Debug;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, intr: Interaction) {
        let data = &intr.data.clone().unwrap();
        if data.name=="status" {
            intr.create_interaction_response(&ctx.http, |i| {
                i.interaction_response_data(|f| {
                    f.content("fuck off")
                })
            }).await.expect("ae");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "Connected to discord as {}#{}",
            ready.user.name, ready.user.discriminator
        );
    }
}
