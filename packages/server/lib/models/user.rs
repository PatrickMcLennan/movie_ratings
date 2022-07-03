use diesel::{Queryable};
use juniper::GraphQLObject;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use crate::config::postgres::PgPool;
use diesel::sql_types::Text;
#[derive(Queryable)]
pub struct User {
	pub created_at: DateTime<Utc>,
	pub email: String,
	pub first_name: String,
	pub id: String,
	pub last_name: String,
	pub password_salt: String,
	pub updated_at: DateTime<Utc>,
}

impl User {

	pub async fn authenticate(
		email_attempt: String, 
		password: String, 
		connection: PgPool
	) -> QueryResult<String> {
		use crate::models::diesel_schema::users::dsl::{id, users};
		use diesel::dsl::sql;
		users
			.select(id)
			.filter(
				sql("email=")
					.bind::<Text, _>(email_attempt)
					.sql(" AND password_salt=crypt(")
					.bind::<Text, _>(password)
					.sql(", password_salt)")
			)
			.first::<String>(&connection.get().unwrap())
	}

	pub async fn get_all(connection: &PgPool) -> QueryResult<Vec<User>> {
		use crate::models::diesel_schema::users::dsl::users;
		users.load::<User>(&connection.get().unwrap())
	}
	
	pub async fn get_by_id(id: String, connection: &PgPool) -> QueryResult<User> {
		use crate::models::diesel_schema::users::dsl::users;
		users.find(id).first(&connection.get().unwrap())
	}
	
}

#[derive(GraphQLObject)]
#[graphql(description = "Public information about a user available")]
pub struct PublicUser {
	#[graphql(description = "Account Creation Time")]
	pub created_at: DateTime<Utc>,
	#[graphql(description = "Users email")]
	pub email: String,
	#[graphql(description = "Users first name")]
	pub first_name: String,
	#[graphql(description = "Users id")]
	pub id: String,
	#[graphql(description = "Users last name")]
	pub last_name: String,
	#[graphql(description = "Account last updated time")]
	pub updated_at: DateTime<Utc>,  
}

impl PublicUser {
	pub fn from_user(user: User) -> Self {
		Self { 
			created_at: user.created_at, 
			email: user.email, 
			first_name: user.first_name, 
			id: user.id, 
			last_name: user.last_name, 
			updated_at: user.updated_at 
		}
	}
}
