#![no_std]
#![no_main]
#![deny(warnings)]

/*
/// Crates to be used.
use rustduino::atmega2560p::hal::analog::Analog;
*/

#[no_mangle]
pub fn main() {
    // let a = unsafe { Analog::new() };    
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
