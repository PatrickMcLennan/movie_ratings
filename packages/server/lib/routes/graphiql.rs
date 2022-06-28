use actix_web::{get, Responder};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source};

#[get("/graphiql")]
pub async fn graphiql() -> impl Responder {
    Html(graphiql_source("/api/graphql", None))
}
