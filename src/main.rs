use actix_web::{web, App, HttpRequest, HttpServer, Responder, HttpResponse};

async fn health_checkpoint() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_checkpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
