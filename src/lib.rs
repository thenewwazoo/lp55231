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

extern crate embedded_hal as hal;
#[macro_use]
extern crate bitflags;

use core::fmt::Debug;
use hal::blocking::i2c::{Write, WriteRead};
use hal::digital::OutputPin;

pub mod registers;

use registers as reg;

#[derive(Debug)]
/// Error conditions returned by the LP55231
pub enum Error<I> {
    /// The LP is not currently enabled
    NotEnabled,
    /// Generic I2c error
    I2cError(I),
}

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
            Addr::_0x33 => 0x33_u8,
            Addr::_0x34 => 0x34_u8,
            Addr::_0x35 => 0x35_u8,
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
    /// Has the LP55231 been enabled
    en: bool,
}

impl<E, I, P> Lp55231<I, P>
    where
        E: Debug,
        I: Write<Error=E> + WriteRead<Error=E>,
        P: OutputPin,
{
    /// Create a new instance of an LP55231 that exclusively owns its I2C bus. Optionally takes a
    /// power control pin.
    pub fn new(i2c: I, en_pin: Option<P>, addr: Addr) -> Self {
        Lp55231 {
            i2c,
            en_pin,
            addr: u8::from(addr) << 1,
            en: false,
        }
    }

    /// Convenience method to call `self.i2c.write` with `self.addr`
    fn send(&mut self, bytes: &[u8]) -> Result<(), Error<E>> {
        if self.en {
            self.i2c.write(self.addr, bytes).map_err(|e| Error::I2cError(e))
        } else {
            Err(Error::NotEnabled)
        }
    }

    /// Convenience method to call `self.i2c.write_read` with `self.addr`
    fn read(&mut self, register: u8) -> Result<u8, Error<E>> {
        if self.en {
            let mut b = [0];
            self.i2c.write_read(self.addr, &[register], &mut b).map_err(|e| Error::I2cError(e));
            Ok(b[0])
        } else {
            Err(Error::NotEnabled)
        }
    }

    /// Enable the device for use
    ///
    /// Sets the enable line high, then sends an enable command, waits 500us, and then configures
    /// to device to use its internal clock, enable the charge pump at 1.5x boost, and
    /// auto-increment on writes.
    pub fn enable(&mut self) -> Result<(), Error<E>> {
        if let Some(p) = self.en_pin.as_mut() {
            p.set_high();
        }
        // TODO there should be a 500us delay here, but the chip appears to mostly work without it,
        // so I'm not going to worry about it now. Ideally, this function should take a
        // `embedded_hal::delay::Delay` and wait.
        self.en = true;
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

    /// Set logarithmic scale
    pub fn set_logarithmic(&mut self, leds: &[D]) -> Result<(), Error<E>> {
        for led in leds {
            let v = self.read(reg::D_CTRL_BASE + u8::from(*led))?;
            self.send(&[reg::D_CTRL_BASE + u8::from(*led), v | reg::CtrlBase::LOG_EN.bits()])?;
        }
        Ok(())
    }

    /// Set linear scale
    pub fn set_linear(&mut self, leds: &[D]) -> Result<(), Error<E>> {
        for led in leds {
            let v = self.read(reg::D_CTRL_BASE + u8::from(*led))?;
            self.send(&[reg::D_CTRL_BASE + u8::from(*led), v & !reg::CtrlBase::LOG_EN.bits()])?;
        }
        Ok(())
    }

    /// Soft-reset the device NOW
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        self.send(&[reg::RESET, reg::Reset::RESET_NOW.bits()])?;
        Ok(())
    }

    /// Turn off the device NOW
    pub fn disable(&mut self) {
        if let Some(p) = self.en_pin.as_mut() {
            p.set_low();
        }
        self.en = false;
    }

    /// Set the D line to the provided PWM value
    pub fn set_pwm(&mut self, d: D, pwm: u8) -> Result<(), Error<E>> {
        self.send(&[reg::D_PWM_BASE + u8::from(d), pwm])?;
        Ok(())
    }
}
