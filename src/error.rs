/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use serenity::prelude::SerenityError;
use std::fmt;

#[derive(Debug)]
pub enum BotError {
  Unauthorized,
  ServerError(String, String),
}

impl fmt::Display for BotError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      BotError::Unauthorized => write!(f, "Unauthorized"),
      BotError::ServerError(name, error) => write!(f, "{}: {}", name, error),
    }
  }
}

impl From<SerenityError> for BotError {
  fn from(err: SerenityError) -> BotError {
    BotError::ServerError("SerenityError".into(), err.to_string())
  }
}
