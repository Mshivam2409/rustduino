#![no_std]
#![no_main]
#![deny(warnings)]
use::rustduino::atmega2560p::com::usart::*;
fn main() {
   //Create a new Serial struct to access all Usarts
   let serial=Serial::serial_new();
   //This initializes usart0 and makes it ready to transmit and recieve.
   serial.usart[0].begin_set_baud(9600);

   loop {
     //This sends string from arduino through Txd0 pin.
     serial.usart[0].write("Hello World!");
     rustduino::delay::delay_ms(1000);
   }
}
