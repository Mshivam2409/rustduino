#![no_std]
#![no_main]
#![deny(warnings)]

/// Crates included which are to be used for the Blinking LED example.
use rustduino::atmega2560p::hal::pin::Pins;
use rustduino::atmega2560p::hal::watchdog::WatchDog;

#[no_mangle]
pub fn main() {
    // Disable watchdog
    let watchdog = unsafe { WatchDog::new() };
    watchdog.disable();

    // This pins represents pin 7 of port B ( pin 13).
    let mut pins = Pins::new();

    //This sets pin 7 of port B (pin 13) as output.
    pins.digital[13].set_output();

    loop {
        //This sets pin high.
        pins.digital[13].high();

        rustduino::delay::delay_ms(1000);
        //This sets pin as low.
        pins.digital[13].low();

        rustduino::delay::delay_ms(1000);
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
