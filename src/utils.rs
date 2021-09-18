/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use serenity::{
  client::Context,
  model::{
    channel::{Reaction, ReactionType},
    id::ChannelId,
  },
};

pub async fn give_role(ROLES: &[u64], REACTS: &[&str], ctx: Context, r: Reaction) {
  let g = r.guild_id.expect("Failed to get guild");
  let u = r.user_id.expect("Failed to get user");
  let role = ROLES[REACTS
    .iter()
    .position(|&rts| rts == r.emoji.to_string())
    .unwrap()];
  println!("Giving {} role {}", &u, role);
  ctx
    .cache
    .member(&g, &u)
    .await
    .expect("Failed to get member to add role")
    .add_role(&ctx, role)
    .await
    .expect("Failed to add role");
}

pub async fn take_role(ROLES: &[u64], REACTS: &[&str], ctx: Context, r: Reaction) {
  let g = r.guild_id.expect("Failed to get guild");
  let u = r.user_id.expect("Failed to get user");
  let role = ROLES[REACTS
    .iter()
    .position(|&rts| rts == r.emoji.to_string())
    .unwrap()];
  println!("Ungiving {} role {}", &u, role);
  ctx
    .cache
    .member(&g, &u)
    .await
    .expect("Failed to get member to remove role")
    .remove_role(&ctx, role)
    .await
    .expect("Failed to remove role");
}

pub async fn role_message(ROLES: &[u64], REACTS: &[&str], ctx: Context, r: Reaction) {
  let crm = ChannelId(814810596038017034)
    .send_message(&ctx, |m| {
      m.embed(|e| {
        e.title("Pick a country role:");
        e.description(
          ROLES
            .into_iter()
            .map(|r| format!("<@&{}>", r))
            .collect::<Vec<String>>()
            .join("\n"),
        )
      })
    })
    .await
    .expect("Failed to send welcome");
  for r in REACTS {
    crm
      .react(&ctx, ReactionType::Unicode(r.to_owned().to_owned()))
      .await
      .expect("Failed");
  }
}
