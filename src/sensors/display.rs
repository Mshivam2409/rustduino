//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Sahil Aggarwal, Indian Institute of Technology Kanpur
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

use crate::hal::pin::Pins;
use crate::hal::shift::*;
use core::usize;

/// Setup for the 7-Segment Display.
/// # Arguments
/// * `datapin` - a usize, to select the number of the digital pin to be used for 7-segment display data.
/// * `clockpin` - a usize, to select the number of the digital pin to be used for clock setup.
/// * `latchpin` - a usize, to select the number of the digital pin to be used for setup of latch adjustments.
/// * `decpt` - a boolean, to check the decrypting value for display.
/// * `common_anode` - a boolean, to set the common anode for the display.
/// * `value` - a u8, to select the value which is to be displayed.
pub fn setup(
    datapin: usize,
    clockpin: usize,
    latchpin: usize,
    decpt: bool,
    common_anode: bool,
    value: u8,
) {
    let pins = Pins::new();
    let mut data = pins.digital[datapin];
    let mut clock = pins.digital[clockpin];
    let mut latch = pins.digital[latchpin];

    data.set_output();
    latch.set_output();
    clock.set_output();

    out(datapin, clockpin, latchpin, decpt, common_anode, value);
}

/// Display the required data on the display screen.
/// # Arguments
/// * `datapin` - a usize, to select the number of the digital pin to be used for 7-segment display data.
/// * `clockpin` - a usize, to select the number of the digital pin to be used for clock setup.
/// * `latchpin` - a usize, to select the number of the digital pin to be used for setup of latch adjustments.
/// * `decpt` - a boolean, to check the decrypting value for display.
/// * `common_anode` - a boolean, to set the common anode for the display.
/// * `value` - a u8, to select the value which is to be displayed.
pub fn myfn_update_display(
    datapin: usize,
    clockpin: usize,
    latchpin: usize,
    _decpt: bool,
    common_anode: bool,
    mut value: u8,
) {
    let pins = Pins::new();
    let mut latch = pins.digital[latchpin];

    if common_anode {
        //if using a common anode display
        value = value ^ 0b11111111; // then flip all bits using XOR
    }
    latch.low(); // prepare shift register for data
    shift_out(datapin, clockpin, BitOrder::LSBFIRST, value); // send data
    latch.high(); //update display
}

/// Set the appropriate registers for output.
/// # Arguments
/// * `datapin` - a usize, to select the number of the digital pin to be used for 7-segment display data.
/// * `clockpin` - a usize, to select the number of the digital pin to be used for clock setup.
/// * `latchpin` - a usize, to select the number of the digital pin to be used for setup of latch adjustments.
/// * `decpt` - a boolean, to check the decrypting value for display.
/// * `common_anode` - a boolean, to set the common anode for the display.
/// * `value` - a u8, to select the value which is to be displayed.
pub fn out(
    datapin: usize,
    clockpin: usize,
    latchpin: usize,
    decpt: bool,
    common_anode: bool,
    value: u8,
) {
    let mut bits: u8 = myfn_num_to_bits(value);
    if decpt {
        bits = bits | 0b00000001; // add decimal point if needed
    }
    myfn_update_display(datapin, clockpin, latchpin, decpt, common_anode, bits);
    // display alphanumeric digit
}

pub fn myfn_num_to_bits(somenumber: u8) -> u8 {
    if somenumber == 0 {
        return 0b11111100;
    } else if somenumber == 1 {
        return 0b01100000;
    } else if somenumber == 2 {
        return 0b11011010;
    } else if somenumber == 3 {
        return 0b11110010;
    } else if somenumber == 4 {
        return 0b01100110;
    } else if somenumber == 5 {
        return 0b10110110;
    } else if somenumber == 6 {
        return 0b10111110;
    } else if somenumber == 7 {
        return 0b11100000;
    } else if somenumber == 8 {
        return 0b11111110;
    } else if somenumber == 9 {
        return 0b11110110;
    } else if somenumber == 10 {
        return 0b11101110; //10=='A'
    } else if somenumber == 11 {
        return 0b00111110; // Hexidecimal B
    } else if somenumber == 12 {
        return 0b10011100; // Hexidecimal C or use for Centigrade
    } else if somenumber == 13 {
        return 0b01111010; // Hexidecimal D
    } else if somenumber == 14 {
        return 0b10011110; // Hexidecimal E
    } else if somenumber == 15 {
        return 0b10001110; // Hexidecimal F or use for Fahrenhei
    } else {
        return 0b10010010; // Error condition, displays three vertical bars
    }
}
