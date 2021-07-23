#![no_std]
#![deny(warnings)]
#![feature(asm)]
#![feature(llvm_asm)]
#![allow(dead_code)]
#![allow(unused_variables)]

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

        pub mod analog;

        pub mod digitalpins;

        pub mod analogpins;
    }

    /// Communication Protocols
    #[cfg(feature = "com")]
    pub mod com {
        pub mod serial;

        pub mod usart;

        pub mod usart_transmit;

        pub mod usart_initialize;

        pub mod usart_recieve;

        pub mod i2c;
    }
}

#[cfg(feature = "atmega2560p")]
pub use atmega2560p::*;

/// Library for ATmega328P chip.
#[cfg(feature = "atmega328p")]
pub mod atmega328p {
    /// Hardware Abstraction Library (HAL).
    #[cfg(feature = "atmega328p-hal")]
    pub mod hal {
        pub mod port;

        pub mod pins;

        pub mod watchdog;

        pub mod interrupt;

        pub mod power;

        pub mod gating;

        pub mod analog;

        pub mod analogpins;

        pub mod digitalpins;
    }

    #[cfg(feature = "com")]
    pub mod com {}
}

#[cfg(feature = "atmega328p")]
pub use atmega328p::*;

#[cfg(feature = "sensors")]
pub mod sensors {
    pub mod mpu6050;
}

pub mod avr;
pub mod config;
pub mod delay;
pub mod math;
