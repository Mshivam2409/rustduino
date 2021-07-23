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

//Include the required crates for the code.
use crate::atmega2560p::hal::port::*;

pub struct DigitalPin {
    pub digipin: Pin,
    pub pinno: usize,
}
pub struct DigitalPins {
    pub digital: [DigitalPin; 14],
}
impl DigitalPins {
    pub fn new() -> DigitalPins {
        DigitalPins {
            digital: [
                DigitalPin {
                    digipin: Pin::new(PortName::D, 0).unwrap(),
                    pinno: 0,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::D, 1).unwrap(),
                    pinno: 1,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::D, 2).unwrap(),
                    pinno: 2,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::D, 3).unwrap(),
                    pinno: 3,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::D, 4).unwrap(),
                    pinno: 4,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::D, 5).unwrap(),
                    pinno: 5,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::D, 6).unwrap(),
                    pinno: 6,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::D, 7).unwrap(),
                    pinno: 7,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 0).unwrap(),
                    pinno: 8,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 1).unwrap(),
                    pinno: 9,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 2).unwrap(),
                    pinno: 10,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 3).unwrap(),
                    pinno: 11,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 4).unwrap(),
                    pinno: 12,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 5).unwrap(),
                    pinno: 13,
                },
            ],
        }
    }
}