#![no_std]
#![no_main]
#![deny(warnings)]
use::rustduino::atmega2560p::com::{serial::*,usart,usart_transmit::*};
pub fn main() {
    let serial=Serial::new();
    serial.usart[0].begin(9600);
    loop {
      serial.usart[0].write("Hello World!");
      rustduino::delay::delay_ms(1000);
    }
}
