//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Ayush Agarwal, Indian Institute of Technology Kanpur
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

// Include the required crates for the code.
use crate::atmega328p::hal::port::*;

/// This struct contains the Pin struct and its analog pin number.
pub struct AnalogPin {
    pub anapin: Pin,
    pub pinno: u32,
}

/// This structs contains all the 6 analog pins of Arduino UNO in form of an array.
pub struct AnalogPins {
    pub analog: [AnalogPin; 6],
}

impl AnalogPins {
    // This function creates returns all analog pins.
    // This pins can be accesed like an array.
    pub fn new() -> AnalogPins {
        AnalogPins {
            analog: [
                AnalogPin {
                    anapin: Pin::new(PortName::C, 0).unwrap(),
                    pinno: 0,
                },
                AnalogPin {
                    anapin: Pin::new(PortName::C, 1).unwrap(),
                    pinno: 1,
                },
                AnalogPin {
                    anapin: Pin::new(PortName::C, 2).unwrap(),
                    pinno: 2,
                },
                AnalogPin {
                    anapin: Pin::new(PortName::C, 3).unwrap(),
                    pinno: 3,
                },
                AnalogPin {
                    anapin: Pin::new(PortName::C, 4).unwrap(),
                    pinno: 4,
                },
                AnalogPin {
                    anapin: Pin::new(PortName::C, 5).unwrap(),
                    pinno: 5,
                },
            ],
        }
    }
}
