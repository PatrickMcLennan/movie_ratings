// use actix_session::Session;
use actix_web::{route,web::{Data, Json}, HttpResponse, Responder, HttpRequest};
use juniper::http::GraphQLRequest;
use actix_web::cookie::Cookie;

use crate::models::Context;
use crate::config;

#[route("/api/graphql", method = "GET", method = "POST")]
pub async fn graphql(
	graphql: Data<config::graphql::Schema>,
	req: Json::<GraphQLRequest>,
	http_req: HttpRequest,
	db: Data<config::postgres::PgPool>,
) -> impl Responder {
	let cookies = http_req.cookies().unwrap();

	let id_cookie = match cookies
		.iter()
		.find(|c| {
			let cookie_name = c.name();
			println!("cookie_name is: {:?}", cookie_name);
			cookie_name == "id"
		}) {
			Some(c) => c,
			None => return HttpResponse::Unauthorized().body("User must be logged in")
		};

	println!("id_cookie is :{:?}", id_cookie);

	let context = Data::new(Context {
		db: db.clone(),
	});
    let res = req.execute(&graphql, &context).await;
    HttpResponse::Ok().json(res)
}
 