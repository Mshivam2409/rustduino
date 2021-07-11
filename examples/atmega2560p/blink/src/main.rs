#![no_std]
#![no_main]
#![deny(warnings)]

use rustduino::atmega2560p::hal;

#[no_mangle]
pub extern "C" fn main() {
    // Disable watchdog
    let watchdog = hal::watchdog::Watchdog::new();
    watchdog.disable();
    //this pins represents pin 2 of port B
    let mut pins = hal::port::Pin::new(hal::port::PortName::B,2);
    //this sets pin 2 of port B as output
    pins.set_pin_mode(&mut self,hal::port::IOMode::Output);

    loop {
        //this sets pin high
        pins.high();

        rustduino::delay_ms(1000);
        //this sets pin as low
        pins.low();

        rustduino::delay_ms(1000);
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
