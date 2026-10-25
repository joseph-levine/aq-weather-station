mod reading;

use crate::reading::{AirQualityWebReading, WebWeatherReading};
use actix_web::error::ErrorInternalServerError;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    web::{Data, Json, resource},
};
use aq_weather_station_db::{db, list_aq_readings, list_weather_readings};
use std::io::{Error as IoError, ErrorKind, Result as IoResult};
use std::ops::Sub;
use turso::Database;

#[get("/weather-readings")]
async fn weather_readings(pool: Data<Database>) -> actix_web::Result<impl Responder> {
    let conn = pool.connect().map_err(ErrorInternalServerError)?;
    let recent = list_weather_readings(&conn).await.map_err(ErrorInternalServerError)?;
    let web_readings: Vec<WebWeatherReading> = recent.into_iter().map(Into::into).collect();

    Ok(Json(web_readings))
}

#[get("/air-quality-readings")]
async fn air_quality_readings(pool: Data<Database>) -> actix_web::Result<impl Responder> {
    let conn = pool.connect().map_err(ErrorInternalServerError)?;
    let recent = list_aq_readings(&conn).await.map_err(ErrorInternalServerError)?;
    let web_readings: Vec<AirQualityWebReading> = recent
        .into_iter()
        .map(TryInto::try_into)
        .flatten()
        .collect();

    Ok(Json(web_readings))
}

async fn home() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../../web-frontend/dist/index.html"))
}

async fn css() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(include_str!("../../web-frontend/dist/app.css"))
}

async fn js() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(include_str!("../../web-frontend/dist/app.js"))
}

#[actix_web::main]
async fn main() -> IoResult<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = match db().await {
        Ok(p) => p,
        Err(e) => {
            return Err(IoError::new(ErrorKind::Other, e));
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(air_quality_readings)
            .service(weather_readings)
            .service(resource("/").get(home))
            .service(resource("app.css").get(css))
            .service(resource("app.js").get(js))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
