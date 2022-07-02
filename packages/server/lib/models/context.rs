use crate::config::{postgres};
use std::{sync::{Arc}};

#[derive(Clone)]
pub struct Context {
	pub db: Arc<postgres::PgPool>,
	pub redis: Arc<redis::Connection>
}

impl juniper::Context for Context {}

impl Context {

	pub fn get_db(&self) -> Option<postgres::PgPool> {
		match Arc::try_unwrap(self.db.clone()) {
			Ok(d) => return Some(d),
			Err(_e) => {
				println!("error getting db");
				return None
			}
		}
	}

	pub fn get_redis(&self) -> Option<redis::Connection> {
		match Arc::try_unwrap(self.redis.clone()) {
			Ok(d) => Some(d),
			Err(_e) => {
				println!("error getting db");
				None
			}
		}
	}
}
