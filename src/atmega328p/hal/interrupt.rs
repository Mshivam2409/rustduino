// RustDuino : A generic HAL implementation for Arduino Boards in Rust
// Copyright (C) 2021  Askhit Verma, Indian Institute of Technology Kanpur

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

use core;
// #[no(main)]

/// section 6.3.1 of manual
pub struct Interrupt {
    sreg: u8,
}

impl Interrupt {
    pub unsafe fn new() -> &'static mut Interrupt {
        &mut *(0x5F as *mut Interrupt)
    }

    /// Getting address of the sreg register.
    /// Reading value stored in sreg.\
    /// Toggling 8th bit to 0; 7F in binary is 01111111.
    /// Writing value back in sreg
    pub fn disable(&mut self) {
        unsafe {
            // let mut interrupt = get();
            let mut ctrl_sreg = core::ptr::read_volatile(&self.sreg);
            ctrl_sreg &= 0x7F;
            core::ptr::write_volatile(&mut self.sreg, ctrl_sreg);
        }
    }

    /// Getting address of the sreg register.
    /// Reading value stored in sreg
    /// Toggling 8th bit to 1; 7F in binary is 11111111
    /// Writing value back in sreg
    pub fn enable(&mut self) {
        unsafe {
            // let mut interrupt = get();
            let mut ctrl_sreg = core::ptr::read_volatile(&self.sreg);
            ctrl_sreg &= 0xFF;
            core::ptr::write_volatile(&mut self.sreg, ctrl_sreg);
        }
    }
}
