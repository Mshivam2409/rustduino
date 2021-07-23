#![no_std]
#![no_main]
#![deny(warnings)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

/// Source codes required.
use rustduino::math;

#[no_mangle]
fn main() {
    
}


/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}