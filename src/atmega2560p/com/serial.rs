//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Nikhil Gupta, Indian Institute of Technology Kanpur
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


//! This files contain the code for combining all serial ports into one structure for easier implementation.
//! See the section 22 of ATMEGA2560P datasheet.
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf


/// Crates which would be used in the implementation.
/// We will be using standard volatile and bit_field crates now for a better read and write.
use core::ptr::read_volatile;
use bit_field::BitField;
use volatile::Volatile;
use rustduino::atmega2560p::usart_initialize::{Usart,UsartName};
use rustduino::atmega2560p::usart_recieve;
use rustduino::atmega2560p::usart_transmit;


/// This struct contains all 4 USART in ARDUINO MEGA arranged in a array.
/// First a new Serial is needed to be created to access all USARTs.
/// Each USART can be accesed through Serial.usart[n], where 0<= n <=3 
pub struct Serial {
    usart : [Usart ; 4],
}

impl Serial{
    /// This function creates a new Serial struct.
    /// The struct serial will contain all the USARTs at one place.
    pub fn new() -> Serial {
        unsafe {
            Serial {
                usart:[
                    Usart::new(UsartName::usart0).unwrap(),
                    Usart::new(UsartName::usart1).unwrap(),
                    Usart::new(UsartName::usart2).unwrap(),
                    Usart::new(UsartName::usart3).unwrap(),
                ]
            }
        }
    }
}