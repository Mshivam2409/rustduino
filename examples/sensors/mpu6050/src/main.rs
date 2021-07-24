#![no_std]
#![no_main]
#![deny(warnings)]

use rustduino::sensors::mpu6050::*;
use rustduino::hal::watchdog::*;
use rustduino::delay::delay_ms;

#[no_mangle]
pub fn main() {
    // Disable watchdog
    let watchdog = unsafe { WatchDog::new() };
    watchdog.disable();
    // Initialize MPU6050 struct.
    let sensor = MPU6050::new();
    
    loop {
        sensor.begin(MPUdpsT::MPU6050Scale250DPS,MPURangeT::MPU6050Range2G);
        
        sensor.read_gyro();
        //Print these values on screen using USART;
        //The vec gyro_output stores the raw values of the gyroscope where gyro_output[0] is the x-axis, gyro_output[1] is the y-axis and gyro_output[2] is the z-axis output respectively.These raw values are then converted to degrees per second according to the scale given as input in `begin()` function.
        
        sensor.read_accel();
        //Print these values on screen using USART;
        //The vec accel_output stores the raw values of the accelerometer where accel_output[0] is the x-axis, accel_output[1] is the y-axis and accel_output[2] is the z-axis output respectively.These raw values are then converted to g's per second according to the scale given as input in `begin()` function.

        // Waiting for 2 seconds.
        delay_ms(2000);
    }
} 

// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}