/*
//  Copyright 2021 catgirl.moe contributors <https://catgirl.moe>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use diesel::{pg::Pg, result::Error, BoxableExpression, Table};
use serde::Deserialize;

pub mod comments;
pub mod memes;
pub mod tags;
pub mod users;
pub mod votes;

#[derive(Deserialize, Debug)]
pub struct ApiQuery {
  #[serde(default = "def_lim")]
  pub limit: i64,
  #[serde(default)]
  pub page: i64,
  #[serde(default)]
  pub offset: i64,
  #[serde(default)]
  pub sort: String,
  #[serde(default)]
  pub asc: bool,
}

fn def_lim() -> i64 {
  25
}

#[macro_export]
macro_rules! paginate {
  ($table:expr, $opts:expr, $lim:expr) => {
    $table.limit(std::cmp::min($opts.limit, $lim)).offset($opts.offset + $opts.page * $opts.limit)
  };
}

#[macro_export]
macro_rules! sort {
  ($table:ident, $aq:expr) => {
    $table::table.order($table::table::qsort($aq).unwrap())
  };
}

pub trait Sortable: Table {
  fn sort(c: &str, asc: bool) -> Result<Box<dyn BoxableExpression<Self, Pg, SqlType = ()>>, Error>;

  fn qsort(q: &ApiQuery) -> Result<Box<dyn BoxableExpression<Self, Pg, SqlType = ()>>, Error> {
    Self::sort(q.sort.as_str(), q.asc)
  }
}
//TODO: A bit more refining once I fully learn how macros work
#[macro_export]
macro_rules! sortable {
  ($($table:ident)::* -> $column_type:ty, ($fallback_name:expr => $($fallback_col:ident)::*, $($column_name:expr => $($column:ident)::*),*)) => {
    impl $crate::models::Sortable for $($table)::* {
      fn sort(column: &str, asc: bool) -> Result<Box<dyn $crate::diesel::expression::BoxableExpression<Self, $crate::diesel::pg::Pg, SqlType = ()>>, diesel::result::Error> {
        match column {
          $($column_name => if asc {
            Ok(Box::new(diesel::ExpressionMethods::asc($($column)::*)))
          } else {
            Ok(Box::new(diesel::ExpressionMethods::desc($($column)::*)))
          }),*,
          $fallback_name | _ => if asc {
            Ok(Box::new(diesel::ExpressionMethods::asc($($fallback_col)::*)))
          } else {
            Ok(Box::new(diesel::ExpressionMethods::desc($($fallback_col)::*)))
          }
        }
      }
    }
  };
}
