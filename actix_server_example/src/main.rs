use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;

#[get("/api")]
async fn api_call() -> impl Responder {
    HttpResponse::Ok().body("Hello from API")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(api_call)
            .service(fs::Files::new("/", "www"))
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
