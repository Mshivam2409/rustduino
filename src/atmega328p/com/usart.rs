//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021 Richa Prakash Sachan, Indian Institute of Technology Kanpur
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Affero General Public License as published
//     by the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Affero General Public License for more details.
//
//     You should have received a copy of the GNU Affero General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>

//! Functions provided to the user for ATMEGA328P USART implementation.
//! This file contains the println() functions in various versions which the user will
//! use to transmit data using USART on ATMEGA328P.
//! This file combines all the functions in other USART source code to make useful functions.
//! See the section 19 and 20 of ATMEGA328P datasheet.

// Crates which would be used in the implementation.
//
use crate::atmega328p::com::serial::Serial;
use crate::atmega328p::com::usart_initialize::Usart;
use crate::atmega328p::com::usart_initialize::{
    UsartDataSize, UsartModes, UsartNum, UsartParity, UsartPolarity, UsartStop,
};

// Standard datatypes to be used
use core::{f64, u32};

// Default setting parameters for various modes of USART in case user want's to skip them.
// Baud Rate.
const BAUD: i64 = 2400;
// Frame Settings.
const SIZE: UsartDataSize = UsartDataSize::Eight;

const PARITY: UsartParity = UsartParity::No;
const STOP: UsartStop = UsartStop::One;
// USART mode.
const MODE: UsartModes = UsartModes::Normasync;
// Default USART number to be used.
const NUM: UsartNum = UsartNum::Usart0;
// Default clock polarity mode.
const _POLARITY: UsartPolarity = UsartPolarity::Outputrise;

impl Serial {
    /// Gives a new serial port object which can be used to control all the
    /// USART at one place.
    /// This is just a alternative function to the new() function given for stability.
    pub unsafe fn serial_new() -> Serial {
        Serial::new()
    }
}

impl Usart {
    /// This function can be use to initialize with default settings.
    /// Like Mode:Normal asynchronuous,stopbit:one,data bit:8,parity type:no
    pub unsafe fn begin(&mut self) {
        self.transmit_enable();
        self.recieve_enable();
        self.initialize(MODE, BAUD, STOP, SIZE, PARITY);
    }

    /// This function can be use to initialize with baud rate and remaining settings will be set to default
    /// Like Mode:Normal asynchronuous,stopbit:one,data bit:8,parity type:no
    /// # Arguments
    /// * `baud1` - a i64, the baud rate of USART the user wants to set.
    pub unsafe fn begin_set_baud(&mut self, baud1: i64) {
        self.transmit_enable();
        self.recieve_enable();
        self.initialize(MODE, baud1, STOP, SIZE, PARITY);
    }

    /// This function can be used to stop the functioning of USART.
    pub unsafe fn end(&mut self) {
        self.transmit_disable();
        self.recieve_disable();
    }
}

/// Main println() function for using USART according to default used values.
/// Transmitter mode is first enabled for the default usart.
/// Then the function takes the usart and initializes it.
/// Then the string given by the user is transmitted through the USART.
/// # Arguments
/// * `data` - a string object, which is to be transmitted using USART.
pub fn println_string(data: &'static str) {
    let u: &mut Usart = unsafe { Usart::new(NUM) };
    u.transmit_enable();
    u.initialize(MODE, BAUD, STOP, SIZE, PARITY);
    u.write_string(data);
    u.transmit_disable();
}

/// Main println() function for using USART according to default used values.
/// Transmitter mode is first enabled for the default usart.
/// Then the function takes the usart and initializes it.
/// Then the string given by the user is transmitted through the USART.
/// This will be used to transmit integer data.
/// # Arguments
/// * `data` - a u32, which is to be transmitted using USART.
pub fn println_integer(data: u32) {
    let u: &mut Usart = unsafe { Usart::new(NUM) };
    u.transmit_enable();
    u.initialize(MODE, BAUD, STOP, SIZE, PARITY);
    u.write_integer(data);
    u.transmit_disable();
}

/// Main println() function for using USART according to default used values.
/// Transmitter mode is first enabled for the default usart.
/// Then the function takes the usart and initializes it.
/// Then the string given by the user is transmitted through the USART.
/// This will be used to transmit float data.
/// # Arguments
/// * `data` - a f32, which is to be transmitted using USART.
/// * `precision` - a u32, the number of decimal precision required in the transmission.
pub fn println_float(data: f64, precision: u32) {
    let u: &mut Usart = unsafe { Usart::new(NUM) };
    u.transmit_enable();
    u.initialize(MODE, BAUD, STOP, SIZE, PARITY);
    u.write_float(data, precision);
    u.transmit_disable();
}

/// println() function for using USART according to default used values and user defined value of baud rate.
/// Transmitter mode is first enabled for the default usart.
/// Then the function takes the usart and initializes it with user defined.
/// Then the string given by the user is transmitted through the USART.
/// # Arguments
/// * `data` - a string object, which is to be transmitted using USART.
/// * `baud1` - a i64, the baud rate of USART the user wants to set.
pub fn println_set_baud(data: &'static str, baud1: i64) {
    let u: &mut Usart = unsafe { Usart::new(NUM) };
    u.transmit_enable();
    u.initialize(MODE, baud1, STOP, SIZE, PARITY);
    u.write_string(data);
    u.transmit_disable();
}

/// Main println() function for using USART according to default used values and user defined value of frame.
/// Transmitter mode is first enabled for the default usart.
/// Then the function takes the usart and initializes it.
/// Then the string given by the user is transmitted through the USART.
/// # Arguments
/// * `data` - a string object, which is to be transmitted using USART.
/// * `size1` - a `UsartDatSize` object, the size of set of bits to transmit.
/// * `parity1` - a `UsartParity` object, which gives the Parity bit mode for USART.
/// * `stop1` - a `UsartStop` object, which will be used to set the stop bits of data frame.
pub fn println_set_frame(
    data: &'static str,
    size1: UsartDataSize,
    parity1: UsartParity,
    stop1: UsartStop,
) {
    let u: &mut Usart = unsafe { Usart::new(NUM) };
    u.transmit_enable();
    u.initialize(MODE, BAUD, stop1, size1, parity1);
    u.write_string(data);
    u.transmit_disable();
}

/// Main println() function for using USART according to user defined mode parameters.
/// Transmitter mode is first enabled for the default usart.
/// Then the function takes the usart and initializes it.
/// Then the string given by the user is transmitted through the USART.
/// # Arguments
/// * `data` - a string object, which is to be transmitted using USART.
/// * `num1` - a `UsartNum` object, which defines the USART to be used.
/// * `mode1` - a `UsartModes` object, which defines the mode of USART to use.
/// * `baud1` - a i64, the baud rate of USART the user wants to set.
/// * `size1` - a `UsartDatSize` object, the size of set of bits to transmit.
/// * `parity1` - a `UsartParity` object, which gives the Parity bit mode for USART.
/// * `stop1` - a `UsartStop` object, which will be used to set the stop bits of data frame.
pub fn println_detail(
    data: &'static str,
    num1: UsartNum,
    mode1: UsartModes,
    baud1: i64,
    size1: UsartDataSize,
    parity1: UsartParity,
    stop1: UsartStop,
) {
    let u: &mut Usart = unsafe { Usart::new(num1) };
    u.transmit_enable();
    u.initialize(mode1, baud1, stop1, size1, parity1);
    u.write_string(data);
    u.transmit_disable();
}
