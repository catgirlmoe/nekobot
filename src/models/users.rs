/*
//  Copyright 2021 catgirl.moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::{db::conn, error::WebError};
use actix_session::Session;
use diesel::{insert_into, result::Error, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

table! {
  users (id) {
    id -> BigInt,
    username -> VarChar,
    discriminator -> SmallInt,
    avatar -> Nullable<VarChar>,
    permissions -> Integer,
    points -> Integer,
  }
}

#[serde_as]
#[derive(Queryable, Serialize)]
pub struct User {
  #[serde_as(as = "DisplayFromStr")]
  pub id: i64,
  pub username: String,
  pub discriminator: i16,
  pub avatar: Option<String>,
  pub permissions: i32,
  pub points: i32,
}

impl User {
  pub fn from_id<'a>(id: i64) -> Result<Self, Error> {
    users::table.find(id).first::<Self>(&conn())
  }

  // TODO: this should probably be moved into utils or smth
  pub fn from_session<'a>(session: Session) -> Result<Option<Self>, WebError> {
    let uid = session.get::<i64>("uid")?;
    if uid.is_none() {
      return Ok(None);
    }
    Ok(Some(users::table.find(uid.unwrap()).first::<Self>(&conn())?))
  }
}

#[serde_as]
#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "users"]
pub struct UpdateUser {
  #[serde_as(as = "DisplayFromStr")]
  pub id: i64,
  pub username: String,
  #[serde_as(as = "DisplayFromStr")]
  pub discriminator: i16,
  pub avatar: Option<String>,
}

impl UpdateUser {
  pub fn update(&self) -> Result<usize, Error> {
    insert_into(users::table)
      .values(self)
      .on_conflict(users::id)
      .do_update()
      .set(self)
      .execute(&conn())
  }
}
