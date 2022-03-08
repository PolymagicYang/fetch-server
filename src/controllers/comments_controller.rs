use actix_web::{HttpServer, App, web, Responder, HttpResponse, get, post, middleware::{Logger}, dev::ResourceDef};
use mongodb::Client;

#[get("/fetch/{api}")]
pub async fn fetch_comments(client: web::Data<Client>) -> impl Responder {
	web::Json(vec!["233"])
}
