//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Devansh Kumar Jha, Indian Institute of Technology Kanpur
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


/// Crates which would be used in the implementation.
use bit_field::BitField;
use core;
use volatile::Volatile;
use crate::rustduino::hal::interrupts;

/// This structure contains various registers needed to control usart communication
/// through ATMEGA2560P device.
pub enum USARTName{
    USART0,
    USART1,
    USART2,
    USART3,
}
pub struct Usart {
     ucsra:Volatile<u8>,
     ucsrb:Volatile<u8>,
     ucsrc:Volatile<u8>,
     pad_:Volatile<u8>,
     ubrrl:Volatile<u8>,
     ubrrh:Volatile<u8>,
     udr:Volatile<u8>,
}

impl Usart {
    pub unsafe fn new(name:USARTName) -> &'static mut Usart {
       match name{
           USARTName::USART0 =>{&mut *(0xC0 as *mut Usart)}
           USARTName::USART1 =>{&mut *(0xC8 as *mut Usart)}
           USARTName::USART2 =>{&mut *(0xD0 as *mut Usart)}
           USARTName::USART3 =>{&mut *(0x130 as *mut Usart)}
       }
    }
   
}