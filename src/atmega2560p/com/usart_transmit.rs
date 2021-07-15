//    RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Aniket Sharma, Indian Institute of Technology Kanpur
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


use bit_field::BitField;
use core;
use crate::rustduino::hal::interrupts;
use volatile::Volatile;
use crate::rustduino::atmega2560p::Usart::usart_initialize;

#[derive(Clone, Copy)]
pub enum datalen {
    bit5,
    bit6,
    bit7,
    bit8,
    bit9,
}                  // to select the data lenght to be transmitted



impl Usart{

    

// initialization



pub fn enab_Transmitter() {
    
    
    


      }               // set TXEN bit to 1 to enable the Transmitter 



pub fn send_frame( ) {









// sending the 5-8 data bit 



// if 9 data bit to be send 9th is stored in UCSRnB before low byte of character written to UDRn

}

// interrupts and Flags 

}