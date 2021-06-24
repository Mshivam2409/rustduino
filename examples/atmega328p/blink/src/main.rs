#![no_std]
#![no_main]
#![deny(warnings)]

use rustduino::atmega328p::hal as arduino_uno;

#[no_mangle]
pub extern "C" fn main() {
    // Disable the watchdog
    let wdog = arduino_uno::watchdog::Watchdog::new();
    wdog.disable();
    // Get a mutable reference to the Pins struct
    let mut pins = arduino_uno::pins::Pins::get();
    //Set the digital pin 13 as an output pin.
    pins.digital[13].set_output();
    // Iterative function to make the LED blink.
    loop {
        pins.digital[13].high();

        rustduino::delay_ms(1000);

        pins.digital[13].low();

        rustduino::delay_ms(1000);
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
