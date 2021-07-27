#![cfg(feature="rustduino")]
#![no_std]
#![deny(warnings)]
#![feature(asm)]
#![feature(llvm_asm)]


/// Library for AVR ATMEGA2560P Micro-controller
/// For more information see the data sheet provided below
/// `<https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf>`
#[cfg(feature = "atmega2560p")]
pub mod atmega2560p {

    /// Hardware Abstraction Library (HAL)
    #[cfg(feature = "hal")]
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
/// For more information see the data sheet provided below
/// `<https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf>`
#[cfg(feature = "atmega328p")]
pub mod atmega328p {

    /// Hardware Abstraction Library (HAL)
    #[cfg(feature = "hal")]
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
pub mod avr;

/// Math functions for assistance in implementation
pub mod math;

/// Configuration setup and time control
pub mod config;
pub mod delay;
