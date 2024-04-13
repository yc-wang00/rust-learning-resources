use actix_web::{get, App, HttpServer, Responder};

#[get("/hello")]
async fn get_index() -> impl Responder {
    format!("Hello name!")
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}