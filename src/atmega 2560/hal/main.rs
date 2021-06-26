#![feature(stdsimd)]
#![no_std]
#![no_main]
#![deny(warnings)]
#![allow(unknown_lints)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::empty_loop)]

mod port;
mod prescalar;
mod sleep_mode;
mod watchdog;
mod interrupt;
mod power;

pub extern "C" fn main() {

    // Watchdog disabled by the program
    let wdog = watchdog::WatchDog::new();
    wdog.disable();

    /*
    // Enabling Clock Gating in the program at 32 cycles as an example
    let sim = sim::Sim::new();
    sim.enable_clock(32);
    */

    // Enabling sleep mode and setting the required sleep mode further it is disabled
    // Various modes are
        // IDLE => Idle sleep mode          
        // ADC => ADC Noise Reduction
        // PD => Power-down    
        // PS => Power-save
        // SBY => Standby      
        // ESBY => Extended Standby

    let sleep = sleep_mode::Sleep::new();
    let mode = sleep_mode::Options::PD;
    sleep.select_mode(mode);

    /* Your device is running with sleep mode enabled in Power-Save mode now 
       You can work under sleep mode here in any mode you want 
       To work further on sleep mode comment the sleep.disable() command below 
    */

    sleep.disable();


    // Setting up various power modes by using power reduction registers
    // The options correspond to real world as shown - 
       //  TWI =>  Power Reduction TWI     
       //  TIMER2 =>  Power Reduction Timer/Counter2
       //  TIMER0 =>  Power Reduction Timer/Counter0 
       //  TIMER1 =>  Power Reduction Timer/Counter1
       //  SPI => Power Reduction Serial Peripheral Interface
       //  USART0 =>  Power Reduction USART0 
       //  ADC => Power Reduction ADC
       //  TIMER5 =>  Power Reduction Timer/Counter5 
       //  TIMER4 =>  Power Reduction Timer/Counter4
       //  TIMER3 =>  Power Reduction Timer/Counter3 
       //  USART3 => Power Reduction USART3
       //  USART2 => Power Reduction USART2  
       //  USART1 => Power Reduction USART1
    
    let power = power::Power::new();
    let m = power::Options::TWI;
    power.disable_clocks(m);

    /* Here you can work in low power scheme as here the TWI Clock system
       is shut off so that the power usage is low.
       You can shut down more clocks using the functions given */

    power.enable_clocks(m);


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

// #[link_section = ".flashconfig"]
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