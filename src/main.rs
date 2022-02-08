pub mod device;

use std::{
    hash::Hasher,
    ops::{Deref, DerefMut},
    thread::sleep_ms,
};

use embedded_hal::blocking::delay::DelayMs;
use i2cdev::{core::I2CDevice, linux::LinuxI2CError};
use linux_embedded_hal::{Delay, I2cdev};

fn main() -> Result<(), device::TroykaHatError<LinuxI2CError>> {
    let i2c = I2cdev::new("/dev/i2c-1").map_err(device::TroykaHatError::I2c)?;
    let mut th = device::TroykaHat::new(i2c);
    let mut delay = Delay;

    th.init(&mut delay)?;
    // servo test
    th.pwm_freq(50);
    loop {
        th.analog_write(7, 12);
        sleep_ms(1000);
        th.analog_write(7, 25);
        sleep_ms(1000);
    }
    Ok(())
}
