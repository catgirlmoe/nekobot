/*
//  Copyright 2021 catgirl.moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::models::memes::memes;
use serde::Serialize;

table! {
  comments (id) {
    id -> Integer,
    content -> VarChar,
    user_id -> BigInt,
    timestamp -> BigInt,
    parent_id -> Nullable<Integer>,
  }
}

table! {
  meme_comments (meme_id, comment_id) {
    meme_id -> Integer,
    comment_id -> Integer,
  }
}

joinable!(meme_comments -> memes (meme_id));
joinable!(meme_comments -> comments (comment_id));

#[derive(Queryable, Serialize)]
pub struct Comment {
  pub id: i32,
  pub content: String,
  pub user_id: i64,
  pub timestamp: i64,
  pub parent_id: Option<i32>,
}
