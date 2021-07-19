/// Library for ATmega328P chip.
// #[cfg(feature = "atmega328p")]
pub mod atmega328p {
    /// Hardware Abstraction Library (HAL).
    // #[cfg(feature = "atmega328p-hal")]
    pub mod hal {
        pub mod port;

        pub mod pins;

        pub mod watchdog;

        pub mod interrupt;

        pub mod power;

        pub mod gating;
    }

    // #[cfg(feature = "com")]
    pub mod com {
         pub mod usart;

         pub mod usart_initialize;

         pub mod usart_receive;

         pub mod usart_transmit;

         pub mod serial;
    }
}

// #[cfg(feature = "atmega328p")]
pub use atmega328p::*;