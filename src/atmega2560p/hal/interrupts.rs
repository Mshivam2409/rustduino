/*RustDuino : A generic HAL implementation for Arduino Boards in Rust
Copyright (C) 2021  Nikhil Gupta,

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>*//


use core;
///struct for global interrupts
pub struct GlobalInterrupts {  
   sreg:u8,                 
}
///in section 2-(Overview) point 7.4 about (SREG) 

impl GlobalInterrupts {
    ///returns new Global_Interrupts
    pub unsafe fn new() -> &'static mut GlobalInterrupts {
        &mut *(0x5F as *mut Global_Interrupts) 
    }

    ///disable global interrupts
    ///also known as CLI
    ///sets I-bit of SREG 0
    pub fn disable(&mut self) {
        unsafe {
           let mut ctrl_sreg=core::ptr::read_volatile(&self.sreg);
           ctrl_sreg &= ~(1<<7);                                   
           core::ptr::write_volatile(&mut self.sreg,ctrl_sreg);
        }
    }

    ///sets I_bit of SREG 1
    ///enable global interrupts
    ///also known as SEI
    pub fn enable(&mut self){
        unsafe{
           let mut ctrl_sreg=core::ptr::read_volatile(&self.sreg);
           ctrl_sreg |=(1<<7);                                        
           core::ptr::write_volatile(&mut self.sreg,ctrl_sreg);
        }
    }
}