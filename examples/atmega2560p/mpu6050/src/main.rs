#![no_std]
#![no_main]
#![deny(warnings)]

use rustduino::sensors::*;

#[no_mangle]
pub fn main() {
     // Disable watchdog
     let watchdog = unsafe { WatchDog::new() };
     watchdog.disable();
 
    let sensor = MPU6050::new(& mut MPU6050::get());

    loop {
        
        sensor.begin();

        
        sensor.read_gyro();

        sensor.read_accel();

        // Waiting for 2 seconds.
        rustduino::delay::delay_ms(2000);
    }
} 

// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}