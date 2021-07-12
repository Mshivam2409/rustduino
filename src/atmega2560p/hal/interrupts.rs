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
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf

use core;

///This contains the registers to be manipulated for controlling global interrupts setup.
///This represents struct for Globalinterrupts and is used to control sreg register.
pub struct GlobalInterrupts {
    sreg: u8,
}
///In section 7.4 about (SREG).
impl GlobalInterrupts {
    ///Returns new struct of Global_Interrupts.
    pub unsafe fn new() -> &'static mut GlobalInterrupts {
        &mut *(0x5F as *mut GlobalInterrupts)
    }

    ///This fnction Disable global interrupts.
    ///Also known as CLI.
    pub fn disable(&mut self) {
        unsafe {
            let mut ctrl_sreg = core::ptr::read_volatile(&self.sreg);
            ctrl_sreg &= !(1 << 7);
            core::ptr::write_volatile(&mut self.sreg, ctrl_sreg);
        }
    }

    ///This function Enable global interrupts.
    ///Also known as SEI.
    pub fn enable(&mut self) {
        unsafe {
            let mut ctrl_sreg = core::ptr::read_volatile(&self.sreg);
            ctrl_sreg |= 1 << 7;
            core::ptr::write_volatile(&mut self.sreg, ctrl_sreg);
        }
    }
}
