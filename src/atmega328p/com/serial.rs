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
//! See the section 19 of ATMEGA328P datasheet.

// Source code crates required
use crate::atmega328p::com::usart_initialize::{Usart, UsartNum};

/// This struct contains  USART0 in ARDUINO MEGA arranged in a array.
/// First a new Serial is needed to be created to access all USARTs.
#[repr(C, packed)]
pub struct Serial {
    pub usart: [&'static mut Usart; 1],
}

impl Serial {
    /// This function creates a new Serial struct.
    /// The struct serial will contain all the USARTs at one place.
    /// # Returns
    /// * `a struct object` - Which is to be worked upon.
    pub unsafe fn new() -> Serial {
        Serial {
            usart: [Usart::new(UsartNum::Usart0)],
        }
    }
}
