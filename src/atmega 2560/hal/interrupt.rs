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

// Section 7.4 of the manual
// https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf
use core;

// This contains the registers to be manipulated for controlling global interrupts setup.
// Details of SREG register are as follows -  
//         Bit 7 – I: Global Interrupt Enable
//         Bit 6 – T: Bit Copy Storage
//         Bit 5 – H: Half Carry Flag
//         Bit 4 – S: Sign Bit, S = N + V
//         Bit 3 – V: Two’s Complement Overflow Flag
//         Bit 2 – N: Negative Flag
//         Bit 1 – Z: Zero Flag
//         Bit 0 – C: Carry Flag
#[repr(C,packed)]
pub struct Status {
   SREG:u8
}

impl Status {
    pub unsafe fn new() -> &'static mut Status {
        // Return a mutable static reference to a instance of structure  
        &mut *(0x5F as *mut Status) 
    }

    pub fn disable(&mut self) {
        // Create a new instance of Status registers
        Status *ptr = new();
        // Set the global interrupt bit as 0
        let mut sreg = core::ptr::read_volatile(&mut self.SREG);
        sreg = sreg & 0x7F; 
        core::ptr::write_volatile(&mut self.SREG, sreg); 
    }

    pub fn enable(&mut self) {
        // Create a new instance of Status registers
        Status *ptr = new();
        // Set the global interrupt bit as 0
        let mut sreg = core::ptr::read_volatile(&mut self.SREG);
        sreg = sreg | 0x80; 
        core::ptr::write_volatile(&mut self.SREG, sreg); 
    }
}
