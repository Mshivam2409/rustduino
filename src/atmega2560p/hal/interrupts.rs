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

//! Global interrupts configured in the ATMEGA2560P chip is controlled here.
//! Section 7.4 of the manual

// Crates required in the code for reading and writing to registers.
use core::ptr::{read_volatile, write_volatile};

/// This contains the registers to be manipulated for controlling global interrupts setup.
/// This represents struct for Globalinterrupts and is used to control sreg register.
#[repr(C, packed)]
pub struct Interrupt {
    pub sreg: u8,
}

impl Interrupt {
    /// Creates a new struct of Global_Interrupts.
    /// # Returns
    /// * `a reference to Interrupt structure` - to control the global interrupt implementations.
    pub unsafe fn new() -> &'static mut Interrupt {
        &mut *(0x5F as *mut Interrupt)
    }

    ///  This fnction Disable global interrupts.
    ///  Also known as CLI.
    pub fn disable(&mut self) {
        let mut ctrl_sreg = unsafe { read_volatile(&self.sreg) };
        ctrl_sreg &= !(1 << 7);
        unsafe {
            write_volatile(&mut self.sreg, ctrl_sreg);
        }
    }

    ///  This function Enable global interrupts.
    ///  Also known as SEI.
    pub fn enable(&mut self) {
        let mut ctrl_sreg = unsafe { read_volatile(&self.sreg) };
        ctrl_sreg |= 1 << 7;
        unsafe {
            write_volatile(&mut self.sreg, ctrl_sreg);
        }
    }
}
