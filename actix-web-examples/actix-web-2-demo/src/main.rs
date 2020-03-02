use actix_web::{web, App, HttpServer};

mod config;
mod hello;
mod greeting;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let config = match config::Config::from_env() {
        Ok(cfg) => cfg,
        Err(config::ConfigError::MissingRequiredVariable{ missing_variables }) => {
            println!("error: required environment variables are missing: {:?}", missing_variables);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "missing required variables"));
        }
    };

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .route("/api/hello/{subject}", web::get().to(hello::handler))
            .route("/api/greeting", web::post().to(greeting::handler))
    })
    .bind(&config.server_addr)?
    .run()
    .await
}
