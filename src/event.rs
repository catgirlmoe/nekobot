/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::consts::*;
use crate::utils::*;

use serenity::{
  async_trait,
  client::{Context, EventHandler},
  model::{
    channel::Reaction,
    guild::Member,
    id::{ChannelId, GuildId},
    prelude::{Ready, User},
  },
  utils::Colour,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, _ctx: Context, ready: Ready) {
    println!(
      "Connected as {}#{}",
      ready.user.name, ready.user.discriminator
    );
  }

  async fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, member: Member) {
    ChannelId(GENERAL)
      .send_message(&ctx.http, |m| {
        m.embed(|e| {
          let u = member.user;
          e.author(|a| {
            a.icon_url(u.avatar_url().expect("Error retrieving avatar"));
            a.name(format!("{}#{}", u.name, u.discriminator));
            a.url(format!("https://discord.com/users/{}", u.id))
          });
          e.colour(Colour::from_rgb(139, 195, 74));
          e.description(format!("Welcome <@{}> to the server!", u.id))
        })
      })
      .await
      .expect("Failed to send welcome");
  }

  async fn guild_member_removal(
    &self, ctx: Context, _guild_id: GuildId, u: User, _member: Option<Member>,
  ) {
    ChannelId(GENERAL)
      .send_message(&ctx.http, |m| {
        m.embed(|e| {
          e.author(|a| {
            a.icon_url(u.avatar_url().expect("Error retrieving avatar"));
            a.name(format!("{}#{}", u.name, u.discriminator));
            a.url(format!("https://discord.com/users/{}", u.id))
          });
          e.colour(Colour::from_rgb(244, 67, 54));
          e.description(format!("<@{}> has left the server!", u.id))
        })
      })
      .await
      .expect("Failed to send welcome");
  }

  async fn reaction_add(&self, ctx: Context, r: Reaction) {
    if r.user_id.expect("Failed to get user id") == BOTID {
      return;
    }
    match r.message_id.as_u64() {
      COLOR_MSG => give_role(COLOR_ROLES, COLOR_REACTS, ctx, r).await,
      OPT_MSG => give_role(OPT_ROLES, OPT_REACTS, ctx, r).await,
      ACCESS_MSG => give_role(ACCESS_ROLES, ACCESS_REACTS, ctx, r).await,
      CC_MSG => give_role(CC_ROLES, CC_REACTS, ctx, r).await,
      _ => {}
    }
  }

  async fn reaction_remove(&self, ctx: Context, r: Reaction) {
    if r.user_id.expect("Failed to get user id") == BOTID {
      return;
    }
    match r.message_id.as_u64() {
      COLOR_MSG => take_role(COLOR_ROLES, COLOR_REACTS, ctx, r).await,
      OPT_MSG => take_role(OPT_ROLES, OPT_REACTS, ctx, r).await,
      ACCESS_MSG => take_role(ACCESS_ROLES, ACCESS_REACTS, ctx, r).await,
      CC_MSG => take_role(CC_ROLES, CC_REACTS, ctx, r).await,
      _ => {}
    }
  }
}
