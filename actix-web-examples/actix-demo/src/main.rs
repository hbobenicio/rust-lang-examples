#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

extern crate actix_web;

mod hello;
mod auth;

fn create_actix_app() -> actix_web::App {
    actix_web::App::new()
        .middleware(actix_web::middleware::Logger::default())
        // .middleware(actix_web::middleware::Logger::new("%a %{User-Agent}i"))
        .resource("/", |r| {
            r.f(hello::index)
        })
        .resource("/auth/login", |r| {
            r.method(actix_web::http::Method::POST).with(auth::login::basic_login)
        })
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let address = "localhost:8080";
    let server = actix_web::server::new(create_actix_app);
    let server = match server.bind(address) {
        Ok(s) => s,
        Err(err) => {
            error!("actix-demo error: could not listen to '{}': {}", address, err);
            std::process::exit(1);
        }
    };
    
    info!("starting server...");
    server.run();
}
