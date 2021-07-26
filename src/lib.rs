#![no_std]
#![deny(warnings)]
#![feature(asm)]
#![feature(llvm_asm)]

/// Library for ATmega328P chip.
#[cfg(feature = "atmega2560p")]
pub mod atmega2560p {
    /// Hardware Abstraction Library (HAL).
    #[cfg(feature = "atmega2560p-hal")]
    pub mod hal {
        pub mod port;

        pub mod pin;

        pub mod watchdog;

        pub mod interrupts;

        pub mod power;

        pub mod sleep_mode;
    }

    /// Communication Protocols
    #[cfg(feature = "com")]
    pub mod com {
        pub mod serial;

        pub mod usart;

        pub mod usart_transmit;

        pub mod usart_initialize;

        pub mod usart_recieve;
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
    }
<<<<<<< HEAD
=======

    #[cfg(feature = "com")]
    pub mod com {
        pub mod serial;

        pub mod usart;

        pub mod usart_initialize;

        pub mod usart_receive;

        pub mod usart_transmit;

        pub mod i2c;
    }
>>>>>>> 4cc062aeacfafa30ec421d6df3c66a5d99a7e247
}

#[cfg(feature = "atmega328p")]
pub use atmega328p::*;

<<<<<<< HEAD
=======
#[cfg(feature = "sensors")]
pub mod sensors;

>>>>>>> 4cc062aeacfafa30ec421d6df3c66a5d99a7e247
pub mod avr;
pub mod config;
pub mod delay;
