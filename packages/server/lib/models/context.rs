use actix_web::{web::Data};
use crate::config::postgres;

#[derive(Clone)]
pub struct Context {
	pub db: Data<postgres::PgPool>,
}
