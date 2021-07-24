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
use bit_field::BitField;
use core::{f64, u8, usize};
use fixed_slice_vec::FixedSliceVec;

/// Other source code files to be used.
use crate::atmega2560p::com::usart_initialize::{UsartDataSize, UsartObject};
use crate::delay::delay_ms;

//This is a implementation for Usart
impl UsartObject {
    /// Enables the Transmitter.
    /// once it is enabled it takes control of the TXDn pin as a transmitting output.   
    pub unsafe fn transmit_enable(&mut self) {
        (*self.usart).ucsrb.update(|srb| {
            srb.set_bit(3, true);
        });
    }

    /// Storing data in Transmit Buffer which takes parameter as a u32 and and data bit length.
    pub unsafe fn transmitting_data(&mut self, data: u32, len: UsartDataSize) {
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

        let mut udr = (*self.usart).udr.read();

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
                (*self.usart).ucsrb.update(|ctrl| {
                    ctrl.set_bit(0, data.get_bit(8));
                });
                udr.set_bits(0..8, data.get_bits(0..8) as u8);
            }
        }
    }

    /// Checks that transmission buffer if ready for transmission.
    /// Returns true if ready otherwise false.
    pub unsafe fn avai_write(&mut self) -> bool {
        let ucsra = (*self.usart).ucsra.read();
        if ucsra.get_bit(5) == true {
            true
        } else {
            false
        }
    }

    /// This waits for the transmission to complete by checking the appropriate register.
    pub unsafe fn flush_transmit(&mut self) {
        let mut ucsra = (*self.usart).ucsra.read();
        let mut i: i32 = 10;
        while ucsra.get_bit(6) == false {
            ucsra = (*self.usart).ucsra.read();
            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!()
            }
        }
    }

    /// This is used to disable the Transmitter and once disabled the pins used for USART
    /// return into their default I/O pin mode.
    pub fn transmit_disable(&mut self) {
        let ucsra = unsafe { (*self.usart).ucsra.read() };
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
            (*self.usart).ucsrb.update(|srb| {
                srb.set_bit(3, false);
            });
        }
    }

    /// Sends a character byte of 5,6,7 or 8 bits.
    pub fn transmit_data(&mut self, data: u8) {
        let mut ucsra = unsafe { (*self.usart).ucsra.read() };
        let mut udre = ucsra.get_bit(5);

        let mut i: i32 = 100;
        while udre == false {
            ucsra = unsafe { (*self.usart).ucsra.read() };
            udre = ucsra.get_bit(5);

            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!();
            }
        }

        unsafe {
            self.set_txn();
            (*self.usart).udr.write(data)
        };
    }

    /// Send's data of type string byte by byte using USART.
    pub fn write_string(&mut self, data: &'static str) {
        let mut vec: FixedSliceVec<u8> = FixedSliceVec::new(&mut []);

        for c in data.chars() {
            vec.push(c as u8);
        }

        for i in 0..(vec.len()) {
            self.transmit_data(vec[i]);
        }
    }

    /// Send's data of type integer(u32) byte by byte.
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
                _ => unreachable!(),
            }
        }
        for i in 0..(vec.len()) {
            self.transmit_data(vec[vec.len() - 1 - i]);
        }
    }

    /// Send's data of type float(f64) byte by byte till the precision required.
    pub fn write_float(&mut self, data: f64, precision: u32) {
        let mut vec: FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        let a: f64 = data;
        let mut f: f64 = a % 1.0;
        let mut i: i64 = (a - (a % 1.0)) as i64;
        let mut x: u32 = precision;
        let mut n: usize = 0;
        while f != 0.00 && x != 0 {
            let k: i64 = ((f * 10.0) - ((f * 10.0) % 1.0)) as i64; // gives you decimal digit of data one by one from left to right
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
                _ => unreachable!(),
            }
            f = (f * 10.0) % 1.0; // then f loses its left most digit (in decimal part)
            x = x - 1;
            n = n + 1;
        }

        vec.push('.' as u8);

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

        for ia in 0..(vec.len() - n - 1) {
            self.transmit_data(vec[vec.len() - 1 - ia]);
        }

        for ia in 0..n - 1 {
            self.transmit_data(vec[ia]);
        }
    }
}
