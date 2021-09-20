/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::{
  config::{self, RoleMenu},
  error::BotError,
};
use serenity::{
  client::Context,
  model::{
    id::RoleId,
    interactions::{
      application_command::ApplicationCommandInteraction as AppComInter,
      message_component::MessageComponentInteraction as MsgComInter, InteractionResponseType::*,
    },
  },
};

pub async fn application_command(ctx: Context, ap: AppComInter) -> Result<(), BotError> {
  match ap.data.name.as_str() {
    "spawnroles" => {
      if ap.user.id != 638230362711130132 {
        ap.create_interaction_response(&ctx.http, |r| {
          r.kind(ChannelMessageWithSource)
            .interaction_response_data(|d| d.content("Unauthorized"))
        })
        .await?;
        return Err(BotError::Unauthorized);
      }
      for menu in config::JSON_CONFIG.role_menus.clone() {
        ap.channel_id
          .send_message(&ctx.http, |m| {
            m.content(&menu.text)
              .components(|c| c.create_action_row(|row| row.add_select_menu(menu.into())))
          })
          .await?;
      }
      ap.create_interaction_response(&ctx.http, |r| r.kind(DeferredUpdateMessage))
        .await?;
    }
    _ => {}
  };
  Ok(())
}

pub async fn message_component(ctx: Context, mc: MsgComInter) -> Result<(), BotError> {
  let optmenu: Option<RoleMenu> = config::JSON_CONFIG
    .role_menus
    .clone()
    .into_iter()
    .find(|i| i.id == mc.data.custom_id.as_str());
  if let Some(menu) = optmenu {
    let mut member = mc.member.clone().unwrap();
    let values = &mc.data.values;
    member
      .remove_roles(
        &ctx.http,
        &menu
          .roles
          .into_iter()
          .map(|v| v.id)
          .collect::<Vec<RoleId>>(),
      )
      .await?;
    member
      .add_roles(
        &ctx.http,
        &values
          .into_iter()
          .map(|v| RoleId(v.parse().unwrap()))
          .collect::<Vec<RoleId>>(),
      )
      .await?;
    mc.create_interaction_response(&ctx.http, |res| res.kind(DeferredUpdateMessage))
      .await?;
  }
  Ok(())
}
