#![no_std]
#![deny(warnings)]

//! Rustduino library for arduino

/// Library for ATmega328P chip.
pub mod atmega328p {
    /// Hardware Abstraction Library (HAL).
    pub mod hal {
        pub mod port;

        pub mod pins;

        pub mod watchdog;

        pub mod interrupt;

        pub mod power;
    }
}

/// delay for N miliseconds
///
/// ## Arguments
/// * 'ms' - an u32, number of milliseconds to busy-wait
pub fn delay_ms(ms: u32) {
    avr_delay::delay_ms(ms);
}

/// delay for N microseconds
///
/// ## Arguments
/// * 'us' - an u32, number of microseconds to busy-wait
pub fn delay_us(us: u32) {
    avr_delay::delay_us(us);
}
