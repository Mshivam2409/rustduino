#![no_std]
#![deny(warnings)]
#![feature(asm)]
#![feature(llvm_asm)]
#![allow(dead_code)]

//! Rustduino library for arduino

/// Library for ATmega2560P chip.
pub mod atmega2560p {
    /// Hardware Abstraction Library (HAL).
    pub mod hal {
        pub mod power;

        pub mod watchdog;

        pub mod sleep_mode;

        pub mod port;

        pub mod interrupts;

        pub mod pin;

    }
    /// Serial communication (COM).
    pub mod com {
        pub mod i2c;
    }
        //pub mod usart;

}

/// Library for ATmega328P chip.
pub mod atmega328p {
    /// Hardware Abstraction Library (HAL).
    pub mod hal {
        pub mod port;

        pub mod pins;

        pub mod watchdog;

        pub mod interrupt;

        pub mod power;

        pub mod gating;
    }
}

pub mod avr;
pub mod config;
pub mod delay;
