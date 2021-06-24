#![no_std]
#![no_main]
#![deny(warnings)]

use rustduino::atmega328p::hal as arduino_uno;

#[no_mangle]
pub extern "C" fn main() {
    // Disable watchdog
    let wdog = arduino_uno::watchdog::Watchdog::new();
    wdog.disable();

    let mut pins = arduino_uno::pins::Pins::get();

    pins.digital[13].set_output();

    loop {
        pins.digital[13].high(); // setting digital portpin13 to 1(high)

        rustduino::delay_ms(1000); // adding a delay between high and low to 1s

        pins.digital[13].low(); // setting pin to 0

        rustduino::delay_ms(1000);
    }
} //toggling the LED periodically

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
