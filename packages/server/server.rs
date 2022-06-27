extern crate env_logger;
use std::{sync::{Arc}};
use dotenv::dotenv;
use lib::{models, config, routes};
use actix_web::{
    get, middleware, web::{Data},
    App, HttpResponse, 
	HttpServer, Responder,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	std::env::set_var("RUST_LOG", "actix_web=debug");
	env_logger::init();
	let graphql = Arc::new(config::graphql::create_schema());
	let db = Arc::new(config::postgres::establish_postgres_connection().unwrap());

    HttpServer::new(move || {
        App::new()
			.app_data(Data::new(models::Context {
				db: db.clone(),
			}))
			.app_data(Data::new(graphql.clone()))
			.wrap(middleware::Compress::default())
			.wrap(middleware::Logger::default())
			.service(routes::graphql::graphql)
            .service(routes::graphiql::graphiql)
            .service(hello)
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
