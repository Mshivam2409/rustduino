//! Pins implementation.

use crate::atmega328p::hal::port::*;

/// All pins inside a single struct.
pub struct Pins {
    /// All five analog pins.
    pub analog: [Pin; 5],

    /// All 14 digital I/O pins.
    pub digital: [Pin; 14],
}

impl Pins {
    /// Returns all pins at once as a single struct.
    pub fn new() -> Pins {
        Pins {
            analog: [
                Pin::new(PortName::C, 0).unwrap(),
                Pin::new(PortName::C, 1).unwrap(),
                Pin::new(PortName::C, 2).unwrap(),
                Pin::new(PortName::C, 3).unwrap(),
                Pin::new(PortName::C, 4).unwrap(),
            ],
            digital: [
                Pin::new(PortName::D, 0).unwrap(),
                Pin::new(PortName::D, 1).unwrap(),
                Pin::new(PortName::D, 2).unwrap(),
                Pin::new(PortName::D, 3).unwrap(),
                Pin::new(PortName::D, 4).unwrap(),
                Pin::new(PortName::D, 5).unwrap(),
                Pin::new(PortName::D, 6).unwrap(),
                Pin::new(PortName::D, 7).unwrap(),
                Pin::new(PortName::B, 0).unwrap(),
                Pin::new(PortName::B, 1).unwrap(),
                Pin::new(PortName::B, 2).unwrap(),
                Pin::new(PortName::B, 3).unwrap(),
                Pin::new(PortName::B, 4).unwrap(),
                Pin::new(PortName::B, 5).unwrap(),
            ],
        }
    }
}
