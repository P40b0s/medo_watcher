use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod services;
use services::medo_service as medo;
#[tokio::main]
async fn main() -> std::io::Result<()> 
{
    HttpServer::new(|| 
    {
        App::new()
            .service(medo::hello)
            .service(medo::echo)
            .route("/hey", web::get().to(medo::manual_hello))
    })
    .bind(("127.0.0.1", 8181))?
    .run()
    .await
}