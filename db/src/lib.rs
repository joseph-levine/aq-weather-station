use jiff::Timestamp;
use std::env::home_dir;
use turso::{Builder, Connection, Database, params};

#[derive(Debug)]
pub struct AirQualityReading {
    pub created_at: Timestamp,
    pub pm1_0_standard_ug_m3: i64,
    pub pm2_5_standard_ug_m3: i64,
    pub pm10_standard_ug_m3: i64,
    pub pm1_0_env_ug_m3: i64,
    pub pm2_5_env_ug_m3: i64,
    pub pm10_env_ug_m3: i64,
    pub particles_0_3um: i64,
    pub particles_0_5um: i64,
    pub particles_1_0um: i64,
    pub particles_2_5um: i64,
    pub particles_5_0um: i64,
    pub particles_10um: i64,
}

#[derive(Debug)]
pub struct WeatherReading {
    pub created_at: Timestamp,
    pub temperature_c: f64,
    pub humidity_pct: f64,
    pub pressure_pascal: f64,
}

pub async fn record_air_quality(
    conn: Connection,
    air_quality_reading: &AirQualityReading,
) -> turso::Result<()> {
    let ts = air_quality_reading
        .created_at
        .to_string();
    let c = conn
        .execute("INSERT INTO air_quality_reading (created_at, pm1_0_standard_ug_m3, pm2_5_standard_ug_m3, pm10_standard_ug_m3, pm1_0_env_ug_m3, pm2_5_env_ug_m3, pm10_env_ug_m3, particles_0_3um, particles_0_5um, particles_1_0um, particles_2_5um, particles_5_0um, particles_10um) VALUES (?1, ?2, ?3, ?4, ?5, 6?, 7?, ?8, ?9, ?10, ?11, ?12, ?13)",
                 params![ts, air_quality_reading.pm1_0_standard_ug_m3, air_quality_reading.pm2_5_standard_ug_m3, air_quality_reading.pm10_standard_ug_m3, air_quality_reading.pm1_0_env_ug_m3,air_quality_reading. pm2_5_env_ug_m3,air_quality_reading. pm10_env_ug_m3,air_quality_reading. particles_0_3um,air_quality_reading. particles_0_5um, air_quality_reading.particles_1_0um, air_quality_reading.particles_2_5um, air_quality_reading.particles_5_0um, air_quality_reading.particles_10um])
        .await?;
    Ok(())
}

pub async fn record_weather(
    conn: &Connection,
    weather_reading: &WeatherReading,
) -> turso::Result<()> {
    let ts = weather_reading
        .created_at
        .to_string();
    let c = conn
        .execute("INSERT INTO weather_reading (created_at, temperature_c, humidity_pct, pressure_pascal) VALUES (?1, ?2, ?3, ?4)",
                 params![ts, weather_reading.temperature_c, weather_reading.humidity_pct, weather_reading.pressure_pascal])
        .await?;
    Ok(())
}

pub async fn list_aq_readings(conn: &Connection) -> turso::Result<Vec<AirQualityReading>> {
    let mut aq_readings = Vec::new();
    let mut rows = conn.query("SELECT created_at, pm1_0_standard_ug_m3, pm2_5_standard_ug_m3, pm10_standard_ug_m3, pm1_0_env_ug_m3, pm2_5_env_ug_m3, pm10_env_ug_m3, particles_0_3um, particles_0_5um, particles_1_0um, particles_2_5um, particles_5_0um, particles_10um FROM air_quality_reading ORDER BY created_at", ()).await?;
    while let Some(row) = rows.next().await? {
        let created_at_str: String = row.get(0)?;
        let ts: Timestamp = created_at_str.parse().expect("Failed to parse timestamp");
        let aqr = AirQualityReading {
            created_at: ts,
            pm1_0_standard_ug_m3: row.get(1)?,
            pm2_5_standard_ug_m3: row.get(2)?,
            pm10_standard_ug_m3: row.get(3)?,
            pm1_0_env_ug_m3: row.get(4)?,
            pm2_5_env_ug_m3: row.get(5)?,
            pm10_env_ug_m3: row.get(6)?,
            particles_0_3um: row.get(7)?,
            particles_0_5um: row.get(8)?,
            particles_1_0um: row.get(9)?,
            particles_2_5um: row.get(10)?,
            particles_5_0um: row.get(11)?,
            particles_10um: row.get(12)?,
        };
        aq_readings.push(aqr);
    }
    Ok(aq_readings)
}

pub async fn list_weather_readings(conn: &Connection) -> turso::Result<Vec<WeatherReading>> {
    let mut weater_readings = Vec::new();
    let mut rows = conn.query("SELECT created_at, temperature_c, humidity_pct, pressure_pascal FROM weather_reading", ()).await?;
    while let Some(row) = rows.next().await? {
        let created_at_str: String = row.get(0)?;
        let ts: Timestamp = created_at_str.parse().expect("Failed to parse timestamp");
        let wr = WeatherReading {
            created_at: ts,
            temperature_c: row.get(1)?,
            humidity_pct: row.get(2)?,
            pressure_pascal: row.get(3)?,
        };
        weater_readings.push(wr);
    }
    Ok(weater_readings)
}

pub async fn db() -> turso::Result<Database> {
    let db_path = home_dir()
        .map(|h| h.join(".local/share/aq/db.db"))
        .expect("Couldn't find the home directory");
    assert!(db_path.parent().expect("No parent directory").exists());
    let b =
        Builder::new_local(db_path.to_str().expect("Infallible"))
            .experimental_multiprocess_wal(true);
    let db = b.build().await?;
    let conn = db.connect()?;
    let _ = conn
        .execute(include_str!("../sql/air_quality_reading.sql"), ())
        .await?;
    let _ = conn
        .execute(include_str!("../sql/weather_reading.sql"), ())
        .await?;
    Ok(db)
}
