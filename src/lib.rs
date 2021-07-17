#![no_std]
#![deny(warnings)]
#![feature(asm)]
#![feature(llvm_asm)]

//! Rustduino library for arduino

/// Library for ATmega328P chip.
pub mod atmega328p {
    /// Hardware Abstraction Library (HAL).
    pub mod hal {
        pub mod gating;

        pub mod interrupt;

        pub mod pins;

        pub mod port;

        pub mod power;

        pub mod usart;

        pub mod watchdog;
    }
}

pub mod config;
pub mod delay;
