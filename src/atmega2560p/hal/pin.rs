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

///  The ATMEGA2560P microcontroller IC has a total of 100 pins to configure the functioning of the
///  microcontroller. Out of those 86 pins are set as I/O pins which are configured into 11 ports each controlling
///  8 pins except port G which controls 6 pins. All 8 pins of port F and K are Analog pins and total 54 digital pins
///  are available and the rest 16 pins are for various other purposes.
///  This structure declaration contains the space to control all the 86 pins in one memory mapped I/O.
#[derive(Clone, Copy)]
pub struct Pins {
    /// All 16 analog pins.
    pub analog: [AnalogPin; 16],
    /// All 54 digital I/O pins.
    pub digital: [DigitalPin; 54],
}

/// This struct contain digital pin and its corresponding digital pin no.
#[derive(Clone, Copy)]
pub struct DigitalPin {
    pub pin: Pin,
    pub pinno: u32,
}

/// This struct contain analog pin and its corresponding analog pin no.
#[derive(Clone, Copy)]
pub struct AnalogPin {
    pub pin: Pin,
    pub pinno: u32,
}

impl Pins {
    /// Returns all pins at once as a single struct.
    /// No new memory is created, just the already created space is given
    /// a name so it is a memory mapped I/O.

    pub fn new() -> Pins {
        Pins {
            analog: [
                AnalogPin {
                    pin: Pin::new(PortName::F, 0).unwrap(),
                    pinno: 0,
                },
                AnalogPin {
                    pin: Pin::new(PortName::F, 1).unwrap(),
                    pinno: 1,
                },
                AnalogPin {
                    pin: Pin::new(PortName::F, 2).unwrap(),
                    pinno: 2,
                },
                AnalogPin {
                    pin: Pin::new(PortName::F, 3).unwrap(),
                    pinno: 3,
                },
                AnalogPin {
                    pin: Pin::new(PortName::F, 4).unwrap(),
                    pinno: 4,
                },
                AnalogPin {
                    pin: Pin::new(PortName::F, 5).unwrap(),
                    pinno: 5,
                },
                AnalogPin {
                    pin: Pin::new(PortName::F, 6).unwrap(),
                    pinno: 6,
                },
                AnalogPin {
                    pin: Pin::new(PortName::F, 7).unwrap(),
                    pinno: 7,
                },
                AnalogPin {
                    pin: Pin::new(PortName::K, 0).unwrap(),
                    pinno: 8,
                },
                AnalogPin {
                    pin: Pin::new(PortName::K, 1).unwrap(),
                    pinno: 9,
                },
                AnalogPin {
                    pin: Pin::new(PortName::K, 2).unwrap(),
                    pinno: 10,
                },
                AnalogPin {
                    pin: Pin::new(PortName::K, 3).unwrap(),
                    pinno: 11,
                },
                AnalogPin {
                    pin: Pin::new(PortName::K, 4).unwrap(),
                    pinno: 12,
                },
                AnalogPin {
                    pin: Pin::new(PortName::K, 5).unwrap(),
                    pinno: 13,
                },
                AnalogPin {
                    pin: Pin::new(PortName::K, 6).unwrap(),
                    pinno: 14,
                },
                AnalogPin {
                    pin: Pin::new(PortName::K, 7).unwrap(),
                    pinno: 15,
                },
            ],
            digital: [
                DigitalPin {
                    pin: Pin::new(PortName::E, 0).unwrap(),
                    pinno: 0,
                },
                DigitalPin {
                    pin: Pin::new(PortName::E, 1).unwrap(),
                    pinno: 1,
                },
                DigitalPin {
                    pin: Pin::new(PortName::E, 4).unwrap(),
                    pinno: 2,
                },
                DigitalPin {
                    pin: Pin::new(PortName::E, 5).unwrap(),
                    pinno: 3,
                },
                DigitalPin {
                    pin: Pin::new(PortName::G, 5).unwrap(),
                    pinno: 4,
                },
                DigitalPin {
                    pin: Pin::new(PortName::E, 3).unwrap(),
                    pinno: 5,
                },
                DigitalPin {
                    pin: Pin::new(PortName::H, 3).unwrap(),
                    pinno: 6,
                },
                DigitalPin {
                    pin: Pin::new(PortName::H, 4).unwrap(),
                    pinno: 7,
                },
                DigitalPin {
                    pin: Pin::new(PortName::H, 5).unwrap(),
                    pinno: 8,
                },
                DigitalPin {
                    pin: Pin::new(PortName::H, 6).unwrap(),
                    pinno: 9,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 4).unwrap(),
                    pinno: 10,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 5).unwrap(),
                    pinno: 11,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 6).unwrap(),
                    pinno: 12,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 7).unwrap(),
                    pinno: 13,
                },
                DigitalPin {
                    pin: Pin::new(PortName::J, 0).unwrap(),
                    pinno: 14,
                },
                DigitalPin {
                    pin: Pin::new(PortName::J, 1).unwrap(),
                    pinno: 15,
                },
                DigitalPin {
                    pin: Pin::new(PortName::H, 1).unwrap(),
                    pinno: 16,
                },
                DigitalPin {
                    pin: Pin::new(PortName::H, 0).unwrap(),
                    pinno: 17,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 3).unwrap(),
                    pinno: 18,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 2).unwrap(),
                    pinno: 19,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 1).unwrap(),
                    pinno: 20,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 0).unwrap(),
                    pinno: 21,
                },
                DigitalPin {
                    pin: Pin::new(PortName::A, 0).unwrap(),
                    pinno: 22,
                },
                DigitalPin {
                    pin: Pin::new(PortName::A, 1).unwrap(),
                    pinno: 23,
                },
                DigitalPin {
                    pin: Pin::new(PortName::A, 2).unwrap(),
                    pinno: 24,
                },
                DigitalPin {
                    pin: Pin::new(PortName::A, 3).unwrap(),
                    pinno: 25,
                },
                DigitalPin {
                    pin: Pin::new(PortName::A, 4).unwrap(),
                    pinno: 26,
                },
                DigitalPin {
                    pin: Pin::new(PortName::A, 5).unwrap(),
                    pinno: 27,
                },
                DigitalPin {
                    pin: Pin::new(PortName::A, 6).unwrap(),
                    pinno: 28,
                },
                DigitalPin {
                    pin: Pin::new(PortName::A, 7).unwrap(),
                    pinno: 29,
                },
                DigitalPin {
                    pin: Pin::new(PortName::C, 7).unwrap(),
                    pinno: 30,
                },
                DigitalPin {
                    pin: Pin::new(PortName::C, 6).unwrap(),
                    pinno: 31,
                },
                DigitalPin {
                    pin: Pin::new(PortName::C, 5).unwrap(),
                    pinno: 32,
                },
                DigitalPin {
                    pin: Pin::new(PortName::C, 4).unwrap(),
                    pinno: 33,
                },
                DigitalPin {
                    pin: Pin::new(PortName::C, 3).unwrap(),
                    pinno: 34,
                },
                DigitalPin {
                    pin: Pin::new(PortName::C, 2).unwrap(),
                    pinno: 35,
                },
                DigitalPin {
                    pin: Pin::new(PortName::C, 1).unwrap(),
                    pinno: 36,
                },
                DigitalPin {
                    pin: Pin::new(PortName::C, 0).unwrap(),
                    pinno: 37,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 7).unwrap(),
                    pinno: 38,
                },
                DigitalPin {
                    pin: Pin::new(PortName::G, 2).unwrap(),
                    pinno: 39,
                },
                DigitalPin {
                    pin: Pin::new(PortName::G, 1).unwrap(),
                    pinno: 40,
                },
                DigitalPin {
                    pin: Pin::new(PortName::G, 0).unwrap(),
                    pinno: 41,
                },
                DigitalPin {
                    pin: Pin::new(PortName::L, 7).unwrap(),
                    pinno: 42,
                },
                DigitalPin {
                    pin: Pin::new(PortName::L, 6).unwrap(),
                    pinno: 43,
                },
                DigitalPin {
                    pin: Pin::new(PortName::L, 5).unwrap(),
                    pinno: 44,
                },
                DigitalPin {
                    pin: Pin::new(PortName::L, 4).unwrap(),
                    pinno: 45,
                },
                DigitalPin {
                    pin: Pin::new(PortName::L, 3).unwrap(),
                    pinno: 46,
                },
                DigitalPin {
                    pin: Pin::new(PortName::L, 2).unwrap(),
                    pinno: 47,
                },
                DigitalPin {
                    pin: Pin::new(PortName::L, 1).unwrap(),
                    pinno: 48,
                },
                DigitalPin {
                    pin: Pin::new(PortName::L, 0).unwrap(),
                    pinno: 49,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 3).unwrap(),
                    pinno: 50,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 2).unwrap(),
                    pinno: 51,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 1).unwrap(),
                    pinno: 52,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 0).unwrap(),
                    pinno: 53,
                },
            ],
        }
    }
}

