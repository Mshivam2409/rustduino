#![no_std]
#![no_main]
#![deny(warnings)]
use::rustduino::atmega2560p::com::{serial::*,usart,usart_transmit::*};
<<<<<<< HEAD
fn main() {
   //Create a new Serial struct to access all Usarts
   let serial=Serial::new();
   //This initializes usart0 and makes it ready to transmit and recieve.
   serial.usart[0].begin(9600);

   loop {
     //This sends string from arduino through Txd0 pin.
     serial.usart[0].write("Hello World!");
     rustduino::delay::delay_ms(1000);
   }
=======
pub fn main() {
    let serial=Serial::new();
    serial.usart[0].begin(9600);
    loop {
      serial.usart[0].write("Hello World!");
      rustduino::delay::delay_ms(1000);
    }
>>>>>>> 2fa9b0f28633621ce1fd936f13ac6e39b02f24b6
}
