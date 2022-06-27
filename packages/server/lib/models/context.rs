use crate::config::{postgres};
use std::{sync::{Arc}};

#[derive(Clone)]
pub struct Context {
	pub db: Arc<postgres::PgPool>,
	pub redis: Arc<redis::Connection>
}

impl juniper::Context for Context {}
