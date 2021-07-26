#![no_std]
#![no_main]
#![deny(warnings)]

use rustduino::hal::watchdog::WatchDog;
/// Source codes required.
use rustduino::math::RandomNumberGenerator;

#[no_mangle]
pub fn main() {
    // Disable the watchdog.
    let wdog = unsafe { WatchDog::new() };
    wdog.disable();

    let mut rand = RandomNumberGenerator::new();

    loop {
        // Generate Random numbers using Analog pin inputs.
        // This number could be sent to peripheral device using USART.
        let _x: u8 = rand.generate_by_analog();

        // Generate Random numbers by MPU6050 gyroscopic sensor.
        // This number could be sent to peripheral device using USART.
        let _y: u8 = rand.generate_by_mpu();
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
