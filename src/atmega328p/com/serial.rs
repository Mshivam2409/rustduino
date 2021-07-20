//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Richa Prakash Sachan, Indian Institute of Technology Kanpur
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

//! This files contain the code for combining  serial ports into a structure for easier implementation.
//!  https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf

/// Crates which would be used in the implementation.
/// We will be using standard volatile and bit_field crates now for a better read and write.
use crate::atmega328p::com::usart_initialize::{Usart, UsartNum};

/// This struct contains  USART0 in ARDUINO MEGA arranged in a array.
/// First a new Serial is needed to be created to access all USARTs.

pub struct Serial {
    pub usart: [&'static mut Usart; 1],
}

impl Serial {
    /// This function creates a new Serial struct.
    /// The struct serial will contain all the USARTs at one place.
    pub fn new() -> Serial {
        unsafe {
            Serial {
                usart: [Usart::new(UsartNum::Usart0)],
            }
        }
    }
}
