use log::error;
use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::{PgConnection, Queryable};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
pub struct SubjectPayload {
    pub code: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct GreetingEntity {
    pub id: i32,
    pub code: String,
    pub message: String,
}

type DbBlockingError = actix_web::error::BlockingError<diesel::result::Error>;
type DbBlockingResult = Result<Vec<GreetingEntity>, DbBlockingError>;

pub async fn handler(subject: web::Json<SubjectPayload>, db_pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::greetings::dsl::*;

    // execute blocking sync code in threadpool
    let res: DbBlockingResult = web::block(move || {
        let conn = db_pool.get().unwrap();

        let gs: Vec<GreetingEntity> = greetings
            .filter(code.eq(&subject.code))
            .load::<GreetingEntity>(&conn)?;

        Ok(gs)
    }).await;

    match res {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
