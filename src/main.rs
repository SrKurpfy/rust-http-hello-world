use actix_web::{web, App, HttpRequest, HttpServer, Responder};

mod controller;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    return HttpServer::new(|| {
       App::new()
       .route("/", web::get().to(controller::user_controller::get)) 
    }).bind("127.0.0.1:3000")?.run()
    .await;

}