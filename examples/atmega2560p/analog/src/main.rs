#![no_std]
#![no_main]
#![deny(warnings)]

/// Crates to be used.
use rustduino::atmega2560p::hal::analog::{map_from1023_to255, RefType};
use rustduino::atmega2560p::hal::analogpins::AnalogPins;
use rustduino::atmega2560p::hal::digitalpins::DigitalPins;
use rustduino::atmega2560p::hal::watchdog::WatchDog;

#[no_mangle]
pub fn main() {
    // Disable watchdog
    let watchdog = unsafe { WatchDog::new() };
    watchdog.disable();

    // Creates a array object consisting of all the pins.
    let mut analog_pins = AnalogPins::new();
    let mut digital_pins = DigitalPins::new();

    // Infinite loop for read and write continuously through the I/O pins.
    loop {
        // Take input into the zeroth analog pin.
        let a: u32 = analog_pins.analog[0].analog_read(RefType::DEFAULT);

        // Make the input value ready to be sent through a digital pin.
        let b: u8 = map_from1023_to255(a);

        rustduino::delay::delay_ms(1000);

        // Give output from the 13th digital pin.
        digital_pins.digital[13].analog_write(b);

        rustduino::delay::delay_ms(1000);
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}