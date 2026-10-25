use serde::Serialize;
use thiserror::Error;
use jiff::Timestamp;
use aq_weather_station_db::{AirQualityReading, WeatherReading};

#[derive(Error, Debug)]
#[error("AQI is not calculable")]
pub struct ImpossibleAqi;

#[derive(Error, Debug)]
pub enum ReadingToWebError {
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
pub struct AirQualityWebReading {
    pub timestamp: Timestamp,
    pub pm1_0: i64,
    pub pm2_5: i64,
    pub pm10: i64,
    pub aqi: u32,
    pub aqi_name: String,
    pub aqi_color: String,
}

#[derive(Serialize, Debug)]
pub struct WebWeatherReading {
    pub timestamp: Timestamp,
    pub temperature_c: f64,
    pub temperature_f: f64,
    pub humidity_pct: f64,
    pub pressure_pascal: f64,
    pub pressure_in_hg: f64,
}

struct CalculatedAqi {
    value: u32,
    name: String,
    color: String,
}

impl TryInto<AirQualityWebReading> for AirQualityReading {
    type Error = ReadingToWebError;

    fn try_into(self) -> Result<AirQualityWebReading, Self::Error> {
        let CalculatedAqi { value, name, color } = aqi(self.pm2_5_standard_ug_m3 as f64)?;
        Ok(AirQualityWebReading {
            timestamp: self.created_at,
            // temperature_c: self.temperature_c,
            // temperature_f: self.temperature_c * 1.8 + 32.,
            // humidity_pct: self.humidity_pct,
            pm1_0: self.pm1_0_env_ug_m3,
            pm2_5: self.pm2_5_env_ug_m3,
            pm10: self.pm10_standard_ug_m3,
            aqi: value,
            aqi_name: name,
            aqi_color: color,
        })
    }
}

impl Into<WebWeatherReading> for WeatherReading {
    fn into(self) -> WebWeatherReading {
       WebWeatherReading {
           timestamp: self.created_at,
           temperature_c: self.temperature_c,
           temperature_f: self.temperature_c * 1.8f64 + 32f64,
           humidity_pct: self.humidity_pct,
           pressure_pascal: self.pressure_pascal,
           pressure_in_hg: self.pressure_pascal / 3386.39f64,
       }
    }
}

fn aqi(concentration: f64) -> Result<CalculatedAqi, ImpossibleAqi> {
    if concentration < 0.0 {
        return Err(ImpossibleAqi);
    }
    let breakpoint = BREAKPOINTS
        .iter()
        .filter(|bp| concentration < bp.high && concentration > bp.low)
        .next()
        .unwrap_or_else(|| &BREAKPOINTS.last().expect("static value")); // if it's over, just continue to use the highest breakpoint

    Ok(CalculatedAqi {
        value: ((breakpoint.aqi_high as f64 - breakpoint.aqi_low as f64)
            / (breakpoint.high - breakpoint.low)
            * (concentration - breakpoint.low)
            + breakpoint.aqi_low as f64)
            .round() as u32,
        name: breakpoint.name.to_string(),
        color: breakpoint.color.to_string(),
    })
}
