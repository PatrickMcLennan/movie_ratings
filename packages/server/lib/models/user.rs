use diesel::{Queryable};
use juniper::GraphQLObject;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use crate::config::postgres::PgPool;

#[derive(GraphQLObject, Queryable)]
#[graphql(description = "Information about a user")]
pub struct User {
	#[graphql(description = "Account Creation Time")]
	pub created_at: DateTime<Utc>,
	#[graphql(description = "Users email")]
	pub email: String,
	#[graphql(description = "Users first name")]
	pub first_name: String,
	#[graphql(description = "Users id")]
	pub id: i32,
	#[graphql(description = "Users last name")]
	pub last_name: String,
	#[graphql(description = "Account last updated time")]
	pub updated_at: DateTime<Utc>,
}

impl User {
	pub async fn get_all(connection: &PgPool) -> QueryResult<Vec<User>> {
		use crate::models::diesel_schema::users;
		let conn = connection.get().unwrap();
		users::table.load::<User>(&conn)
	}

	pub async fn get_by_id(id: i32, connection: &PgPool) -> QueryResult<User> {
		use crate::models::diesel_schema::users;
		let conn = connection.get().unwrap();
		users::table.find(id).first(&conn)
	}
}
