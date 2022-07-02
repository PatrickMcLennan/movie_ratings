use actix_web::{route,web::{Data, Json}, HttpRequest, HttpResponse, Responder};
use crate::{config, models::{dto::login::LoginDTO, User}};
use std::thread;

type JsonDTO = Json<LoginDTO>;

#[route("/api/login", method = "POST")]
pub async fn login(
	_req: HttpRequest,
	dto: JsonDTO,
	db: Data<config::postgres::PgPool>
) -> impl Responder {
	let dto_struct = LoginDTO::from_json(dto);
	if !dto_struct.validate() { return HttpResponse::BadRequest().body::<String>(String::from("Invalid email or password")) };

	let id = match thread::spawn({
		let db_clone = db.get_ref().clone();
		move || {
			User::authenticate(dto_struct.email, dto_struct.password, db_clone)
		}
	})
		.join()
		.unwrap()
		.await
		.map_err(actix_web::error::ErrorInternalServerError) {
			Ok(v) => v,
			Err(e) => {
				println!("from inside id");
				println!("{:?}", e);
				return HttpResponse::BadRequest().body::<String>(String::from("Invalid email or password"))
			}
		};

	println!("Users id is: {}", id);

    HttpResponse::Ok().body::<String>(String::from("working"))
}
 