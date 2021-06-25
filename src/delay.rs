// RustDuino : A generic HAL implementation for Arduino Boards in Rust
// Copyright (C) 2021 Shivam Malhotra, Indian Institute of Technology Kanpur

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

#[inline(always)]
/// Internal function to implement a variable busy-wait loop.
/// # Arguments
/// * 'count' - an i32, the number of times to cycle the loop.
pub fn delay(count: u32) {
    // Our asm busy-wait takes a 16 bit word as an argument,
    // so the max number of loops is 2^16
    let outer_count = count / 65536;
    let last_count = ((count % 65536) + 1) as u16;
    for _ in 0..outer_count {
        // Each loop through should be 4 cycles.
        unsafe {
            llvm_asm!("1: sbiw $0,1
                      brne 1b"
                     :
                     : "w" (0)
                     :
                     :)
        }
    }
    unsafe {
        llvm_asm!("1: sbiw $0,1
                      brne 1b"
                 :
                 : "w" (last_count)
                 :
                 :)
    }
}

///delay for N seconds
/// # Arguments
/// * 's' - an u32, number of seconds to busy-wait
#[inline(always)]
pub fn delay_s(s: u32) {
    // microseconds
    let ms = s * 1000;
    delay_ms(ms);
}

///delay for N miliseconds
/// # Arguments
/// * 'ms' - an u32, number of milliseconds to busy-wait
#[inline(always)]
pub fn delay_ms(ms: u32) {
    // microseconds
    let us = ms * 1000;
    delay_us(us);
}

///delay for N microseconds
/// # Arguments
/// * 'ms' - an u32, number of microseconds to busy-wait
#[inline(always)]
pub fn delay_us(us: u32) {
    // nanoseconds
    let ns = us * 1000;
    let ns_lp = 1000000000 / (crate::config::CPU_FREQUENCY_HZ / 4);
    let loops = (ns / ns_lp) as u32;
    delay(loops);
}
