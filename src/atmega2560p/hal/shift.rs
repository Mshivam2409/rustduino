// RustDuino : A generic HAL implementation for Arduino Boards in Rust
// Copyright (C) 2021  Akshit Verma, Indian Institute of Technology Kanpur

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>

use crate::atmega2560p::hal::pin::Pins;
use core::usize;

/// Enum for bit order of the value.
pub enum BitOrder {
    LSBFIRST,
    MSBFIRST,
}

/// It sets the shift register according to the input from the user.
/// # Arguments
/// * `datapin` - a usize, containing the number of the digital pin from which data will be read.
/// * `clockpin` - a usize, containing the number of the digital pin from which clock source will be adjusted.
/// * `bit_order` - a `BitOrder` object, to specify the order of bits in the shift register.
/// # Returns
/// * `a u8` - The value stored in the shift register.
pub fn shift_in(datapin: usize, clockpin: usize, bit_order: BitOrder) -> u8 {
    let mut value: u8 = 0;
    let mut i: u8 = 0;
    let pins = Pins::new();
    let mut data = pins.digital[datapin];
    let mut clock = pins.digital[clockpin];
    loop {
        clock.high();

        match bit_order {
            BitOrder::LSBFIRST => value |= data.read() << i,
            BitOrder::MSBFIRST => value |= data.read() << (7 - i),
        }

        clock.low();

        i += 1;
        if i == 7 {
            return value;
        }
    }
}

/// Stores value in the Shift Register.
/// # Arguments
/// * `datapin` - a usize, containing the number of the digital pin from which data will be read.
/// * `clockpin` - a usize, containing the number of the digital pin from which clock source will be adjusted.
/// * `bit_order` - a `BitOrder` object, to specify the order of bits in the shift register.
/// * `value` - a mutable u8, which will store the value which is to be written.
pub fn shift_out(datapin: usize, clockpin: usize, bit_order: BitOrder, mut value: u8) {
    let mut i: u8 = 0;
    let pins = Pins::new();
    let mut data = pins.digital[datapin];
    let mut clock = pins.digital[clockpin];

    loop {
        match bit_order {
            BitOrder::LSBFIRST => {
                if value & 1 == 1 {
                    data.high();
                } else {
                    data.low();
                }
                value >>= 1;
            }

            BitOrder::MSBFIRST => {
                if value & 128 != 0 {
                    data.high();
                } else {
                    data.low();
                }
                value <<= 1;
            }
        }
        clock.high();
        clock.low();

        i += 1;
        if i == 7 {
            return;
        }
    }
}
