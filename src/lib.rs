#![no_std]
#![deny(warnings)]
#![feature(asm)]
#![feature(llvm_asm)]

//! Rustduino library for arduino
/*
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
*/
pub mod config;
pub mod delay;
