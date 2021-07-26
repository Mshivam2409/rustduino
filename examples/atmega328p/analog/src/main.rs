#![no_std]
#![no_main]
#![deny(warnings)]

/// Crates to be used.
use rustduino::hal::pin::Pins;
use rustduino::hal::watchdog::Watchdog;
use rustduino::math::map_from1023_to255;

#[no_mangle]
pub fn main() {
    // Disable watchdog
    let watchdog = unsafe { WatchDog::new() };
    watchdog.disable();

    // Creates a array object consisting of all the pins.
    let mut pins = Pins::new();

    // Infinite loop for read and write continuously through the I/O pins.
    loop {
        // Take input into the zeroth analog pin.
        let a: u32 = pins.analog[0].read();

        // Make the input value ready to be sent through a digital pin.
        let b: u8 = map_from1023_to255(a);

        rustduino::delay::delay_ms(1000);

        // Give output from the 13th digital pin.
        pins.digital[13].write(b);

        rustduino::delay::delay_ms(1000);
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
