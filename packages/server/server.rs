extern crate env_logger;
use std::{env};
use dotenv::dotenv;
use actix_web::{middleware, http, web::Data, App,  HttpServer};
use actix_session::{CookieContentSecurity, storage::CookieSessionStore, SessionMiddleware};
use lib::{config, routes};
use actix_web::cookie::Key;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	env::set_var("RUST_LOG", "actix_web=trace");
	env_logger::init();

	let secret_key = Key::generate();
	// let secret_key = env::var("COOKIE_KEY").unwrap()
		
    HttpServer::new(move || {
		App::new()
			.wrap(
				SessionMiddleware::builder(
					CookieSessionStore::default(),
					secret_key.clone()
				)
					.cookie_secure(false)
					.cookie_http_only(true)
					.cookie_content_security(CookieContentSecurity::Private)
					.build()		
			)
			.wrap(
				Cors::default()
					.allowed_origin("http://localhost:3000")
					// .allow_any_origin()
					.allowed_origin("http://localhost:4000/graphiql")
					.allowed_methods(vec!["GET", "OPTIONS", "POST"])
					.allowed_headers(vec![
						http::header::CONTENT_TYPE,
						http::header::SET_COOKIE,
						http::header::COOKIE,
						http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
						http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
					])
					.supports_credentials()

			)
			.wrap(middleware::Compress::default())
			.wrap(middleware::Logger::default())
			.app_data(Data::new(config::graphql::create_schema()))
			.app_data(Data::new(config::postgres::establish_postgres_connection().unwrap().clone()))
			.service(routes::graphql::graphql)
            .service(routes::graphiql::graphiql)
            .service(routes::login::login)
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
