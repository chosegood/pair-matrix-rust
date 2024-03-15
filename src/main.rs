use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
mod index;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index::index_handler))
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
