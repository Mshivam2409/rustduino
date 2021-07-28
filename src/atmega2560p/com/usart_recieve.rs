//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Nikhil Gupta, Indian Institute of Technology Kanpur
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

//! This file contains the code for recieving data through a initialized USART.
//! This has functions to put USART in reciever mode and then read the data from the appropriate location.
//! See the section 22 of ATMEGA2560P datasheet.

// Other source code files to be used.
use crate::atmega2560p::com::usart_initialize::UsartObject;

// Crates which would be used in the implementation.
// We will be using standard volatile and bit_field crates now for a better read and write.
use crate::delay::delay_ms;
use bit_field::BitField;
use core::u32;

impl UsartObject {
    /// Enables the reciever function of microcontroller, whithout enabling it no communication is possible.
    pub unsafe fn recieve_enable(&mut self) {
        (*self.usart).ucsrb.update(|ucsrb| {
            ucsrb.set_bit(4, true);
        });
    }

    /// Checks if the data is avialable for reading or not.
    /// # Returns
    /// * `a boolean` - Which is false for no reading data and true if everything fine.
    pub fn available(&mut self) -> bool {
        let ucsra = unsafe { (*self.usart).ucsra.read() };
        if ucsra.get_bit(7) == true {
            true
        } else {
            false
        }
    }

    /// This is used to recieve data of one frame.
    /// Either 5 to 8 bits and 9 bits of data can be recieved from this function.
    /// In case of 5 to 8 bits this function returns u8.
    /// In case of 9 bits it retuns u32 of which first 9 bits are data recieved and remaining bits are insignificant.
    /// In case if an frame error or parity error occurs, this function returns Nothing.
    /// # Returns
    /// * `a Option<u32>` - which is NULL in case of wrong settings and read data u32 if valid input.
    pub fn recieve_data(&mut self) -> Option<u32> {
        let ucsrc = unsafe { (*self.usart).ucsrc.read() };
        let ucsrb = unsafe { (*self.usart).ucsrb.read() };

        let mut i: i32 = 10;
        while self.available() == false {
            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!()
            }
        }

        //  Case when there is 9 bits mode.
        if ucsrc.get_bits(1..3) == 0b11 && ucsrb.get_bit(2) == true {
            let ucsra = unsafe { (*self.usart).ucsra.read() };
            let mut udr: u32 = unsafe { (*self.usart).udr.read() as u32 };
            if ucsra.get_bits(2..5) != 0b000 {
                None
            } else {
                let rxb8: u32 = ucsrb.get_bits(1..2) as u32;
                udr.set_bits(8..9, rxb8);
                Some(udr)
            }
        }
        //  Case when there is a case of 5 to 8 bits.
        else {
            let ucsra = unsafe { (*self.usart).ucsra.read() };
            let udr: u32 = unsafe { (*self.usart).udr.read() as u32 };
            if ucsra.get_bits(2..5) != 0b000 {
                None
            } else {
                Some(udr)
            }
        }
    }

    /// Can be used to check frame error,Data OverRun and Parity errors.
    /// # Returns
    /// * `a boolean` - Which is true if error occurs,else false.
    pub fn error_check(&mut self) -> bool {
        let ucsra = unsafe { (*self.usart).ucsra.read() };
        if ucsra.get_bits(3..5) != 0b00 {
            true
        } else {
            false
        }
    }

    /// Can be used to check parity error.
    /// # Returns
    /// * `a boolean` - Which is true if error occurs,else false.
    pub fn parity_check(&mut self) -> bool {
        let ucsra = unsafe { (*self.usart).ucsra.read() };
        if ucsra.get_bit(2) == true {
            true
        } else {
            false
        }
    }

    /// Disables the reciever function of microcontroller.
    pub unsafe fn recieve_disable(&mut self) {
        (*self.usart).ucsrb.update(|ucsrb| {
            ucsrb.set_bit(4, false);
        });
    }

    /// Clears the unread data in the receive buffer by flushing it
    pub unsafe fn flush_recieve(&mut self) {
        let mut _udr = (*self.usart).udr.read();
        let mut ucsra = (*self.usart).ucsra.read();
        let mut i: i32 = 100;
        while ucsra.get_bit(7) == true {
            ucsra = (*self.usart).ucsra.read();
            _udr = (*self.usart).udr.read();
            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!()
            }
        }

        (*self.usart).ucsra.update(|ucsra| {
            ucsra.set_bit(7, false);
        });
    }

    ///  This is used to recieve data of one frame.
    ///  But it only functions when already data is available for read.which can be checked by available function.
    ///  Either 5 to 8 bits and 9 bits of data can be recieved from this function.
    ///  In case of 5 to 8 bits this function returns u8.
    ///  In case of 9 bits it retuns u32 of which first 9 bits are data recieved and remaining bits are insignificant.
    ///  In case ,if an frame error or parity error occurs, this function returns nothing.
    /// # Returns
    /// * `a Option<u32>` - which is NULL in case of wrong settings and read data u32 if valid input.
    pub fn read(&mut self) -> Option<u32> {
        let ucsrc = unsafe { (*self.usart).ucsrc.read() };
        let ucsrb = unsafe { (*self.usart).ucsrb.read() };

        let mut i: i32 = 10;
        while self.available() == false {
            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!()
            }
        }

        if ucsrc.get_bits(1..3) == 0b11 && ucsrb.get_bit(2) == true {
            let ucsra = unsafe { (*self.usart).ucsra.read() };
            let ucsrb = unsafe { (*self.usart).ucsrb.read() };
            let mut udr: u32 = unsafe { (*self.usart).udr.read() as u32 };
            if ucsra.get_bits(2..5) != 0b000 {
                None
            } else {
                let rxb8: u32 = ucsrb.get_bits(1..2) as u32;
                udr.set_bits(8..9, rxb8);
                Some(udr)
            }
        } else {
            let ucsra = unsafe { (*self.usart).ucsra.read() };
            let udr: u32 = unsafe { (*self.usart).udr.read() as u32 };
            if ucsra.get_bits(2..5) != 0b000 {
                None
            } else {
                Some(udr)
            }
        }
    }
}
