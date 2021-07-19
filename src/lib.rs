#![no_std]
#![deny(warnings)]
#![feature(asm)]
#![feature(llvm_asm)]
#![allow(dead_code)]
#![allow(unused_unsafe)]
// #![allow(non_camel_case_types)]

/// Library for ATmega2560P chip.
#[cfg(feature = "atmega2560p")]
pub mod atmega2560p {

    /// Hardware Abstraction Library (HAL).
    #[cfg(feature = "atmega2560p-hal")]
    pub mod hal {
        pub mod power;

        pub mod watchdog;

        pub mod sleep_mode;

        pub mod port;

        pub mod interrupts;

        pub mod pin;
    }
}

#[cfg(feature = "atmega2560p")]
pub use atmega2560p::*;

<<<<<<< HEAD
pub mod atmega328p {

=======
#[cfg(feature = "atmega328p")]
pub mod atmega328p {
    // Hardware Abstraction Library (HAL).
    #[cfg(feature = "atmega328p-hal")]
>>>>>>> 0ab5b4ee0de4ec5b903be34d9842e442581ec584
    pub mod hal {
        pub mod port;

        pub mod pins;

        pub mod watchdog;

        pub mod interrupt;

        pub mod power;

        pub mod gating;
    }

    #[cfg(feature = "com")]
    pub mod com {
        pub mod i2c;
    }
}

<<<<<<< HEAD
=======
#[cfg(feature = "atmega328p")]
pub use atmega328p::*;

#[cfg(feature = "sensors")]
>>>>>>> 0ab5b4ee0de4ec5b903be34d9842e442581ec584
pub mod sensors {
    pub mod aht10;
}

pub mod avr;
pub mod config;
pub mod delay;
