use embedded_hal::{
    blocking::delay::DelayMs,
    blocking::i2c::{Write, WriteRead},
};

/// Slave address of TroykaHat
pub const TROYKA_I2C_ADDRESS: u8 = 0x2a;
/// Internal register to check slave addr
pub const WHOAMI: u8 = 0x00;

pub const PORT_MODE_INPUT: u8 = 0x04;
pub const PORT_MODE_OUTPUT: u8 = 0x07;
pub const ANALOG_WRITE: u8 = 0x0b;
pub const ANALOG_READ: u8 = 0x0c;
pub const PWM_FREQ: u8 = 0x0d;
pub const ADC_SPEED: u8 = 0x0e;

/// All possible errors in this crate
#[derive(Debug)]
pub enum TroykaHatError<E> {
    /// I2C bus error
    I2c(E),

    /// Invalid chip ID was read
    InvalidChipId(u8),
}

pub struct TroykaHat<I> {
    i2c: I,
    i2c_addr: u8,
}

impl<I, E> TroykaHat<I>
where
    I: Write<Error = E> + WriteRead<Error = E>,
{
    /// Side effect free constructor
    pub fn new(i2c: I) -> Self {
        TroykaHat {
            i2c,
            i2c_addr: TROYKA_I2C_ADDRESS,
        }
    }

    /// Init wakes MPU6050 and verifies register addr, e.g. in i2c
    pub fn init<D: DelayMs<u8>>(&mut self, delay: &mut D) -> Result<(), TroykaHatError<E>> {
        self.wake(delay)?;
        self.verify()?;

        Ok(())
    }

    // TODO always output
    pub fn pin_mode(&mut self, pin: u8) {
        let data = 0x0001 << pin;
        //let send_data:u16 = ((data & 0xff) << 8) | ((data >> 8) & 0xff);

        self.i2c.write(self.i2c_addr, &[PORT_MODE_OUTPUT, 0x80]);
    }

    pub fn analog_write(&mut self, pin: u8, value: u8) {
        self.i2c
            .write(self.i2c_addr, &[ANALOG_WRITE, (pin & 0xff), (value & 0xff)]);
    }

    /// Wakes TroykaHat
    fn wake<D: DelayMs<u8>>(&mut self, delay: &mut D) -> Result<(), TroykaHatError<E>> {
        // TODO I2cdev::new("/dev/i2c-1").map_err(device::TroykaHatError::I2c)?;
        delay.delay_ms(100u8);
        Ok(())
    }

    /// Verifies device address with WHOAMI.addr() Register
    fn verify(&mut self) -> Result<(), TroykaHatError<E>> {
        // TODO
        // let mut buf: [u8; 32] = [0; 32];
        // self.read_bytes(WHOAMI, &mut buf)?;
        // if let Ok(s) = str::from_utf8(&buf) {
        //     println!("{}", s);
        // }
        // let address = self.read_byte(WHOAMI)?;
        // if address != TROYKA_I2C_ADDRESS {
        //     return Err(TroykaHatError::InvalidChipId(address));
        // }
        Ok(())
    }

    /// Reads byte from register
    pub fn read_byte(&mut self, reg: u8) -> Result<u8, TroykaHatError<E>> {
        let mut byte: [u8; 1] = [0; 1];
        self.i2c
            .write_read(self.i2c_addr, &[reg], &mut byte)
            .map_err(TroykaHatError::I2c)?;
        Ok(byte[0])
    }

    /// Reads series of bytes into buf from specified reg
    pub fn read_bytes(&mut self, reg: u8, buf: &mut [u8]) -> Result<(), TroykaHatError<E>> {
        self.i2c
            .write_read(self.i2c_addr, &[reg], buf)
            .map_err(TroykaHatError::I2c)?;
        Ok(())
    }

    /// Writes byte to register
    pub fn write_byte(&mut self, reg: u8, byte: u8) -> Result<(), TroykaHatError<E>> {
        self.i2c
            .write(self.i2c_addr, &[reg, byte])
            .map_err(TroykaHatError::I2c)?;

        Ok(())
    }
}
