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
    pub mod hal {

        pub mod watchdog;

        pub mod sleep_mode;

        pub mod power;

        pub mod port;

        pub mod interrupts;

        pub mod pin;

        pub mod analog;

        pub mod digital;

        pub mod shift;
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

#[cfg(feature = "atmega2560p")]
cfg_if::cfg_if! {
    if #[cfg(doc)]{

    }
    else {
        pub use atmega2560p::*;
    }
}

/// Library for AVR ATMEGA328P Micro-controller
/// For more information see the data sheet provided below
/// `<https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf>`
#[cfg(feature = "atmega328p")]
pub mod atmega328p {

    /// Hardware Abstraction Library (HAL)
    pub mod hal {
        pub mod power;

        pub mod sleep_mode;

        pub mod watchdog;

        pub mod port;

        pub mod interrupts;

        pub mod pin;

        pub mod analog;

        pub mod digital;

        pub mod shift;
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

#[cfg(feature = "atmega328p")]
#[doc(hidden)]
pub use atmega328p::*;

/// Sensor control for AVR Chips
/// For more information see the following links.
/// `<https://server4.eca.ir/eshop/AHT10/Aosong_AHT10_en_draft_0c.pdf>`
/// `<https://invensense.tdk.com/wp-content/uploads/2015/02/MPU-6000-Datasheet1.pdf>`
/// `<https://www.aranacorp.com/en/control-a-servo-with-arduino/>`
#[cfg(feature = "sensors")]
pub mod sensors;

/// Math functions for assistance in implementation
#[cfg(feature = "math")]
pub mod math;

/// Low level control for AVR Chips
pub mod llvm;

#[doc(hidden)]
pub use llvm::*;

/// Configuration setup and time control
pub mod config;
pub mod delay;
