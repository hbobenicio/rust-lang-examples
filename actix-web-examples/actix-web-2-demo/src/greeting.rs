use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct SubjectPayload {
    pub name: String,
}

#[derive(Serialize)]
pub struct Greeting {
    pub message: String,
}

pub async fn handler(subject: web::Json<SubjectPayload>) -> impl Responder {
    let message = format!("Hello, {}!", subject.name);

    let greeting = Greeting { message };
    HttpResponse::Ok().json(greeting)
}
