/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use serenity::{async_trait, model::{gateway::Ready, guild::Member, id::GuildId, interactions::{application_command::ApplicationCommand, Interaction}, prelude::User}, prelude::*, utils::Colour};

use crate::config;

mod interaction;
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn interaction_create(&self, ctx: Context, inter: Interaction) {
    use serenity::model::prelude::Interaction::*;
    let result = match inter {
      ApplicationCommand(com) => interaction::application_command(ctx, com).await,
      MessageComponent(mc) => interaction::message_component(ctx, mc).await,
      _ => Ok(()),
    };

    if let Err(err) = result {
      println!("{}", err);
    }
  }

  async fn guild_member_addition(&self, ctx: Context, _: GuildId, member: Member) {
    config::CHANNEL_ID.send_message(&ctx.http, |m| {
      m.embed( |e| {
        let u = member.user;
        e.author( |a| {
          a.icon_url(get_avatar(&u));
          a.name(format!("{}#{:0>4}", u.name, u.discriminator));
          a.url(format!("https://discord.com/users/{}", u.id))
        });
        e.colour(Colour::from_rgb(139, 195, 74));
        e.description(format!("Welcome <@{}> to the server!", u.id))
      })
    }).await.expect("Failed to send welcome");
  }

  async fn guild_member_removal(&self, ctx: Context, _: GuildId, u: User, _: Option<Member>) {
    config::CHANNEL_ID.send_message(&ctx.http, |m| {
      m.embed( |e| {
        e.author( |a| {
          a.icon_url(get_avatar(&u));
          a.name(format!("{}#{:0>4}", u.name, u.discriminator));
          a.url(format!("https://discord.com/users/{}", u.id))
        });
        e.colour(Colour::from_rgb(244, 67, 54));
        e.description(format!("<@{}> has left the server!", u.id))
      })
    }).await.expect("Failed to send welcome");
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
    /*for c in ApplicationCommand::get_global_application_commands(&ctx.http)
      .await
      .unwrap()
    {
      println!("Deleting {}", c.name);
      ApplicationCommand::delete_global_application_command(&ctx.http, c.id)
        .await
        .unwrap()
    }
    let guild = GuildId((*config::GUILD_ID).to_owned().parse().unwrap());
    for c in guild.get_application_commands(&ctx.http).await.unwrap() {
      println!("Deleting {}", c.name);
      guild.delete_application_command(&ctx.http, c.id).await.unwrap();
    }

    guild
      .create_application_command(&ctx.http, |command| {
        command.name("spawnroles").description("Send roles")
      })
      .await
      .unwrap();
      */
  }
}

fn get_avatar(u: &User) -> String {
  if let Some(avatar) = u.avatar_url() {
    return avatar;
  }
  u.default_avatar_url()
}