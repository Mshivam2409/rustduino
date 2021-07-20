#![no_std]
#![no_main]
#![deny(warnings)]

use rustduino::sensors::*;

#[no_mangle]
pub fn main() {
    let sensor = AHT10::new(& mut AHT10::get());

    loop {
        sensor.relative_humidity();
        sensor.temperature();
        rustduino::delay::delay_ms(2000);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
