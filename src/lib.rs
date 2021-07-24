#![no_std]
#![deny(warnings)]
#![feature(asm)]
#![feature(llvm_asm)]

/// Library for AVR ATMEGA2560P Micro-controller
#[cfg(feature = "atmega2560p")]
pub mod atmega2560p {

    /// Hardware Abstraction Library (HAL)
    #[cfg(feature = "atmega2560p-hal")]
    pub mod hal {
        pub mod power;

        pub mod watchdog;

        pub mod sleep_mode;

        pub mod port;

        pub mod interrupts;

        pub mod pin;

        pub mod analog;

        pub mod digital;
    }

    /// Communication Control Library
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

/// Library for AVR ATMEGA2560P Micro-controller
#[cfg(feature = "atmega2560p")]
pub use atmega2560p::*;

/// Library for AVR ATMEGA328P Micro-controller
#[cfg(feature = "atmega328p")]
pub mod atmega328p {

    /// Hardware Abstraction Library (HAL)
    #[cfg(feature = "atmega328p-hal")]
    pub mod hal {
        pub mod power;

        pub mod watchdog;

        pub mod sleep_mode;

        pub mod port;

        pub mod interrupts;

        pub mod pin;

        pub mod analog;

        pub mod digital;
    }

    /// Communication Control Library
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

/// Library for AVR ATMEGA328P Micro-controller
#[cfg(feature = "atmega328p")]
pub use atmega328p::*;

/// Sensor control for AVR Chips
#[cfg(feature = "sensors")]
pub mod sensors;

/// Low level control for AVR Chips
#[cfg(feature = "avr")]
pub mod avr;

/// Random Number Generation Features
#[cfg(feature = "random")]
pub mod math;

pub mod config;
pub mod delay;
