pub mod device;

use std::{hash::Hasher, ops::{DerefMut, Deref}, thread::sleep_ms};

use embedded_hal::blocking::delay::DelayMs;
use i2cdev::{linux::LinuxI2CError, core::I2CDevice};
use linux_embedded_hal::{Delay, I2cdev};

fn main() -> Result<(), device::TroykaHatError<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1").map_err(device::TroykaHatError::I2c)?;
    let mut th = device::TroykaHat::new(i2c);
    let mut delay = Delay;
    
    th.init(&mut delay)?;
    
    loop {
        th.analog_write(6, 50);
        sleep_ms(300);
        th.analog_write(6, 250);
        sleep_ms(500);
    }
    Ok(())
}
