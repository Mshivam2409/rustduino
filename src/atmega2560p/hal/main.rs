#![feature(stdsimd)]
#![no_std]
#![no_main]
#![deny(warnings)]
#![allow(unknown_lints)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::empty_loop)]

mod port;
// mod sim;
mod sleep_mode;
mod watchdog;
mod interrupt;

pub extern "C" fn main() {

    // Watchdog disabled by the program
    let wdog = watchdog::WatchDog::new();
    wdog.interrupt_toggle();
    wdog.disable();
    wdog.interrupt_toggle();

    /*
    // Enabling Clock Gating in the program at 32 cycles as an example
    let sim = sim::Sim::new();
    sim.enable_clock(32);
    */

    // Enabling sleep mode and setting the required sleep mode further it is disabled
    // The various sleep modes which could be used in the program are
        // Idle          // ADC Noise Reduction
        // Power-down    // Power-save
        // Standby       // Extended Standby

    let sleep = sleep_mode::Sleep::new();
    sleep.select_mode("Power-Save");

    /* Your device is running with sleep mode enabled in Power-Save mode now 
       You can work under sleep mode here in any mode you want 
       To work further on sleep mode comment the sleep.disable() command below 
    */

    sleep.disable();

    // setting up of Pin 5 as a GPIO pin
    p = port::PortName::C;
    let port = port::Port::new(p);
    let pin = port.pin(5);
    let mut gpio = pin.make_gpio();
    gpio.output();
    gpio.high();

    loop {}
}

extern "C" {
    fn _stack_top();
}

#[link_section = ".vectors"]
#[no_mangle]
pub static _VECTORS: [unsafe extern "C" fn(); 2] = [_stack_top, main];

#[link_section = ".flashconfig"]
#[no_mangle]

// This part is not confirmed as of now
/*
pub static _FLASHCONFIG : [u8 ; 16] = [
    // All the other bytes except FSEC and FOPT are to be changed.
    // 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	// 0xFF, 0xFF, 0xFF, 0xFF, FSEC, FOPT, 0xFF, 0xFF
    
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
	0xFF, 0xFF, 0xFF, 0xFF, 0xDE, 0xF9, 0xFF, 0xFF
];
*/
#[panic_handler]
fn teensy_panic(_pi: &core::panic::PanicInfo) -> ! {
    loop {}
}
