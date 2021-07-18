//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Devansh Kumar Jha, Indian Institute of Technology Kanpur
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

//! Functions provided to the user for ATMEGA2560P USART implementation.
//! This file contains the println() functions in various versions which the user will
//! use to transmit data using USART on ATMEGA2560P.
//! This file combines all the functions in other USART source code to make useful functions.
//! See the section 22 of ATMEGA2560P datasheet.
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf

/// Crates which would be used in the implementation.
use crate::atmega2560p::com::serial::Serial;
use crate::atmega2560p::com::usart_initialize::Usart;
use crate::atmega2560p::com::usart_initialize::{
    UsartDataSize, UsartModes, UsartNum, UsartParity, UsartStop, UsartPolarity
};
use crate::atmega2560p::com::usart_transmit::{DataType};


/// Default setting parameters for various modes of USART in case user want's to skip them.
/// Baud Rate.
const baud: i64 = 2400;
/// Frame Settings.
const size: UsartDataSize = UsartDataSize::Eight;
const parity: UsartParity = UsartParity::No;
const stop: UsartStop = UsartStop::One;
/// USART mode.
const mode: UsartModes = UsartModes::Normasync;
/// Default USART number to be used.
const num: UsartNum = UsartNum::Usart0;
/// Default clock polarity mode.
const polarity: UsartPolarity = UsartPolarity::Outputrise;


impl Serial {
    /// Gives a new serial port object which can be used to control all the
    /// USART at one place.
    pub fn serial_new() -> Serial {
        Serial::new()
    }
}

impl Usart {
    /// This function can be use to initialize with default settings.
    /// Like Mode:Normal asynchronuous,stopbit:one,data bit:8,parity type:no
    pub fn begin(&mut self) {
        self.transmit_enable();
        self.recieve_enable();
        self.initialize(mode, baud, stop, size, parity);
    }

    /// This function can be use to initialize with baud rate and remaining settings will be set to default
    /// Like Mode:Normal asynchronuous,stopbit:one,data bit:8,parity type:no
    pub fn begin_set_baud(&mut self, baud1: i64) {
        self.transmit_enable();
        self.recieve_enable();
        self.initialize(mode, baud1, stop, size, parity);
    }

    /// Generic function to transmit data through the USART.
    pub fn write(&mut self, data: DataType) {
        match data {
            DataType::String => {
                self.write_string(data);
            }
            DataType::Integer => {
                self.write_integer(data);
            }
            DataType::Float => {
                self.write_float(data);
            }
        }
    }

    /// This function can be used to stop the functioning of USART.
    pub fn end(&mut self) {
        self.transmit_disable();
        self.recieve_disable();
    }
}

/// Main println() function for using USART according to default used values.
/// Transmitter mode is first enabled for the default usart.
/// Then the function takes the usart and initializes it.
/// Then the string given by the user is transmitted through the USART.
pub fn println(data: &'static mut str) {
    let u: Usart = unsafe { *Usart::new(num) };
    
    u.transmit_enable();
    u.initialize(mode, baud, stop, size, parity);
    u.write_string(data);
    u.transmit_disable();
}

/// println() function for using USART according to default used values and user defined value of baud rate.
/// Transmitter mode is first enabled for the default usart.
/// Then the function takes the usart and initializes it with user defined.
/// Then the string given by the user is transmitted through the USART.
pub fn println_set_baud(data: &str, baud1: i64) {
    let u: Usart = unsafe { *Usart::new(num) };
    u.transmit_enable();
    u.initialize(mode, baud1, stop, size, parity);
    u.write_string(data);
    u.transmit_disable();
}

/// Main println() function for using USART according to default used values and user defined value of frame.
/// Transmitter mode is first enabled for the default usart.
/// Then the function takes the usart and initializes it.
/// Then the string given by the user is transmitted through the USART.
pub fn println_set_frame(data: &str, size1: UsartDataSize, parity1: UsartParity, stop1: UsartStop) {
    let u: Usart = unsafe { *Usart::new(num) };
    u.transmit_enable();
    u.initialize(mode, baud, stop1, size1, parity1);
    u.write_string(data);
    u.transmit_disable();
}

/// Main println() function for using USART according to user defined mode parameters.
/// Transmitter mode is first enabled for the default usart.
/// Then the function takes the usart and initializes it.
/// Then the string given by the user is transmitted through the USART.
pub fn println_detail(
    data: &str,
    num1: UsartNum,
    mode1: UsartModes,
    baud1: i64,
    size1: UsartDataSize,
    parity1: UsartParity,
    stop1: UsartStop,
) {
    let u: Usart = unsafe { *Usart::new(num) };
    u.transmit_enable();
    u.initialize(mode1, baud1, stop1, size1, parity1);
    u.write_string(data);
    u.transmit_disable();
}
