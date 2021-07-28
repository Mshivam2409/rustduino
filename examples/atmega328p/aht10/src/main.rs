#![no_std]
#![no_main]
#![deny(warnings)]

// Crates included which are to be used for the AHT10 example.
use rustduino::sensors::*;

#[no_mangle]
fn main() {
    let sensor = AHT10::new(&mut AHT10::get());

    loop {
        // Get relative humidity.
        sensor.relative_humidity();

        // Get temperature
        sensor.temperature();

        // Waiting for 2 seconds.
        rustduino::delay::delay_ms(2000);
    }
} // Get temperature and relative humidity at 2 second interval

// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
