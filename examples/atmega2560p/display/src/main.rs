#![no_std]
#![no_main]
//#![deny(warnings)]

use rustduino::delay::delay_ms;
//use rustduino::atmega2560p::hal::port::*;
//use rustduino::avr::shift::*;
use rustduino::avr::display::*;

#[no_mangle]
pub fn main() {
    let mut i:u8=0;
    loop{
        if i%2==0{
            setup(4,8,7,true,true,0);
            delay_ms(1000);
        }
        else {
            setup(4,8,7,false,true,1);
            delay_ms(1000);
        }
        i +=1;
    }
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}