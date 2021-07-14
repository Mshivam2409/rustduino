//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Nikhil Gupta,Indian Institute of Technology Kanpur
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

/// Include the required crates for the code.
use crate::atmega2560p::hal::port::*;

/// All pins inside a single struct.
pub struct Pins {
    /// All sixteen analog pins.
    pub analog: [Pin; 16],
    /// All 54 digital I/O pins.
    pub digital: [Pin; 54],
}

impl Pins {
    /// Returns all pins at once as a single struct.
    pub fn new() -> Pins {
        Pins {
            analog: [
                Pin::new(PortName::F, 0).unwrap(),
                Pin::new(PortName::F, 1).unwrap(),
                Pin::new(PortName::F, 2).unwrap(),
                Pin::new(PortName::F, 3).unwrap(),
                Pin::new(PortName::F, 4).unwrap(),
                Pin::new(PortName::F, 5).unwrap(),
                Pin::new(PortName::F, 6).unwrap(),
                Pin::new(PortName::F, 7).unwrap(),
                Pin::new(PortName::K, 0).unwrap(),
                Pin::new(PortName::K, 1).unwrap(),
                Pin::new(PortName::K, 2).unwrap(),
                Pin::new(PortName::K, 3).unwrap(),
                Pin::new(PortName::K, 4).unwrap(),
                Pin::new(PortName::K, 5).unwrap(),
                Pin::new(PortName::K, 6).unwrap(),
                Pin::new(PortName::K, 7).unwrap(),
            ],
            digital: [
                Pin::new(PortName::E, 0).unwrap(),
                Pin::new(PortName::E, 1).unwrap(),
                Pin::new(PortName::E, 4).unwrap(),
                Pin::new(PortName::E, 5).unwrap(),
                Pin::new(PortName::G, 5).unwrap(),
                Pin::new(PortName::E, 3).unwrap(),
                Pin::new(PortName::H, 3).unwrap(),
                Pin::new(PortName::H, 4).unwrap(),
                Pin::new(PortName::H, 5).unwrap(),
                Pin::new(PortName::H, 6).unwrap(),
                Pin::new(PortName::B, 4).unwrap(),
                Pin::new(PortName::B, 5).unwrap(),
                Pin::new(PortName::B, 6).unwrap(),
                Pin::new(PortName::B, 7).unwrap(),
                Pin::new(PortName::J, 0).unwrap(),
                Pin::new(PortName::J, 1).unwrap(),
                Pin::new(PortName::H, 1).unwrap(),
                Pin::new(PortName::H, 0).unwrap(),
                Pin::new(PortName::D, 3).unwrap(),
                Pin::new(PortName::D, 2).unwrap(),
                Pin::new(PortName::D, 1).unwrap(),
                Pin::new(PortName::D, 0).unwrap(),
                Pin::new(PortName::A, 0).unwrap(),
                Pin::new(PortName::A, 1).unwrap(),
                Pin::new(PortName::A, 2).unwrap(),
                Pin::new(PortName::A, 3).unwrap(),
                Pin::new(PortName::A, 4).unwrap(),
                Pin::new(PortName::A, 5).unwrap(),
                Pin::new(PortName::A, 6).unwrap(),
                Pin::new(PortName::A, 7).unwrap(),
                Pin::new(PortName::C, 7).unwrap(),
                Pin::new(PortName::C, 6).unwrap(),
                Pin::new(PortName::C, 5).unwrap(),
                Pin::new(PortName::C, 4).unwrap(),
                Pin::new(PortName::C, 3).unwrap(),
                Pin::new(PortName::C, 2).unwrap(),
                Pin::new(PortName::C, 1).unwrap(),
                Pin::new(PortName::C, 0).unwrap(),
                Pin::new(PortName::D, 7).unwrap(),
                Pin::new(PortName::G, 2).unwrap(),
                Pin::new(PortName::G, 1).unwrap(),
                Pin::new(PortName::G, 0).unwrap(),
                Pin::new(PortName::L, 7).unwrap(),
                Pin::new(PortName::L, 6).unwrap(),
                Pin::new(PortName::L, 5).unwrap(),
                Pin::new(PortName::L, 4).unwrap(),
                Pin::new(PortName::L, 3).unwrap(),
                Pin::new(PortName::L, 2).unwrap(),
                Pin::new(PortName::L, 1).unwrap(),
                Pin::new(PortName::L, 0).unwrap(),
                Pin::new(PortName::B, 3).unwrap(),
                Pin::new(PortName::B, 2).unwrap(),
                Pin::new(PortName::B, 1).unwrap(),
                Pin::new(PortName::B, 0).unwrap(),
            ],
        }
    }
}