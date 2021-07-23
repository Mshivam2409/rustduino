#![allow(dead_code)]
#![allow(unused_imports)]

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


//! This code contains two functions to generate random numbers using
//! the analog read implementation and MPU 6050 gyroscopic sensor.
//! ARDUINO being a micro-controller doesn't contain very precise and robust random
//! number generator algorithms which are both fast and truly random.
//! In this source code we try to implement two fairly efficient ways of generating random numbers.


/// Standard crates required.
use core;
use bit_field::BitField;
use volatile::Volatile;

/// Source codes required.
use crate::atmega2560p::hal::analogpins::AnalogPins as mega_pins;
use crate::atmega328p::hal::analogpins::AnalogPins as uno_pins;


/// Structure to control the implementation of Random Number Generators
pub struct RandomNumberGenerator {

}

impl RandomNumberGenerator {

}
