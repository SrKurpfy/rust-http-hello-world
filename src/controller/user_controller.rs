use actix_web::{HttpRequest, Responder};

pub async fn get(_ : HttpRequest) -> impl Responder {
    return std::thread::current().name().unwrap_or("idk").to_string();
}