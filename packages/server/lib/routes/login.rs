use actix_web::{route,web::{Data, Json}, HttpResponse, Responder};
use actix_session::Session;
use crate::{config, models::{dto::login::LoginDTO, User}};
use std::thread;

type JsonDTO = Json<LoginDTO>;


#[route("/api/login", method = "POST")]
pub async fn login(
	dto: JsonDTO,
	db: Data<config::postgres::PgPool>,
	session: Session,
) -> impl Responder {
	let four_hundred = HttpResponse::BadRequest().body::<String>(String::from("Invalid email or password"));

	let dto_struct = LoginDTO::from_json(dto);
	if !dto_struct.validate() { return four_hundred };

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
			Err(_e) => { return four_hundred }
		};

	match session.insert(&id.to_string(), String::new()) {
		Ok(_) => HttpResponse::Created().body::<String>(String::from("User logged in")),
		Err(e) => {
			println!("{:?}", e);
			HttpResponse::InternalServerError().body::<String>(String::from("Error creating a session"))
		}
	}

}
 