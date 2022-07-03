// use actix_session::Session;
use actix_web::{route,web::{Data, Json}, HttpResponse, Responder};
use juniper::http::GraphQLRequest;

use crate::models::Context;
use crate::config;

#[route("/api/graphql", method = "GET", method = "POST")]
pub async fn graphql(
	graphql: Data<config::graphql::Schema>,
	req: Json::<GraphQLRequest>,
	db: Data<config::postgres::PgPool>,
	// session: Session
) -> impl Responder {
	let context = Data::new(Context {
		db: db.clone(),
	});
    let res = req.execute(&graphql, &context).await;
    HttpResponse::Ok().json(res)
}
 