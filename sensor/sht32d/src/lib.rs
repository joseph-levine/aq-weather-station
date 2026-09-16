use std::thread::sleep;
use std::time::Duration;
use crc::{Algorithm, CRC_8_NRSC_5, Crc};
use embedded_hal::i2c::I2c;

const READSERIALNBR: u16 = 0x3780;
const READSTATUS: u16 = 0xF32D;
const CLEARSTATUS: u16 = 0x3041;
const HEATER_ENABLE: u16 = 0x306D;
const HEATER_DISABLE: u16 = 0x3066;
const SOFTRESET: u16 = 0x30A2;
const NOSLEEP: u16 = 0x303E;
const PERIODIC_FETCH: u16 = 0xE000;
const PERIODIC_BREAK: u16 = 0x3093;

const CRC_ALGO: Algorithm<u8> = CRC_8_NRSC_5;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum I2cAddress {
    DefaultAddress = 0x44,
    SecondaryAddress = 0x45,
}

pub enum MeasurementsPerSecond {
    OneHalf,
    One,
    Two,
    Four,
    Ten,
}

pub struct Sht32d<I>
where
    I: I2c,
{
    device: I,
    address: I2cAddress,
    crc: Crc<u8>,
}

pub enum ClockStretch {
    Enabled,
    Disabled,
}

#[derive(Copy, Clone, Debug)]
pub enum Repeatability {
    Low,
    Medium,
    High,
}

impl Repeatability {
    pub fn delay(&self) -> Duration {
        Duration::from_millis(
            match self {
                Repeatability::Low => 4,
                Repeatability::Medium => 6,
                Repeatability::High => 15
            }
        )
    }
}

pub struct Measurement {
    pub temperature: i32,
    pub humidity: f32,
}

enum Command {
    SingleFetch(ClockStretch, Repeatability),
    ReadSerialNumber,
    ReadStatus,
    ClearStatus,
    HeaterEnable,
    HeaterDisable,
    SoftReset,
    NoSleep,
    PeriodicFetch(MeasurementsPerSecond, Repeatability),
    PeriodicBreak,
    PeriodicAcceleratedResponseTime,
}

impl Command {
    fn code(&self) -> Vec<u8> {
        let res: &[u8] = match self {
            Command::SingleFetch(stretch, repeat) => match (stretch, repeat) {
                (ClockStretch::Disabled, Repeatability::Low) => &[0x24, 0x16],
                (ClockStretch::Disabled, Repeatability::Medium) => &[0x24, 0x0B],
                (ClockStretch::Disabled, Repeatability::High) => &[0x24, 0x00],
                (ClockStretch::Enabled, Repeatability::Low) => &[0x2C, 0x10],
                (ClockStretch::Enabled, Repeatability::Medium) => &[0x2C, 0x0D],
                (ClockStretch::Enabled, Repeatability::High) => &[0x2C, 0x06],
            },
            Command::ReadSerialNumber => &[0x37, 0x80],
            Command::ReadStatus => &[0xF3, 0x2D],
            Command::ClearStatus => &[0x30, 0x41],
            Command::HeaterEnable => &[0x30, 0x6D],
            Command::HeaterDisable => &[0x30, 0x66],
            Command::SoftReset => &[0x30, 0xA2],
            Command::NoSleep => &[0x30, 0x3E],
            Command::PeriodicFetch(per_sec, repeat) => match (per_sec, repeat) {
                (MeasurementsPerSecond::OneHalf, Repeatability::Low) => &[0xE0, 0x00, 0x20, 0x2F],
                (MeasurementsPerSecond::OneHalf, Repeatability::Medium) => &[0xE0, 0x00, 0x20, 0x24],
                (MeasurementsPerSecond::OneHalf, Repeatability::High) => &[0xE0, 0x00, 0x20, 0x32],
                (MeasurementsPerSecond::One, Repeatability::Low) => &[0xE0, 0x00, 0x21, 0x2D],
                (MeasurementsPerSecond::One, Repeatability::Medium) => &[0xE0, 0x00, 0x21, 0x26],
                (MeasurementsPerSecond::One, Repeatability::High) => &[0xE0, 0x00, 0x21, 0x30],
                (MeasurementsPerSecond::Two, Repeatability::Low) => &[0xE0, 0x00, 0x22, 0x2B],
                (MeasurementsPerSecond::Two, Repeatability::Medium) => &[0xE0, 0x00, 0x22, 0x20],
                (MeasurementsPerSecond::Two, Repeatability::High) => &[0xE0, 0x00, 0x22, 0x36],
                (MeasurementsPerSecond::Four, Repeatability::Low) => &[0xE0, 0x00, 0x23, 0x29],
                (MeasurementsPerSecond::Four, Repeatability::Medium) => &[0xE0, 0x00, 0x23, 0x22],
                (MeasurementsPerSecond::Four, Repeatability::High) => &[0xE0, 0x00, 0x23, 0x34],
                (MeasurementsPerSecond::Ten, Repeatability::Low) => &[0xE0, 0x00, 0x27, 0x2A],
                (MeasurementsPerSecond::Ten, Repeatability::Medium) => &[0xE0, 0x00, 0x27, 0x21],
                (MeasurementsPerSecond::Ten, Repeatability::High) => &[0xE0, 0x00, 0x27, 0x37],
            },
            Command::PeriodicBreak => &[0x30, 0x93],
            Command::PeriodicAcceleratedResponseTime => &[0x2B, 0x32],
        };
        res.to_vec()
    }
}

impl<I> Sht32d<I>
where
    I: I2c,
{
    pub fn new(i2c_device: I, i2c_address: I2cAddress) -> Self {
        Self {
            device: i2c_device,
            address: i2c_address,
            crc: Crc::<u8>::new(&CRC_8_NRSC_5),
        }
    }

    fn execute_command(
        &mut self,
        command: Command,
    ) -> Result<(), I::Error> {
        self.device.write(self.address as u8, &command.code())?;
        Ok(())
    }

    pub fn measure(&mut self) -> Result<Measurement, I::Error> {
        let repeatability = Repeatability::High;
        self.execute_command(Command::SingleFetch(
            ClockStretch::Enabled,
            repeatability,
        ))?;
        let mut buffer = [0; 6];
        sleep(repeatability.delay());
        self.device.read(self.address as u8, &mut buffer)?;
        let (temp_crc, humidity_crc) = buffer.split_at(3);
        let temp_checksum = self.crc.checksum(&[temp_crc[0], temp_crc[1]]);
        if temp_crc[2] != temp_checksum {

        }
        let humidity_checksum = self.crc.checksum(&[humidity_crc[0], humidity_crc[1]]);
        if humidity_crc[2] != humidity_checksum {
            return Ok((0., 0.)); // todo: make error
        }
        let temp = u16::from_be_bytes([temp_crc[0], temp_crc[1]]);
        let humidity = u16::from_be_bytes([temp_crc[0], temp_crc[1]]);

        Ok((
            -45. + (175. * (temp as f64 / 65535. /*  2^16 - 1 */)),
            (100 * humidity) / (u16::max() - 1) /* 2^16 - 1 */,
        ))
    }
}
