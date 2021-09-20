/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::config;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use lazy_static::lazy_static;

lazy_static! {
  static ref POOL: Pool<ConnectionManager<PgConnection>> =
    Pool::new(ConnectionManager::new((*config::DATABASE_URL).to_owned()))
      .expect("Failed to initialize database pool");
}

pub fn conn() -> PooledConnection<ConnectionManager<PgConnection>> {
  POOL.get().expect("Failed to retrieve database connection")
}
