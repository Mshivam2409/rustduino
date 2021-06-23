#![no_std]
#![deny(warnings)]

/// Library for ATmega328P chip.
pub mod atmega328p {
    /// Hardware Abstraction Library (HAL).
    pub mod hal {
        pub mod port;

        pub mod pins;

        pub mod watchdog;

        pub mod interrupt;
    }
}

pub fn delay_ms(ms: u32) {
    avr_delay::delay_ms(ms);
}

pub fn delay_us(us: u32) {
    avr_delay::delay_us(us);
}