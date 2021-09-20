/*
//  Copyright 2021 catgirl.moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::models::{memes::memes, users::users};

table! {
  meme_votes (meme_id, user_id) {
    meme_id -> Integer,
    user_id -> BigInt,
    vote -> BigInt,
  }
}

joinable!(meme_votes -> memes (meme_id));
joinable!(meme_votes -> users (user_id));

allow_tables_to_appear_in_same_query!(meme_votes, memes, users);
