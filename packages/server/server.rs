extern crate env_logger;
use std::{env};
use dotenv::dotenv;
use actix_web::{middleware, web::{Data}, App,  HttpServer};
use actix_session::{CookieContentSecurity, storage::{CookieSessionStore}, SessionMiddleware};
use lib::{config, routes};
use actix_web::cookie::Key;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	env::set_var("RUST_LOG", "actix_web=trace");
	env_logger::init();
	// let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

	let secret_key = Key::generate();
		
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
			.app_data(Data::new(config::graphql::create_schema()))
			.app_data(Data::new(config::postgres::establish_postgres_connection().unwrap().clone()))
			.wrap(middleware::Compress::default())
			.wrap(middleware::Logger::default())
			.service(routes::graphql::graphql)
            .service(routes::graphiql::graphiql)
            .service(routes::login::login)
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
