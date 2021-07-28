//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Saurabh Singh,Indian Institute of Technology Kanpur
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

//! Pins implementation for ATMEGA238P where all pins are packed in a single structure.
//! Section 13.2.1 and 13.2.2 of ATmega328P datasheet.

use crate::atmega328p::hal::port::*;

/// All pins inside a single struct.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Pins {
    /// All six analog pins.
    pub analog: [AnalogPin; 6],

    /// All 14 digital I/O pins.
    pub digital: [DigitalPin; 14],
}

/// This struct contains the Pin struct and its analog pin number.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct AnalogPin {
    pub pin: Pin,
    pub pinno: u32,
}

/// Structure to represent one digital pin with Pin structure and pin number.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct DigitalPin {
    pub pin: Pin,
    pub pinno: usize,
}

impl Pins {
    /// Returns all pins at once as a single struct.
    /// No new memory is created, just the already created space is given
    /// a name so it is a memory mapped I/O.
    /// # Returns
    /// * `a Pins object` - used to control all pins of AVR chip at one place.
    pub fn new() -> Pins {
        Pins {
            analog: [
                AnalogPin {
                    pin: Pin::new(PortName::C, 0).unwrap(),
                    pinno: 0,
                },
                AnalogPin {
                    pin: Pin::new(PortName::C, 1).unwrap(),
                    pinno: 1,
                },
                AnalogPin {
                    pin: Pin::new(PortName::C, 2).unwrap(),
                    pinno: 2,
                },
                AnalogPin {
                    pin: Pin::new(PortName::C, 3).unwrap(),
                    pinno: 3,
                },
                AnalogPin {
                    pin: Pin::new(PortName::C, 4).unwrap(),
                    pinno: 4,
                },
                AnalogPin {
                    pin: Pin::new(PortName::C, 5).unwrap(),
                    pinno: 5,
                },
            ],
            digital: [
                DigitalPin {
                    pin: Pin::new(PortName::D, 0).unwrap(),
                    pinno: 0,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 1).unwrap(),
                    pinno: 1,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 2).unwrap(),
                    pinno: 2,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 3).unwrap(),
                    pinno: 3,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 4).unwrap(),
                    pinno: 4,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 5).unwrap(),
                    pinno: 5,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 6).unwrap(),
                    pinno: 6,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 7).unwrap(),
                    pinno: 7,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 0).unwrap(),
                    pinno: 0,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 1).unwrap(),
                    pinno: 1,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 2).unwrap(),
                    pinno: 2,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 3).unwrap(),
                    pinno: 3,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 4).unwrap(),
                    pinno: 4,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 5).unwrap(),
                    pinno: 5,
                },
            ],
        }
    }
}

/// This function returns digital pin corresponding to it's number.
/// # Arguments
/// * `a u32` - The pin number which is to be used.
/// # Returns
/// * `a Pin object` - The memory mapped I/O object to control the Digital Pin.
fn _make_pin(pin: u8) -> Pin {
    match pin {
        0 => return Pin::new(PortName::D, 0).unwrap(),
        1 => return Pin::new(PortName::D, 1).unwrap(),
        2 => return Pin::new(PortName::D, 2).unwrap(),
        3 => return Pin::new(PortName::D, 3).unwrap(),
        4 => return Pin::new(PortName::D, 4).unwrap(),
        5 => return Pin::new(PortName::D, 5).unwrap(),
        6 => return Pin::new(PortName::D, 6).unwrap(),
        7 => return Pin::new(PortName::D, 7).unwrap(),

        8 => return Pin::new(PortName::B, 8).unwrap(),
        9 => return Pin::new(PortName::B, 9).unwrap(),
        10 => return Pin::new(PortName::B, 10).unwrap(),
        11 => return Pin::new(PortName::B, 11).unwrap(),
        12 => return Pin::new(PortName::B, 12).unwrap(),
        13 => return Pin::new(PortName::B, 13).unwrap(),

        _ => unreachable!(),
    }
}
