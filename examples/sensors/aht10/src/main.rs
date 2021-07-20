#![no_std]
#![no_main]
#![deny(warnings)]

use rustduino::aht10::AHT10;

#[no_mangle]
pub fn main(){
    let sensor = unsafe { AHT10::new() };
    
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
