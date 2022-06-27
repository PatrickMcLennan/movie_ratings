use actix_web::{route,web::{Data, Json}, HttpResponse, Responder};
use std::{sync::{Arc}};
use juniper::http::GraphQLRequest;

use crate::models::Context;
use crate::config::graphql::Schema;

#[route("/graphql", method = "GET", method = "POST")]
pub async fn graphql(
	context:  Data<Context>,
	graphql: Data<Arc<Schema>>,
	req: Json::<GraphQLRequest>
) -> impl Responder {
    let res = req.execute(&graphql, &context).await;
    HttpResponse::Ok().json(res)
}
 