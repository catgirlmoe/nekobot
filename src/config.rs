/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use lazy_static::lazy_static;
use std::env;

lazy_static! {
  pub static ref PORT: String = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN");
}
