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

/*
/// Codes to be used for this file.
pub mod usart_initialize;
pub mod usart_recieve;
pub mod usart_transmit;
*/

/// Crates which would be used in the implementation.
use crate::rustduino::atmega2560p::com::usart_initialize;
use crate::rustduino::atmega2560p::com::usart_recieve;
use crate::rustduino::atmega2560p::com::usart_transmit;


/// Default setting parameters for various modes of USART in case user want's to skip them.
/// Baud Rate.
const baud : i64 = 2400;
/// Frame Settings.
const size : (usart_initialize::UsartDataSize ) = eight;
const parity : (usart_initialize::UsartParity ) = no;
const stop : (usart_initialize::UsartStop ) = one;
/// USART mode.
const mode : (usart_initialize::UsartModes) = norm_async;
/// Default USART number to be used.
const num : (usart_initialize::UsartNum ) = usart0;
/// Default clock polarity mode.
const polarity : (usart_initialize::UsartPolarity ) = output_rise;


/// Main println() function for using USART according to default used values.
pub fn println() {
    pub fn begin(&mut self, baud: i64, mode:UsartModes, stop : UsartStop, size : UsartDataSize, parity : UsartParity) {
        initialize(&mut self, mode, baud, stop, size, parity);
        Transmit_enable(); 
        recieve_enable();
    }
    
    pub fn end() {
        Transmit_disable();
        recieve_disable();
    }
}

/// Main println() function for using USART according to default used values and user defined value of baud rate.
pub fn println_set_baud() {}

/// Main println() function for using USART according to default used values and user defined value of frame.
pub fn println_set_frame() {}

/// Main println() function for using USART according to default used values and user defined value of frame.
pub fn println_detail() {}
