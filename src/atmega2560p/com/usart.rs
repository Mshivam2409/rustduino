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

/// Main println() function for using USART according to default used values.
pub fn println() {}

/// Main println() function for using USART according to default used values and user defined value of baud rate.
pub fn println_set_baud() {}

/// Main println() function for using USART according to default used values and user defined value of frame.
pub fn println_set_frame() {}

/// Main println() function for using USART according to default used values and user defined value of frame.
pub fn println_detail() {}
