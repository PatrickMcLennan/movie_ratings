use std::{sync::Arc};
use actix_web::{
    get, middleware, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

mod schema;
use crate::schema::{create_schema, Schema};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// Create Juniper schema
	let schema = Arc::new(create_schema());
    HttpServer::new(move || {
        App::new()
			.app_data(Data::from(schema.clone()))
			.service(graphql)
            .service(graphql_playground)
            .service(hello)
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
