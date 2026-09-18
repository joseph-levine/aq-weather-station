use serde::Serialize;
use thiserror::Error;
use time::UtcDateTime;

#[derive(Error, Debug)]
#[error("AQI is not calculable")]
pub struct ImpossibleAqi;

#[derive(Error, Debug)]
pub enum ReadingToWebError {
    #[error(transparent)]
    UnixTimestampError(#[from] time::error::ComponentRange),
    #[error(transparent)]
    ImpossibleAqi(#[from] ImpossibleAqi),
}

struct Breakpoint<'a> {
    low: f64,
    high: f64,
    aqi_low: u32,
    aqi_high: u32,
    name: &'a str,
    color: &'a str,
}

const BREAKPOINTS: [Breakpoint; 6] = [
    Breakpoint {
        low: 0.0,
        high: 9.0,
        aqi_low: 0,
        aqi_high: 50,
        name: "Good",
        color: "00E400",
    },
    Breakpoint {
        low: 9.1,
        high: 35.4,
        aqi_low: 51,
        aqi_high: 100,
        name: "Moderate",
        color: "FFFF00",
    },
    Breakpoint {
        low: 35.5,
        high: 55.4,
        aqi_low: 101,
        aqi_high: 150,
        name: "Unhealthy for Sensitive Groups",
        color: "FF7E00",
    },
    Breakpoint {
        low: 55.5,
        high: 125.4,
        aqi_low: 151,
        aqi_high: 200,
        name: "Unhealthy",
        color: "FF0000",
    },
    Breakpoint {
        low: 125.5,
        high: 225.4,
        aqi_low: 201,
        aqi_high: 300,
        name: "Very unhealthy",
        color: "8F3F97",
    },
    Breakpoint {
        low: 225.5,
        high: 325.4,
        aqi_low: 301,
        aqi_high: 400,
        name: "Hazardous",
        color: "7F0023",
    },
];
#[derive(Serialize, Debug)]
pub struct Reading {
    pub epoch_timestamp: i64,
    pub temperature_c: f64,
    pub humidity_pct: f64,
    pub pm10_standard: i64,
    pub pm25_standard: i64,
    pub pm100_standard: i64,
    pub pm10_env: i64,
    pub pm25_env: i64,
    pub pm100_env: i64,
    pub particles_03um: i64,
    pub particles_05um: i64,
    pub particles_10um: i64,
    pub particles_25um: i64,
    pub particles_50um: i64,
    pub particles_100um: i64,
}

#[derive(Serialize, Debug)]
pub struct WebReading {
    pub utc_datetime: UtcDateTime,
    pub temperature_c: f64,
    pub temperature_f: f64,
    pub humidity_pct: f64,
    pub pm_25_ug_cubic_m: f64,
    pub aqi: u32,
    pub aqi_name: String,
    pub aqi_color: String,
}

impl TryInto<WebReading> for Reading {
    type Error = ReadingToWebError;

    fn try_into(self) -> Result<WebReading, Self::Error> {
        let (aqi, aqi_name, aqi_color) = aqi(self.particles_25um as f64)?;
        Ok(WebReading {
            utc_datetime: UtcDateTime::from_unix_timestamp(self.epoch_timestamp)?,
            temperature_c: self.temperature_c,
            temperature_f: self.temperature_c * 1.8 + 32.,
            humidity_pct: self.humidity_pct,
            pm_25_ug_cubic_m: self.particles_25um as f64,
            aqi,
            aqi_name,
            aqi_color,
        })
    }
}

#[inline]
pub fn aqi(concentration: f64) -> Result<(u32, String, String), ImpossibleAqi> {
    if concentration < 0.0 {
        return Err(ImpossibleAqi);
    }
    let breakpoint = BREAKPOINTS
        .iter()
        .filter(|bp| concentration < bp.high && concentration > bp.low)
        .next()
        .unwrap_or_else(|| &BREAKPOINTS.last().expect("static value")); // if it's over, just continue to use the highest breakpoint

    Ok((
        ((breakpoint.aqi_high as f64 - breakpoint.aqi_low as f64)
            / (breakpoint.high - breakpoint.low)
            * (concentration - breakpoint.low)
            + breakpoint.aqi_low as f64)
            .round() as u32,
        breakpoint.name.to_string(),
        breakpoint.color.to_string(),
    ))
}
