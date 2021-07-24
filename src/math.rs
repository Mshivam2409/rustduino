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
use bit_field::BitField;

use crate::delay::delay_ms;
/// Source codes required.
use crate::hal::analogpins::AnalogPins;
use crate::sensors::mpu6050::MPU6050;

/// Structure to control the implementation of Random Number Generators
#[repr(C, packed)]
pub struct RandomNumberGenerator {
    pins: AnalogPins,
    mpu: &'static mut MPU6050<'static>,
}

impl RandomNumberGenerator {
    /// Create a new structure object for Random Number Generation.
    /// This structure contains elements for both type of number generation.
    pub fn new() -> RandomNumberGenerator {
        RandomNumberGenerator {
            pins: AnalogPins::new(),
            mpu: MPU6050::new(),
        }
    }

    /// Generation of random number through random noise in environment
    /// detected by read through analog pins input.
    pub fn generate_by_analog(&mut self) -> u8 {
        let mut bits1: u8 = xor_rotate();

        bits1 = xor_shift(bits1);

        let bits2: u8 = xor_rotate();

        bits1 = xor(bits1, bits2);

        let mut lbuf: u8 = xor_rotate();
        let mut rbuf: u8 = xor_rotate();
        let buf: u8 = xor(lbuf, rbuf);

        let mut bits3: u8 = 0;

        for i in 1..4 {
            delay_ms(100);
            let left: u8 = unsafe { self.pins.analog[0].read() } as u8;

            delay_ms(100);
            let right: u8 = unsafe { self.pins.analog[0].read() } as u8;

            bits3 = xor(bits3, rotate(left, i));
            bits3 = xor(bits3, rotate(right, 7 - i));

            for j in 1..8 {
                let lb = left.get_bit(j);
                let rb = right.get_bit(j);

                if lb != rb {
                    if buf % 2 == 0 {
                        lbuf = push_left(lbuf, lb as u8);
                    } else {
                        rbuf = push_right(rbuf, lb as u8);
                    }
                }
            }
        }

        bits1 = xor_shift(bits1);

        bits1 = xor(bits1, bits3);

        xor(bits1, xor(lbuf, rbuf))
    }

    pub fn generate_by_mpu(&mut self) -> u32 {
        100
    }
}

/// Rotate the unsigned integer of 8 bits by n towards left
/// and surrounding back with the overflowing bits.
pub fn rotate(b: u8, n: u8) -> u8 {
    (b >> n) | (b << (8 - n))
}

/// Get the bitwise XOR (exclusive OR) of two 8 bits unsigned integers.
pub fn xor(a: u8, b: u8) -> u8 {
    (a | b) - (a & b)
}

/// XOR Shift for stability in number generation.
/// It implements one round of XORShift PRNG algorithm for statistical stability.
pub fn xor_shift(a: u8) -> u8 {
    let mut ans: u8 = 0;

    ans = xor(ans, a);
    ans = xor(ans, a >> 3);
    ans = xor(ans, a << 5);
    ans = xor(ans, a >> 4);

    ans
}

/// Generate XOR Rotation number.
pub fn xor_rotate() -> u8 {
    let mut bits1: u8 = 0;
    let mut obj = RandomNumberGenerator::new();

    for i in 1..8 {
        let a: u8 = unsafe { obj.pins.analog[0].read() } as u8;
        bits1 = xor(bits1, rotate(a, i));
        delay_ms(100);
    }

    bits1
}

/// Push the required bit with left bias.
pub fn push_left(val: u8, change: u8) -> u8 {
    xor(val << 1, xor(change, val))
}

/// Push the required bit with right bias.
pub fn push_right(val: u8, change: u8) -> u8 {
    xor(val >> 1, xor(change << 7, val))
}
