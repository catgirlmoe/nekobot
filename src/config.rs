/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use lazy_static::lazy_static;
use serenity::{builder::{CreateSelectMenu, CreateSelectMenuOption}, model::{channel::ReactionType, id::{ChannelId, GuildId, RoleId}}};
use std::{env, fs::File, io::Read};

#[rustfmt::skip]
lazy_static! {
  pub static ref DISCORD_TOKEN: String = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN");
  pub static ref APPLICATION_ID: String = env::var("APPLICATION_ID").expect("APPLICATION_ID");
  pub static ref GUILD_ID: GuildId = GuildId(env::var("GUILD_ID").expect("GUILD_ID").parse().expect("GUILD_ID"));
  pub static ref CHANNEL_ID: ChannelId = ChannelId(env::var("CHANNEL_ID").expect("CHANNEL_ID").parse().expect("CHANNEL_ID"));
  //pub static ref DATABASE_URL: String = env::var("DATABASE_URL").expect("DATABASE_URL");

  pub static ref JSON_CONFIG: Config = {
    let mut file = File::open("config.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    serde_json::from_str(&contents).unwrap()
  };
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
  pub role_menus: Vec<RoleMenu>,
}
#[derive(Deserialize, Clone)]
pub struct RoleMenu {
  pub id: String,
  pub text: String,
  pub roles: Vec<RoleButton>,
}

impl Into<CreateSelectMenu> for RoleMenu {
  fn into(self) -> CreateSelectMenu {
    let mut menu = CreateSelectMenu::default();
    menu.custom_id(&self.id);
    menu.min_values(0);
    menu.max_values(self.roles.len() as u64);
    menu.options(|ops| {
      for role in self.roles {
        ops.add_option(role.into());
      }
      ops
    });
    menu
  }
}

#[derive(Deserialize, Clone)]
pub struct RoleButton {
  pub id: RoleId,
  pub emoji: String,
  pub text: String,
  pub desc: Option<String>,
}

impl Into<CreateSelectMenuOption> for RoleButton {
  fn into(self) -> CreateSelectMenuOption {
    let mut opt = CreateSelectMenuOption::default();
    opt.value(self.id);
    opt.emoji(ReactionType::Unicode(self.emoji));
    opt.label(self.text);
    if let Some(desc) = self.desc {
      opt.description(desc);
    }
    opt
  }
}
