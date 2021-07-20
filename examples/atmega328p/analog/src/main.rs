#![no_std]
#![no_main]
#![deny(warnings)]

/*/
/// Crates to be used.
use rustduino::atmega328p::hal::analog::Analog;
*/

#[no_mangle]
pub fn main() {
    
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
