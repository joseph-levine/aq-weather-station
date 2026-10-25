use byteorder::{BigEndian, ReadBytesExt};
use embedded_hal::{i2c, i2c::I2c};
use std::io::Cursor;
use thiserror::Error;

#[derive(Debug)]
pub struct Measurement {
    pub pm10_standard: u16,
    pub pm25_standard: u16,
    pub pm100_standard: u16,
    pub pm10_env: u16,
    pub pm25_env: u16,
    pub pm100_env: u16,
    pub particles_03um: u16,
    pub particles_05um: u16,
    pub particles_10um: u16,
    pub particles_25um: u16,
    pub particles_50um: u16,
    pub particles_100um: u16,
}

#[derive(Error, Debug)]
pub enum Pmsa003iError<E>
where
    E: i2c::Error,
{
    #[error(transparent)]
    I2cError(#[from] E),
    #[error("Header error")]
    HeaderError,
    #[error("Checksum error")]
    ChecksumError,
    #[error("unaligned array")]
    Unaligned,
}

pub struct Pmsa003i<I>
where
    I: I2c,
{
    i2c: I,
}

const ADDRESS: u8 = 0x12;

impl<I> Pmsa003i<I>
where
    I: I2c,
{
    pub fn new(i2c: I) -> Pmsa003i<I> {
        Pmsa003i { i2c }
    }
    pub fn measure(&mut self) -> Result<Measurement, Pmsa003iError<I::Error>> {
        let mut buffer = [0; 32];
        self.i2c.read(ADDRESS, &mut buffer)?;
        let mut rdr = Cursor::new(buffer);
        let header = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let frame_length = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let pm10_standard = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let pm25_standard = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let pm100_standard = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let pm10_env = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let pm25_env = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let pm100_env = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let particles_03um = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let particles_05um = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let particles_10um = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let particles_25um = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let particles_50um = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let particles_100um = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        let _version = rdr.read_u8().map_err(|_| Pmsa003iError::Unaligned)?;
        let _error_code = rdr.read_u8().map_err(|_| Pmsa003iError::Unaligned)?;
        let given_checksum = rdr
            .read_u16::<BigEndian>()
            .map_err(|_| Pmsa003iError::Unaligned)?;
        if header != 0x424d {
            return Err(Pmsa003iError::HeaderError);
        }
        if frame_length != 28 {
            return Err(Pmsa003iError::HeaderError);
        }
        let calculated_checksum: u16 = buffer[0..30].iter().copied().map(u16::from).sum();
        if calculated_checksum != given_checksum {
            return Err(Pmsa003iError::ChecksumError);
        }

        Ok(Measurement {
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
        })
    }
}
