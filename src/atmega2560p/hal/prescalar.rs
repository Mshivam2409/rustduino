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

//! System clock and power use control of the power in ATMEGA2560P using prescalar
//! Section 10.13 of the manual
//! Also references from Section 10.12 and 10.7
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf


/// Crates required in the code for reading and writing to registers.
/// Interrupts would be used for disabling global interrupts which may create problem while execution.
use crate::atmega2560p::hal::interrupt;
use core;

/// The below structure controls the clock prescalar register of the chip.
/// This will be used to control clock pre-scalar settings.
#[repr(C, packed)]
pub struct Prescalar {
    clkpr: u8,
    pad_1: [char; 4], // Appropriate padding
    osccal: u8,
}

impl Prescalar {
    /// Creates a mutable reference to the structure to control the system clock configuration.
    pub unsafe fn new() -> &'static mut Prescalar {
        &mut *(0x61 as *mut Prescalar)
    }

    /// System Clock Pre-scalar register are controlled for clock gating.
    /// Write the Clock Prescaler Change Enable (CLKPCE) bit to one and all other bits in CLKPR to zero.
    /// Within four cycles, write the desired value to CLKPS bits while writing a zero to CLKPCE.
    /// Interrupts must be disabled when changing prescaler setting to make sure the write procedure is not interrupted.
    /// The clock division factor is set to desired value.
    /// Only certain values are allowed for the user.
    pub fn enable_clock(&mut self, freq: u32) {
        unsafe {
            let itr = interrupt::Status::new(); // Object to control interrupts
            itr.disable(); // Global interrupts are disabled

            core::ptr::write_volatile(&mut self.clkpr, 0x80);

            if freq == 1 {
                core::ptr::write_volatile(&mut self.clkpr, 0x00);
            } else if freq == 2 {
                core::ptr::write_volatile(&mut self.clkpr, 0x01);
            } else if freq == 4 {
                core::ptr::write_volatile(&mut self.clkpr, 0x02);
            } else if freq == 8 {
                core::ptr::write_volatile(&mut self.clkpr, 0x03);
            } else if freq == 16 {
                core::ptr::write_volatile(&mut self.clkpr, 0x04);
            } else if freq == 32 {
                core::ptr::write_volatile(&mut self.clkpr, 0x05);
            } else if freq == 64 {
                core::ptr::write_volatile(&mut self.clkpr, 0x06);
            } else if freq == 128 {
                core::ptr::write_volatile(&mut self.clkpr, 0x07);
            } else if freq == 256 {
                core::ptr::write_volatile(&mut self.clkpr, 0x08);
            } else {
                unreachable!()
            }

            itr.enable(); // Enable global interrupts
        }
    }
}