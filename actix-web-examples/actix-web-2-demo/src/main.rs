use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;

#[macro_use]
extern crate diesel;

mod schema;

mod config;
mod hello;
mod greeting;
mod dbgreeting;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

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

    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    let db_pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(actix_web::middleware::Logger::default())
            .route("/api/hello/{subject}", web::get().to(hello::handler))
            .route("/api/greeting", web::post().to(greeting::handler))
            .route("/api/db/greeting", web::post().to(dbgreeting::handler))
    })
    .bind(&config.server_addr)?
    .run()
    .await
}
