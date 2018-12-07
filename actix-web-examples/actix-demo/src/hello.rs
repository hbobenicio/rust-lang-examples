use actix_web::{HttpRequest, Responder};

pub fn index(_request: &HttpRequest) -> impl Responder {
    "Hello, actix-web!"
}
