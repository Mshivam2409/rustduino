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

use crate::avr::shift::*;
use crate::delay::delay_ms;

const decpt: bool = true;
const common: char = 'a';

pub struct Pinsegment {
    datapin_no: u8,
    latchpin_no: u8,
    clockpin_no: u8,
}

impl Pinsegment {
    pub fn setup(&mut self) {
        datapin = make_pin(self.datapin_no);
        latchpin = make_pin(self.latchpin_no);
        clockpin = make_pin(self.clockpin_no);

        datapin.set_pin_mode(Output);
        latchpin.set_pin_mode(Output);
        clockpin.set_pin_mode(Output);
    }

    pub fn loop_run(&mut self) {
        decpt = !decpt;
        for i in 0..16 {
            let mut bits: u8 = self.myfnNumToBits(i);
            if (decpt) {
                bits = bits | B00000001;
            }
            self.myfnUpdateDisplay(bits); // display alphanumeric digit
            delay_ms(500); // pause for 1/2 second
        }
    }

    pub fn myfnUpdateDisplay(eightBits: u8) {
        datapin = make_pin(self.datapin_no);
        latchpin = make_pin(self.latchpin_no);
        clockpin = make_pin(self.clockpin_no);

        if (common = 'a') {
            //if using a common anode display
            eightBits = eightBits ^ B11111111; // then flip all bits using XOR
        }
        latchpin.low(); // prepare shift register for data
        shift_out(datapin, clockpin, LSBFIRST, eightBits); // send data
        latchpin.high(); //update display
    }

    pub fn myfnNumToBits(somenumber: int) -> u8 {
        if somenumber == 0 {
            return B11111100;
        } else if somenumber == 1 {
            return B01100000;
        } else if somenumber == 2 {
            return B11011010;
        } else if somenumber == 3 {
            return B11110010;
        } else if somenumber == 4 {
            return B01100110;
        } else if somenumber == 5 {
            return B10110110;
        } else if somenumber == 6 {
            return B10111110;
        } else if somenumber == 7 {
            return B11100000;
        } else if somenumber == 8 {
            return B11111110;
        } else if somenumber == 9 {
            return B11110110;
        } else if somenumber == 10 {
            return B11101110; // 10=='A'
        } else if somenumber == 11 {
            return B00111110; // Hexidecimal B
        } else if somenumber == 12 {
            return B10011100; // Hexidecimal C or use for Centigrade
        } else if somenumber == 13 {
            return B01111010; // Hexidecimal D
        } else if somenumber == 14 {
            return B10011110; // Hexidecimal E
        } else if somenumber == 15 {
            return B10001110; // Hexidecimal F or use for Fahrenhei
        } else {
            return B10010010; // Error condition, displays three vertical bars
        }
    }
}
