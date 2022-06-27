extern crate env_logger;
use std::{sync::{Arc}};
use dotenv::dotenv;
use actix_web::{middleware, web::{Data}, App,  HttpServer};

use lib::{models, config, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	std::env::set_var("RUST_LOG", "actix_web=debug");
	env_logger::init();

	let graphql = Arc::new(config::graphql::create_schema());
	let db = Arc::new(config::postgres::establish_postgres_connection().unwrap());
	let redis = Arc::new(config::redis::connect_to_redis().unwrap());

    HttpServer::new(move || {
        App::new()
			.app_data(Data::new(models::Context {
				db: db.clone(),
				redis: redis.clone()
			}))
			.app_data(Data::new(graphql.clone()))
			.wrap(middleware::Compress::default())
			.wrap(middleware::Logger::default())
			.service(routes::graphql::graphql)
            .service(routes::graphiql::graphiql)
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
