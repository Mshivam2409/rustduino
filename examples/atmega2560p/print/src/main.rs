#![no_std]
#![no_main]
#![deny(warnings)]

/// Crates included to show the Transmission of String example.
use rustduino::atmega2560p::com::usart::println_string;
use rustduino::atmega2560p::hal::watchdog::WatchDog;

#[no_mangle]
pub fn main() {
    // Disable watchdog
    let watchdog = unsafe { WatchDog::new() };
    watchdog.disable();

    // Send's the required message to the peripheral device at interval of 2 seconds.
    loop {
        // Print function to transmit string through USART.
        println_string("Hello World !!!");

        rustduino::delay::delay_ms(2000);
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
