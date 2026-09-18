use pmsa003i::{Measurement as AqiMeasure, Pmsa003i};
use rpi_pal::i2c::I2c;
use sht32d::{I2cAddress, Measurement as TempHumidMeasure, Sht32d};
use sqlx::SqlitePool;
use std::env::home_dir;
use std::error::Error;
use std::io::Error as IoError;
use std::io::ErrorKind;
use time::{Timestamp, UtcDateTime};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let temp_measure;
    let aqi_measure;
    {
        let i2c = I2c::with_bus(1)?;
        let mut sht32d = Sht32d::new(i2c, I2cAddress::DefaultAddress);
        temp_measure = sht32d.measure()?;
    }
    {
        let i2c = I2c::with_bus(1)?;
        let mut pmsa003i = Pmsa003i::new(i2c);
        aqi_measure = pmsa003i.measure()?;
    }
    let TempHumidMeasure {
        temperature,
        humidity,
    } = temp_measure;
    let AqiMeasure {
        pm10_standard,
        pm25_standard,
        pm100_standard,
        pm10_env,
        pm25_env,
        pm100_env,
        particles_03um,
        particles_05um,
        particles_10um,
        particles_25um,
        particles_50um,
        particles_100um,
    } = aqi_measure;
    let current_epoch_ts = Timestamp::now().as_seconds();
    let db_path = home_dir()
        .map(|h| h.join(".local/share/aq/db.sqlite"))
        .ok_or(IoError::new(ErrorKind::NotFound, "home doesn't exist"))?;
    let connection_string = format!("sqlite://{}", db_path.to_str().expect("infallible"));
    let pool = SqlitePool::connect(&connection_string).await?;
    sqlx::query!("INSERT INTO reading (epoch_timestamp, temperature_c, humidity_pct, pm10_standard, pm25_standard, pm100_standard, pm10_env, pm25_env, pm100_env, particles_03um, particles_05um, particles_10um, particles_25um, particles_50um, particles_100um) values (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
            current_epoch_ts, temperature, humidity, pm10_standard, pm25_standard, pm100_standard, pm10_env, pm25_env, pm100_env, particles_03um, particles_05um, particles_10um, particles_25um, particles_50um, particles_100um )
        .execute(&pool)
        .await?;
    Ok(())
}
