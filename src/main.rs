use actix_web::{HttpServer, App, web, Responder, HttpResponse, get, post, middleware::{Logger}};
use server::connect_to_mongo;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let server = match connect_to_mongo().await {
        Ok(s) => s,
        Err(e) => { 
            println!("failed to connect to database: {}", e);
            return Ok(())
        }
    };
    for name in server.list_database_names(None, None).await.unwrap() {
        println!("{}", name);
    }

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(client.clone()))
            .route("/hello", web::get().to(|| async { "Hello World!" }))
    })
    .bind(("127.0.0.1", 8080))?
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
