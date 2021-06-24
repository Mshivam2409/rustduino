/*
#![no_std]

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/

#![no_std]
#![deny(warnings)]

/// Library for ATmega2560P chip.
pub mod atmega2560p {
    /// Hardware Abstraction Library (HAL).
    pub mod hal {
        pub mod watchdog;
        pub mod sleep_mode;
        pub mod port;
        pub mod interrupt;
    }
}
