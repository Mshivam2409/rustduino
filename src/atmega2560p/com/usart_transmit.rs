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

use crate::atmega2560p::com::usart_initialize::{Usart, UsartDataSize};
use crate::delay::delay_ms;
/// Crates which would be used in the implementation.
/// We will be using standard volatile and bit_field crates now for a better read and write.
use bit_field::BitField;
use fixed_slice_vec::FixedSliceVec;
//This is a implementation for Usart
impl Usart {
    /// Initialization setting begin function
    /// This function is to enable the Transmitter
    /// Once it is enabled it takes control of the TXDn pin as a transmitting output.   
    pub fn transmit_enable(&mut self) {
        unsafe {
            self.ucsrb.update(|srb| {
                srb.set_bit(3, true);
            });
        }
    }

    /// Storing data in Transmit Buffer which takes parameter as a u32 and and data bit length.
    pub fn transmitting_data(&self, data: u32, len: UsartDataSize) {
        unsafe {
            // Checks if the Transmit buffer is empty to receive data.
            // If not the program waits till the time comes.
            let mut i: i32 = 10;
            while self.avai_write() == false {
                if i != 0 {
                    delay_ms(1000);
                    i = i - 1;
                } else {
                    unreachable!()
                }
            }

            let udr = self.udr.read();

            // If the frame is ready for transmission then the appropriate place is written.
            match len {
                UsartDataSize::Five => {
                    udr.set_bits(0..5, data.get_bits(0..5) as u8);
                }
                UsartDataSize::Six => {
                    udr.set_bits(0..6, data.get_bits(0..6) as u8);
                }
                UsartDataSize::Seven => {
                    udr.set_bits(0..7, data.get_bits(0..7) as u8);
                }
                UsartDataSize::Eight => {
                    udr.set_bits(0..8, data.get_bits(0..8) as u8);
                }
                UsartDataSize::Nine => {
                    self.ucsrb.update(|ctrl| {
                        ctrl.set_bit(0, data.get_bit(8));
                    });
                    udr.set_bits(0..8, data.get_bits(0..8) as u8);
                }
            }
        }
    }
    ///This function checks that transmission buffer is ready to be
    pub fn avai_write(&mut self) -> bool {
        let ucsra = self.ucsra.read();
        if ucsra.get_bit(5) == true {
            true
        } else {
            false
        }
    }

    /// This functions waits for the transmission to complete by checking TXCn bit in the ucsrna register
    /// TXCn is set 1 when the transmit is completed and it can start transmitting new data
    pub fn flush_transmit(&mut self) {
        let mut ucsra = self.ucsra.read();
        let mut i: i32 = 10;
        while ucsra.get_bit(6) == false {
            ucsra = self.ucsra.read();
            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!()
            }
        }
    }

    /// This function is used to disable the Transmitter and once disabled the TXDn pin is no longer
    /// used as the transmitter output pin and functions as a normal I/O pin
    pub fn transmit_disable(&mut self) {
        let ucsra = self.ucsra.read();
        let mut uscra6 = ucsra.get_bit(6);
        let mut uscra5 = ucsra.get_bit(5);
        let mut i: i32 = 100;

        // Check for data in Transmit Buffer and Transmit shift register,
        // if data is present in either then disabling of transmitter is not effective
        while uscra6 == false || uscra5 == false {
            uscra6 = ucsra.get_bit(6);
            uscra5 = ucsra.get_bit(5);
            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!()
            }
        }

        unsafe {
            self.ucsrb.update(|srb| {
                srb.set_bit(3, false);
            });
        }
    }

    /// This function sends a character byte of 5,6,7 or 8 bits
    pub fn transmit_data(&self, data: u8) {
        unsafe {
            let ucsra = self.ucsra.read();
            let udre = ucsra.get_bit(5);

            let mut i: i32 = 100;
            while udre == false {
                let ucsra = self.ucsra.read();
                let udre = ucsra.get_bit(5);

                if i != 0 {
                    delay_ms(1000);
                    i = i - 1;
                } else {
                    unreachable!();
                }
            }

            self.udr.write(data);
        }
    }

    /// This function send data type of string byte by byte.
    pub fn write(&mut self, data: &mut str) {
        self.Transmit_enable();
        let mut vec: FixedSliceVec<u8> = unsafe { FixedSliceVec::from_bytes((data).as_bytes()) };
        for i in 0..(vec.len()) {
            self.transmit_data(vec[i]);
        }
        self.transmit_disable();
    }

    /// This function send data type of int(u32) byte by byte.
    pub fn write_integer(&mut self, data: u32) {
        let mut vec: FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        let mut a = data;
        while a != 0 {
            let rem = a % 10;
            a = a / 10;
            match rem {
                0 => vec.push('0' as u8),
                1 => vec.push('1' as u8),
                2 => vec.push('2' as u8),
                3 => vec.push('3' as u8),
                4 => vec.push('4' as u8),
                5 => vec.push('5' as u8),
                6 => vec.push('6' as u8),
                7 => vec.push('7' as u8),
                8 => vec.push('8' as u8),
                9 => vec.push('9' as u8),
                _ => (),
            }
        }
        for i in 0..(vec.len()) {
            self.transmit_data(vec[(vec.len) - 1 - i]);
        }
    }
    
    /// This function send data type of float(f64) byte by byte.
    pub fn write_float(&mut self,data : f64) {
    let mut vec: FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
    let mut a:f64 = data;
    let mut f:f64 = 0.0;
    let mut i:i64 = a-(a%1.0);
    f = a%1.0;
    while f%1.0 {
        let t = f;
        let k:f64 = f*(10.0) -((f*10.0)%1.0); // gives you decimal digit of data one by one from left to right 
                match k {
                0 => vec.push('0' as u8),
                1 => vec.push('1' as u8),
                2 => vec.push('2' as u8),
                3 => vec.push('3' as u8),
                4 => vec.push('4' as u8),
                5 => vec.push('5' as u8),
                6 => vec.push('6' as u8),
                7 => vec.push('7' as u8),
                8 => vec.push('8' as u8),
                9 => vec.push('9' as u8),
                _ => (),
            }
        f = (f*10.0)%1.0; // then f loses its left most digit (in decimal part) 
    }
    vec.push('.' as char);

            while i != 0 {
            let rem = i % 10;
            i = i / 10;
            match rem {
                0 => vec.push('0' as u8),
                1 => vec.push('1' as u8),
                2 => vec.push('2' as u8),
                3 => vec.push('3' as u8),
                4 => vec.push('4' as u8),
                5 => vec.push('5' as u8),
                6 => vec.push('6' as u8),
                7 => vec.push('7' as u8),
                8 => vec.push('8' as u8),
                9 => vec.push('9' as u8),
                _ => (),
            }
        }

        for i in 0..(vec.len()) {
            self.transmit_data(vec[(vec.len) - 1 - i]);
        }
    }
    
}
