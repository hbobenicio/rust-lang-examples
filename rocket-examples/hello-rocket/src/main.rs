#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::Request;
use rocket_contrib::Json;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    code: u16,
    message: String
}

#[catch(404)]
fn not_found(_request: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        code: 404,
        message: String::from("Not Found")
    })
}

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

#[derive(Debug, Deserialize)]
struct UserCredentials {
    username: String,
    password: String
}

#[post("/auth/login", format = "application/json", data = "<credentials>")]
fn login(credentials: Json<UserCredentials>) -> &'static str {
    println!("Username: {}", credentials.username);
    println!("Password: {}", credentials.password);

    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .catch(catchers![not_found])
        .launch();
}
