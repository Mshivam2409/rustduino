//  RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Aniket Sharma, Indian Institute of Technology Kanpur
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


use bit_field::BitField;
use core;
use crate::rustduino::hal::interrupts;
use volatile::Volatile;
use crate::rustduino::atmega2560p::com::{usart_initialize,usart_initialize::Usart};

/// to select the data length to be transmitted
#[derive(Clone, Copy)]
pub enum datalen {
    bit5,
    bit6,
    bit7,
    bit8,
    bit9,
}                  

impl Usart{

// initialization

/// set TXEN bit to 1 to enable the Transmitter 
pub fn Transmitter_enable(&mut self) {

        self.ucsrnb.set_bit(3,true);
    
    }               

/// storing in UDR for 5 to 8 bit data
pub fn storing_UDR_5_8 (&self,data: u8,Len: datalen) {

    match Len{

        Len::bit5 =>self.udr.set_bits(0..5, data),
        Len::bit6 =>self.udr.set_bits(0..6, data),
        Len::bit7 =>self.udr.set_bits(0..7, data),
        Len::bit8 =>self.udr.set_bits(0..8, data),

    }
}

/// storing in UDR for 9 bit data
pub fn storing_UDR_9 (&self,data: Volatile<u32>,Len: datalen){

    self.ucsrnb.update(|ctrl| {
            ctrl.set_bit(0, get_bit(&data,8)); // get_bit used here is wrong I will correct it
    });
        let mut i = 0;
    while i < 8 {
        
        self.udr.set_bit(i,get_bit(&data,i));// get_bit used here is wrong I will correct it
        i=i+1;
    }
    
}

// interrupts and Flags 

/// set TXEN bit to 1 to disable the Transmitter
pub fn Transmitter_disable(&mut self) {

    self.ucsrnb.set_bit(3,false);

}      

}