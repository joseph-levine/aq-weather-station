mod reading;

use crate::reading::{Reading, WebReading};
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    web::{Data, Json, resource},
};
use log::error;
use sqlx::{
    Error as SqlxError, Pool, Sqlite, SqlitePool, migrate, migrate::MigrateDatabase, query_as,
};
use std::env::home_dir;
use std::io::{Error as IoError, ErrorKind, Result as IoResult};

#[get("/readings")]
async fn readings(pool: Data<SqlitePool>) -> actix_web::Result<impl Responder> {
    let res = query_as!(
        Reading,
        "select * from reading order by epoch_timestamp desc limit 100"
    )
    .fetch_all(pool.get_ref())
    .await;
    let Ok(readings) = res else {
        error!("Error fetching readings");
        return Err(actix_web::error::ErrorInternalServerError(
            "Error fetching readings",
        ));
    };
    let web_readings: Vec<WebReading> = readings
        .into_iter()
        .map(TryInto::try_into)
        .flatten()
        .collect();

    Ok(Json(web_readings))
}

async fn home() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(include_str!("../../web-frontend/dist/index.html"))
}

async fn css() -> impl Responder {
    HttpResponse::Ok().content_type("text/css").body(include_str!("../../web-frontend/dist/app.css"))
}

async fn js() -> impl Responder {
    HttpResponse::Ok().content_type("text/javascript").body(include_str!("../../web-frontend/dist/app.js"))
}


async fn init_db(db_path: impl AsRef<str>) -> Result<Pool<Sqlite>, SqlxError> {
    let connection_string = format!("sqlite://{}", db_path.as_ref());
    if !Sqlite::database_exists(&connection_string).await? {
        Sqlite::create_database(&connection_string).await?;
    }
    SqlitePool::connect(&connection_string).await
}

#[actix_web::main]
async fn main() -> IoResult<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let db_path = home_dir().map(|h| h.join(".local/share/aq/db.sqlite"));
    let Some(db_path) = db_path else {
        return Err(IoError::new(ErrorKind::NotFound, "home doesn't exist"));
    };
    let Ok(pool) = init_db(db_path.to_str().expect("infallible")).await else {
        return Err(IoError::new(ErrorKind::NotFound, "db not created"));
    };

    if let Err(e) = migrate!("./db/migrations").run(&pool).await {
        error!("{}", e);
        return Err(IoError::new(ErrorKind::InvalidData, "db migrate failed"));
    }

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(readings)
            .service(resource("/").get(home))
            .service(resource("app.css").get(css))
            .service(resource("app.js").get(js))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
