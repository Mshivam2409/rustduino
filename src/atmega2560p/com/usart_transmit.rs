//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
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

    // initialization setting begin function 

    ///This function is to enable the Transmitter 
    pub fn Transmitter_enable(&mut self) {
        unsafe {

            self.ucsrnb.set_bit(3,true);

        }               
    }  

    /// storing data in Transmit Buffer it takes parameter as a u32 and anddata bit length
    pub fn Transmitting_data (&self,data: Volatile<u32>,Len: datalen) {
        unsafe{
            let mut ucsrna = self.ucsrna ;
            let mut udren = ucsrna.get_bit(6);
            
            /// checks if the Transmit buffer is empty to receive data
            while ( !( ucsrna & (1<<udren))) {};

            match Len{

                Len::bit5 =>self.udr.set_bits(0..5, data.get_bits(0..5)),
                Len::bit6 =>self.udr.set_bits(0..6, data.get_bits(0..6)),
                Len::bit7 =>self.udr.set_bits(0..7, data.get_bits(0..7)),
                Len::bit8 =>self.udr.set_bits(0..8, data.get_bits(0..8)),
                Len::bit9 =>{

                    self.ucsrnb.update(|ctrl| {
                        ctrl.set_bit(0, data.get_bit(8)); 
                    });
                    
                    self.udr.set_bits(0..8,data.get_bits(0..8));

                }
            }
        }
    }  

    ///This function tells if you can write in transmit buffer or not by checking UDREn
    pub fn avai_write(&mut self) -> bool{
        
        unsafe {
            let mut ucsrc =read_volatile(&self.ucsrc);
            
            if ucsrc.get_bit(5) {
                true
            }
            else {
                false
            }
        }
    }

    // interrupts and Flags 
    
    ///This functions waits for the transmission to complete
    pub fn flush(&mut self){
        let ucsrna = read_volatile(&self.ucsrc);

        while !(ucsrna.get_bit(6)) {
            let ucsrna = read_volatile(&self.ucsrc);
        };
    }

    ///this enables parity generator for the frame
    pub fn parity_enable(&mut self){
        unsafe{

            self.ucsrnc.set_bit(5,true); 

        }
    }

    pub fn parity_disable(&mut self){
         
        unsafe{
            self.ucsrnc.set_bit(5,false); 
        }

    }

    ///This function is used to disable the Transmitter
    pub fn Transmitter_disable(&mut self) {
         
        /// check for data in Transmit Buffer and Tansmit shift register, if data is present in either then disabling of transmitter is not effective
        while !(git_bit(&self.uscrna,6) & get_bit(&self.uscrna,5) ) {};
        
        unsafe{
        self.ucsrnb.set_bit(3,false);

        }
    }      
}