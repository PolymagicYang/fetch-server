use actix_web::{HttpServer, App, web, Responder, HttpResponse, get, post, error::InternalError};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use server::models::*;
use server::database::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
