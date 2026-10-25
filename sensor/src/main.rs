use aq_weather_station_db::{AirQualityReading, WeatherReading, db};
use bme280::i2c::BME280;
use pmsa003i::{Measurement as AqiMeasure, Pmsa003i};
use rpi_pal::hal::Delay;
use rpi_pal::i2c::I2c;
use sht32d::{I2cAddress, Measurement as TempHumidMeasure, Sht32d};
use std::error::Error;
use toasty::create;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let temp_measure;
    let aqi_measure;
    let pressure;
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
    {
        let i2c = I2c::with_bus(1)?;
        let mut bmp280 = BME280::new_primary(i2c);
        let mut delay = Delay::new();
        bmp280.init(&mut delay).map_err(|e| e.into())?;
        pressure = bmp280.measure(&mut delay).map_err(|e| e.into())?.pressure;
    }

    let database = db()?;

    

    Ok(())
}
