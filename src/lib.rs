//! LP55231
//!
//! This is a driver for the [TI LP55231](http://www.ti.com/product/LP55231) RGB LED controller IC
//! using the [`embedded_hal`](https://github.com/rust-embedded/embedded-hal/) traits.
//!
//! NOTE that this driver is not yet platform agnostic, as it relies on `cortex_m` for a delay.
//!
//! This driver optionally takes a [digital output
//! pin](https://docs.rs/embedded-hal/0.2.1/embedded_hal/digital/trait.OutputPin.html) to control
//! power to the LP55231. It will drive the pin (digital) high on power-on, and (digital) low on
//! power-off.
#![no_std]
#![deny(missing_docs)]

extern crate cortex_m; // This is used for asm::delay(); TODO replace with an embedded-hal Delay
extern crate embedded_hal as hal;
#[macro_use]
extern crate bitflags;

use hal::blocking::i2c::{Write, WriteRead};
use hal::digital::OutputPin;

pub mod registers;
use registers as reg;

#[derive(Copy, Clone)]
/// Available I2C addresses for the part
pub enum Addr {
    /// ASEL1=GND, ASEL2=GND
    _0x32,
    /// ASEL1=GND, ASEL2=VEN
    _0x33,
    /// ASEL1=VEN, ASEL2=GND
    _0x34,
    /// ASEL1=VEN, ASEL2=VEN
    _0x35,
}

impl From<Addr> for u8 {
    fn from(a: Addr) -> Self {
        match a {
            Addr::_0x32 => 0x32_u8,
            Addr::_0x33 => 0x32_u8,
            Addr::_0x34 => 0x32_u8,
            Addr::_0x35 => 0x32_u8,
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// Enumeration of the 9 LED lines from the chip
pub enum D {
    /// LED line D1
    D1,
    /// LED line D2
    D2,
    /// LED line D3
    D3,
    /// LED line D4
    D4,
    /// LED line D5
    D5,
    /// LED line D6
    D6,
    /// LED line D7
    D7,
    /// LED line D8
    D8,
    /// LED line D9
    D9,
}

impl From<D> for u8 {
    fn from(d: D) -> Self {
        match d {
            D::D1 => 0,
            D::D2 => 1,
            D::D3 => 2,
            D::D4 => 3,
            D::D5 => 4,
            D::D6 => 5,
            D::D7 => 6,
            D::D8 => 7,
            D::D9 => 8,
        }
    }
}

/// The LP55231 device
pub struct Lp55231<I, P> {
    /// The owned I2C bus
    i2c: I,
    /// The owned enable pin
    en_pin: Option<P>,
    /// The I2C address of this device
    addr: u8,
}

impl<E, I, P> Lp55231<I, P>
where
    I: Write<Error = E> + WriteRead<Error = E>,
    P: OutputPin,
{
    /// Create a new instance of an LP55231 that exclusively owns its I2C bus. Optionally takes a
    /// power control pin.
    pub fn new(i2c: I, en_pin: Option<P>, addr: Addr) -> Self {
        Lp55231 {
            i2c,
            en_pin,
            addr: u8::from(addr) << 1,
        }
    }

    /// Convenience method to call `self.i2c.write` with `self.addr`
    fn send(&mut self, bytes: &[u8]) -> Result<(), E> {
        self.i2c.write(self.addr, bytes)?;
        Ok(())
    }

    /// Enable the device for use
    ///
    /// Sets the enable line high, then sends an enable command, waits 500us, and then configures
    /// to device to use its internal clock, enable the charge pump at 1.5x boost, and
    /// auto-increment on writes.
    pub fn enable(&mut self) -> Result<(), E> {
        if let Some(p) = self.en_pin.as_mut() {
            p.set_high();
        }
        cortex_m::asm::delay(16000); // 500us @ 32 MHz; 0.12s @ 132 kHz. minimum 500us delay
        self.send(&[reg::CNTRL1, (reg::Cntrl1::CHIP_EN).bits()])?;
        self.send(&[
            reg::MISC,
            (reg::Misc::INT_CLK_EN
                | reg::Misc::CLK_DET_EN
                | reg::Misc::CP_MODE_1_5x
                | reg::Misc::EN_AUTO_INCR)
                .bits(),
        ])?;
        Ok(())
    }

    /// Soft-reset the device NOW
    pub fn reset(&mut self) -> Result<(), E> {
        self.send(&[reg::Reset::RESET_NOW.bits()])?;
        Ok(())
    }

    /// Turn off the device NOW
    pub fn disable(&mut self) {
        if let Some(p) = self.en_pin.as_mut() {
            p.set_low();
        }
    }

    /// Set the D line to the provided PWM value
    pub fn set_pwm(&mut self, d: D, pwm: u8) -> Result<(), E> {
        self.send(&[reg::D_PWM_BASE + u8::from(d), pwm])?;
        Ok(())
    }
}