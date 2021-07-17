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


//! This file contains functions to enable transmission through the USART and do the transmission.
//! Flushing data in case of error and writing string are some complex implementations provided.
//! See the section 22 of ATMEGA2560P datasheet.
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf


/// Crates which would be used in the implementation.
/// We will be using standard volatile and bit_field crates now for a better read and write.
use core::ptr::{read_volatile,write_volatile};
use crate::rustduino::atmega2560p::com::usart_initialize::{Usart,UsartDataSize};
use crate::rustduino::hal::interrupts;
use bit_field::BitField;
use volatile::Volatile;


impl Usart {
    // Initialization setting begin function

    /// This function is to enable the Transmitter
    /// Once it is enabled it takes control of the TXDn pin as a transmitting output.   
    pub fn Transmit_enable(&mut self) {
        unsafe {
            self.ucsrb.update( |srb| {
                srb.set_bit(3, true);
            });    
        }
    }

    /// Storing data in Transmit Buffer which takes parameter as a u32 and and data bit length.
    pub fn transmitting_data (&self,data: Volatile<u32>,len: UsartDataSize) {
        unsafe{            
            // Checks if the Transmit buffer is empty to receive data.
            // If not the program waits till the time comes.
            let mut i : i32 = 10;
            while avai_write()==false { 
                if i!=0 {
                    delay_ms(1000);
                    i=i-1;
                }
                else {
                    unreachable!()
                }
            };

            // If the frame is ready for transmission then the appropriate place is written.
            match len {
                UsartDataSize::five  =>  self.udr.set_bits(0..5, data.get_bits(0..5)),
                UsartDataSize::six   =>  self.udr.set_bits(0..6, data.get_bits(0..6)),
                UsartDataSize::seven =>  self.udr.set_bits(0..7, data.get_bits(0..7)),
                UsartDataSize::eight =>  self.udr.set_bits(0..8, data.get_bits(0..8)),
                UsartDataSize::nine  =>  {
                    self.ucsrb.update( |ctrl| {
                        ctrl.set_bit(0, data.get_bit(8)); 
                    });

                    self.udr.set_bits(0..8, data.get_bits(0..8));
                }
            }
        }
    }  
    
    /// This functions waits for the transmission to complete by checking TXCn bit in the ucsrna register
    /// TXCn is set 1 when the transmit is completed and it can start transmitting new data 
    pub fn flush(&mut self){
        let mut ucsra = unsafe { read_volatile(&self.ucsrc) }; 
        let mut i : i32 =10;
        while ucsra.get_bit(6)==false {
            ucsra = unsafe { read_volatile(&self.ucsra) };    
            if i!=0 {
                delay_ms(1000);
                i=i-1;
            }
            else {
                unreachable!()
            }
        };
    }

    /// This function is used to disable the Transmitter and once disabled the TXDn pin is no longer
    /// used as the transmitter output pin and functions as a normal I/O pin
    pub fn transmit_disable(&mut self) {

        let mut uscra6=git_bit(&self.uscra,6);
        let mut uscra5=get_bit(&self.uscra,5);
        let mut i : i32 = 100;

        /// Check for data in Transmit Buffer and Transmit shift register,
        /// if data is present in either then disabling of transmitter is not effective
        while uscra6==false || uscra5==false {
            uscra6=git_bit(&self.uscra,6);
            uscra5=get_bit(&self.uscra,5);
            if i!=0 {
               delay_ms(1000);
               i=i-1;
            }
            else {
               unreachable!()
            }
        };
        
        unsafe {
            self.ucsrb.set_bit(3, false);
        }
    }

    /// This function sends a character byte of 5,6,7 or 8 bits
    pub fn Transmit_data(&self, data: Volatile<u8>) {
        unsafe {
            let ucsra = read_volatile(&self.ucsra);
            let udre = ucsra.get_bit(5);
            
            let mut i : i32 =100;
            while udre == false {
                let ucsra = read_volatile(&self.ucsra) ;
                let udre = ucsra.get_bit(5);

                if i != 0 {
                    rustduino::delay::delay_ms(1000);
                    i = i - 1;
                } else {
                    unreachable!();
                }
            };
            
            self.udr.write(data);
        }
    }

    /// This function send data type of string byte by byte
    pub fn write(&mut self, data: String) {
        self.Transmit_enable();
        for b in data.byte() {
            self.Transmit_data(b);
        }
        self.Transmit_disable();
    }

    pub fn write(&mut self, data: u32) {
        let mut v = Vec::new();
    }
}