

/// Other source code files to be used.
use crate::atmega328p::hal::usart_initialize::Usart;

/// Crates which would be used in the implementation.
/// We will be using standard volatile and bit_field crates now for a better read and write.
use crate::delay::delay_ms;
use bit_field::BitField;
use core::u32;

impl Usart 
{
    /// This function enables the reciever function of microcontroller, whithout enabling it no communication is possible.
    pub fn recieve_enable(&mut self) {
        unsafe {
            self.ucsrb.update(|ucsrb| {
                ucsrb.set_bit(4, true);
            });
        }
    }
      /// This function checks if the data is avialable for reading or not.
    /// If no data is available for reading then 0 is returned.
    /// If data is available then 1 is returned.
    pub fn available(&mut self) -> bool {
        let ucsra = self.ucsra.read();
        if ucsra.get_bit(7) == true {
            true
        } else {
            false
        }
    }

    /// This function is used to recieve data of one frame.
    /// Either 5 to 8 bits and 9 bits of data can be recieved from this function.
    /// In case of 5 to 8 bits this function returns u8.
    /// In case of 9 bits it retuns u32 of which first 9 bits are data recieved and remaining bits are insignificant.
    /// In case ,if an frame error or parity error occurs, this function returns Nothing.
    pub fn recieve_data(&mut self) -> Option<u32> {
        unsafe {
            let ucsrc = self.ucsrc.read();
            let ucsrb = self.ucsrb.read();

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
                let ucsra = self.ucsra.read();
                let mut udr: u32 = self.udr.read() as u32;
                if ucsra.get_bits(2..5) != 0b000 {
                    None
                } else {
                    let rxb8: u32 = ucsrb.get_bits(1..2) as u32;
                    udr.set_bits(8..9, rxb8);
                    Some(udr)
                }
            }

            //  when there is a case of 5 to 8 bits.
            else {
                let ucsra = self.ucsra.read();
                let mut udr: u32 = self.udr.read() as u32;
                if ucsra.get_bits(2..5) != 0b000 {
                    None
                } else {
                    Some(udr)
                }
            }
        }
    }

    /// This function can be used to check frame error,Data OverRun and Parity errors.
    /// It returns true if error occurs,else false.
    pub fn error_check(&mut self) -> bool {
        unsafe {
            let ucsra = self.ucsra.read();
            if ucsra.get_bits(3..5) != 0b00 {
                true
            } else {
                false
            }
        }
    }


    /// This function can be used to check parity error.
    /// It returns true if error occurs else false.
    pub fn parity_check(&mut self) -> bool {
        unsafe {
            let ucsra = self.ucsra.read();
            if ucsra.get_bit(2) == true {
                true
            } else {
                false
            }
        }
    }

    /// This function disables the reciever function of microcontroller.
    pub fn recieve_disable(&mut self) {
        unsafe {
            self.ucsrb.update(|ucsrb| {
                ucsrb.set_bit(4, false);
            });
        }
    }
      /// This function clears the unread data in the receive buffer by flushing it
      pub fn flush_recieve(&self) {
        let mut udr = self.udr.read();
        let mut ucsra = self.ucsra.read();
        let mut i: i32 = 100;
        while ucsra.get_bit(7) == true {
            ucsra = self.ucsra.read();
            udr = self.udr.read();
            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!()
            }
        }

        unsafe {
            self.ucsra.update(|ucsra| {
                ucsra.set_bit(7, false);
            });
        }
    }

}
