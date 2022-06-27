use crate::config::{postgres};
use std::{sync::{Arc}};

#[derive(Clone)]
pub struct Context {
	pub db: Arc<postgres::PgPool>,
}

impl juniper::Context for Context {}
