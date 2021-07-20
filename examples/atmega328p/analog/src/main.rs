#![no_std]
#![no_main]
#![deny(warnings)]
#![allow(unused_variables)]

/// Crates to be used.
use rustduino::atmega328p::hal::analog::Analog;

#[no_mangle]
pub fn main() {
    let a = unsafe { Analog::new() };
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
