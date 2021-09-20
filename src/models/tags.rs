/*
//  Copyright 2021 catgirl.moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::models::memes::memes;
use serde::Serialize;

table! {
  tags (id) {
    id -> Integer,
    name -> VarChar,
    explicit -> Bool,
  }
}

table! {
  meme_tags (meme_id, tag_id) {
    meme_id -> Integer,
    tag_id -> Integer,
  }
}

joinable!(meme_tags -> memes (meme_id));
joinable!(meme_tags -> tags (tag_id));

#[derive(Queryable, Serialize)]
pub struct Tag {
  pub id: i32,
  pub name: String,
  pub explicit: bool,
}
