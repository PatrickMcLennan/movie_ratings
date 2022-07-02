use diesel::{Queryable};
use actix_web::{web::{Json}};
use serde::{Deserialize, Serialize};
use juniper::GraphQLObject;
use regex::Regex;

#[derive(Debug, Deserialize, GraphQLObject, Queryable, Serialize)]
#[graphql(description = "Information about a user")]
pub struct LoginDTO {
	#[graphql(description = "Users email")]
	pub email: String,
	#[graphql(description = "Users password")]
	pub password: String,
}

impl LoginDTO {

	pub fn from_json(json: Json<LoginDTO>) -> Self {
		LoginDTO { email: json.email.to_string(), password: json.password.to_string() }
	}

	pub fn validate(&self) -> bool {
		fn is_password_valid(s: &str) -> bool {
			let mut has_whitespace = false;
			let mut has_upper = false;
			let mut has_lower = false;
			let mut has_digit = false;
		
			for c in s.chars() {
				has_whitespace |= c.is_whitespace();
				has_lower |= c.is_lowercase();
				has_upper |= c.is_uppercase();
				has_digit |= c.is_digit(10);
			}
		
			!has_whitespace && has_upper && has_lower && has_digit && s.len() >= 8
		}
		let email_regex = Regex::new(r"[a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?").unwrap();
		let valid_email = email_regex.is_match(&self.email);
		let valid_password = is_password_valid(&self.password);
		valid_password && valid_email
	}
	
}
