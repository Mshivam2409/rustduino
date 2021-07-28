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

use core::ptr::{read_volatile, write_volatile};

/// SREG (Status control Register)
/// The status register contains information about the result of the most recently executed arithmetic instruction. This
/// information can be used for altering program flow in order to perform conditional operations. Note that the status register is
/// updated after all ALU operations, as specified in the instruction set reference. This will in many cases remove the need for
/// using the dedicated compare instructions, resulting in faster and more compact code.
/// The status register is not automatically stored when entering an interrupt routine and restored when returning from an
/// interrupt. This must be handled by software.
///
/// Toggling 8th bit to 0 or 1 can enable or disable interrupt respectively.
#[repr(C, packed)]
pub struct Interrupt {
    sreg: u8,
}

impl Interrupt {
    /// Creates a new struct of Global_Interrupts.
    /// # Returns
    /// * `a reference to Interrupt structure` - to control the global interrupt implementations.
    pub unsafe fn new() -> &'static mut Interrupt {
        &mut *(0x5F as *mut Interrupt)
    }

    /// Disables Interrupts.
    pub fn disable(&mut self) {
        unsafe {
            let mut ctrl_sreg = read_volatile(&self.sreg);
            ctrl_sreg &= 0x7F;
            write_volatile(&mut self.sreg, ctrl_sreg);
        }
    }

    /// Enables Interrupts
    pub fn enable(&mut self) {
        unsafe {
            let mut ctrl_sreg = read_volatile(&self.sreg);
            ctrl_sreg &= 0xFF;
            write_volatile(&mut self.sreg, ctrl_sreg);
        }
    }
}
