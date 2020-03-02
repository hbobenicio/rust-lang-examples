use actix_web::{web, HttpResponse, Responder};

pub async fn handler(path_params: web::Path<String>) -> impl Responder {
    let subject = path_params;
    let message = format!("Hello, {}!", subject);

    HttpResponse::Ok().body(message)
}
