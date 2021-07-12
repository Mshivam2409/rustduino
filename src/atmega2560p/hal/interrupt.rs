//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Devansh Kumar Jha,Indian Institute of Technology Kanpur
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

//! Global interrupts configured in the ATMEGA2560P chip is controlled here.
//! Section 7.4 of the manual
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf

/// Crates required in the code for reading and writing to registers.
use core;

/// This contains the registers to be manipulated for controlling global interrupts setup.
#[repr(C, packed)]
pub struct Status {
    sreg: u8,
}

impl Status {
    /// Return a mutable static reference to a instance of structure Status.
    pub unsafe fn new() -> &'static mut Status {
        &mut *(0x5F as *mut Status)
    }

    /// Set the global interrupt bit as 0.
    pub fn disable(&mut self) {
        unsafe {
            let mut sreg = core::ptr::read_volatile(&mut self.sreg);
            sreg = sreg & 0x7F;
            core::ptr::write_volatile(&mut self.sreg, sreg);
        }
    }

    /// Set the global interrupt bit as 1.
    pub fn enable(&mut self) {
        unsafe {
            let mut sreg = core::ptr::read_volatile(&mut self.sreg);
            sreg = sreg | 0x80;
            core::ptr::write_volatile(&mut self.sreg, sreg);
        }
    }
}