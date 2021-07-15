use bit_field::BitField;
use core;
use core::ptr::{read_volatile};
use volatile::Volatile;
use rustduino::atmega2560p::{usart_initialize,usart_initialize::Usart};

impl Usart{
   pub fn recieve_enable(&mut self){
    unsafe {
        let mut ucsrb=read_volatile(&self.ucsrb);
        self.ucsrb.write(ucsrb | (1<<4));
    }
   }

}