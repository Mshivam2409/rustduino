#![no_std]
#![no_main]
#![deny(warnings)]

use rustduino::atmega2560p::hal::analog::RefType;
/// Crates to be used.
use rustduino::atmega2560p::hal::pin::Pins;

#[no_mangle]
pub fn main() {
    // Creates a array object consisting of all the pins.
    let mut pins = Pins::new();

    loop {
        // Take input into the first analog pin.
        pins.analog[0].analog_read(0, RefType::DEFAULT);

        rustduino::delay::delay_ms(1000);

        // Give output from the first analog pin.
        // pins.analog[0].analog_write();

        rustduino::delay::delay_ms(1000);
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
