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

//! This source code creates a array for controlling all digital pins at one place in form
//! Pins array which would be used so that we get meaningful functions to work upon and
//! also the implementation of rustduino library is easier for the user.
//! For more details see section 16,17,25 and 26 of ATMEGA2560P datasheet.
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf

/// Include the required source codes.
pub use crate::atmega2560p::hal::port::*;

/// Structure to represent one digital pin with Pin structure and pin number.
pub struct DigitalPin {
    pub digipin: Pin,
    pub pinno: usize,
}

/// Structure to contain all the digital pins in one place in form of a array.
pub struct DigitalPins {
    pub digital: [DigitalPin; 54],
}

impl DigitalPins {
    /// Create the new array object.
    pub fn new() -> DigitalPins {
        DigitalPins {
            digital: [
                DigitalPin {
                    digipin: Pin::new(PortName::E, 0).unwrap(),
                    pinno: 0,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::E, 1).unwrap(),
                    pinno: 1,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::E, 4).unwrap(),
                    pinno: 2,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::E, 5).unwrap(),
                    pinno: 3,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::G, 5).unwrap(),
                    pinno: 4,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::E, 3).unwrap(),
                    pinno: 5,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::H, 3).unwrap(),
                    pinno: 6,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::H, 4).unwrap(),
                    pinno: 7,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::H, 5).unwrap(),
                    pinno: 8,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::H, 6).unwrap(),
                    pinno: 9,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 4).unwrap(),
                    pinno: 10,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 5).unwrap(),
                    pinno: 11,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 6).unwrap(),
                    pinno: 12,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 7).unwrap(),
                    pinno: 13,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::J, 0).unwrap(),
                    pinno: 14,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::J, 1).unwrap(),
                    pinno: 15,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::H, 1).unwrap(),
                    pinno: 16,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::H, 0).unwrap(),
                    pinno: 17,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::D, 3).unwrap(),
                    pinno: 18,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::D, 2).unwrap(),
                    pinno: 19,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::D, 1).unwrap(),
                    pinno: 20,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::D, 0).unwrap(),
                    pinno: 21,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::A, 0).unwrap(),
                    pinno: 22,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::A, 1).unwrap(),
                    pinno: 23,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::A, 2).unwrap(),
                    pinno: 24,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::A, 3).unwrap(),
                    pinno: 25,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::A, 4).unwrap(),
                    pinno: 26,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::A, 5).unwrap(),
                    pinno: 27,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::A, 6).unwrap(),
                    pinno: 28,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::A, 7).unwrap(),
                    pinno: 29,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::C, 7).unwrap(),
                    pinno: 30,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::C, 6).unwrap(),
                    pinno: 31,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::C, 5).unwrap(),
                    pinno: 32,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::C, 4).unwrap(),
                    pinno: 33,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::C, 3).unwrap(),
                    pinno: 34,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::C, 2).unwrap(),
                    pinno: 35,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::C, 1).unwrap(),
                    pinno: 36,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::C, 0).unwrap(),
                    pinno: 37,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::D, 7).unwrap(),
                    pinno: 38,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::G, 2).unwrap(),
                    pinno: 39,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::G, 1).unwrap(),
                    pinno: 40,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::G, 0).unwrap(),
                    pinno: 41,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::L, 7).unwrap(),
                    pinno: 42,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::L, 6).unwrap(),
                    pinno: 43,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::L, 5).unwrap(),
                    pinno: 44,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::L, 4).unwrap(),
                    pinno: 45,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::L, 3).unwrap(),
                    pinno: 46,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::L, 2).unwrap(),
                    pinno: 47,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::L, 1).unwrap(),
                    pinno: 48,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::L, 0).unwrap(),
                    pinno: 49,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 3).unwrap(),
                    pinno: 50,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 2).unwrap(),
                    pinno: 51,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 1).unwrap(),
                    pinno: 52,
                },
                DigitalPin {
                    digipin: Pin::new(PortName::B, 0).unwrap(),
                    pinno: 53,
                },
            ],
        }
    }
}