///This function returns digital pin corresponding to its number.
pub fn make_pin(pin: u32) -> Pin {
    match pin {
        0 => return Pin::new(PortName::E, 0).unwrap(),
        1 => return Pin::new(PortName::E, 1).unwrap(),
        2 => return Pin::new(PortName::E, 4).unwrap(),
        3 => return Pin::new(PortName::E, 5).unwrap(),
        4 => return Pin::new(PortName::G, 5).unwrap(),
        5 => return Pin::new(PortName::E, 3).unwrap(),
        6 => return Pin::new(PortName::H, 3).unwrap(),
        7 => return Pin::new(PortName::H, 4).unwrap(),
        8 => return Pin::new(PortName::H, 5).unwrap(),
        9 => return Pin::new(PortName::H, 6).unwrap(),
        10 => return Pin::new(PortName::B, 4).unwrap(),
        11 => return Pin::new(PortName::B, 5).unwrap(),
        12 => return Pin::new(PortName::B, 6).unwrap(),
        13 => return Pin::new(PortName::B, 7).unwrap(),
        14 => return Pin::new(PortName::J, 0).unwrap(),
        15 => return Pin::new(PortName::J, 1).unwrap(),
        16 => return Pin::new(PortName::H, 1).unwrap(),
        17 => return Pin::new(PortName::H, 0).unwrap(),
        18 => return Pin::new(PortName::D, 3).unwrap(),
        19 => return Pin::new(PortName::D, 2).unwrap(),
        20 => return Pin::new(PortName::D, 1).unwrap(),
        21 => return Pin::new(PortName::D, 0).unwrap(),
        22 => return Pin::new(PortName::A, 0).unwrap(),
        23 => return Pin::new(PortName::A, 1).unwrap(),
        24 => return Pin::new(PortName::A, 2).unwrap(),
        25 => return Pin::new(PortName::A, 3).unwrap(),
        26 => return Pin::new(PortName::A, 4).unwrap(),
        27 => return Pin::new(PortName::A, 5).unwrap(),
        28 => return Pin::new(PortName::A, 6).unwrap(),
        29 => return Pin::new(PortName::A, 7).unwrap(),
        30 => return Pin::new(PortName::C, 7).unwrap(),
        31 => return Pin::new(PortName::C, 6).unwrap(),
        32 => return Pin::new(PortName::C, 5).unwrap(),
        33 => return Pin::new(PortName::C, 4).unwrap(),
        34 => return Pin::new(PortName::C, 3).unwrap(),
        35 => return Pin::new(PortName::C, 2).unwrap(),
        36 => return Pin::new(PortName::C, 1).unwrap(),
        37 => return Pin::new(PortName::C, 0).unwrap(),
        38 => return Pin::new(PortName::D, 7).unwrap(),
        39 => return Pin::new(PortName::G, 2).unwrap(),
        40 => return Pin::new(PortName::G, 1).unwrap(),
        41 => return Pin::new(PortName::G, 0).unwrap(),
        42 => return Pin::new(PortName::L, 7).unwrap(),
        43 => return Pin::new(PortName::L, 6).unwrap(),
        44 => return Pin::new(PortName::L, 5).unwrap(),
        45 => return Pin::new(PortName::L, 4).unwrap(),
        46 => return Pin::new(PortName::L, 3).unwrap(),
        47 => return Pin::new(PortName::L, 2).unwrap(),
        48 => return Pin::new(PortName::L, 1).unwrap(),
        49 => return Pin::new(PortName::L, 0).unwrap(),
        50 => return Pin::new(PortName::B, 3).unwrap(),
        51 => return Pin::new(PortName::B, 2).unwrap(),
        52 => return Pin::new(PortName::B, 1).unwrap(),
        53 => return Pin::new(PortName::B, 0).unwrap(),
        _ => unreachable!(),
    }
}
