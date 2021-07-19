

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
    
}