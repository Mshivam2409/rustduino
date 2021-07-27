##![no_std]
#![no_main]
#![deny(warnings)]

use rustduino::delay::delay_ms;
use crate::atmega2560p::hal::port::*;
use crate::avr::shift::*;

#[no_mangle]
pub fn main() {
    let mut pins = Pins::new();
    loop{
        pins.setup();
        // Waiting for 2 seconds.
        delay_ms(2000);
    }
    
    
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}