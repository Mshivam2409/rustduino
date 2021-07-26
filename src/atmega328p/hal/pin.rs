//! Pins implementation.

use crate::hal::port::*;

/// All pins inside a single struct.
pub struct Pins {
    /// All six analog pins.
    pub analog: [AnalogPin; 6],

    /// All 14 digital I/O pins.
    pub digital: [DigitalPin; 14],
}

/// This struct contains the Pin struct and its analog pin number.
pub struct AnalogPin {
    pub pin: Pin,
    pub pinno: u32,
}

/// Structure to represent one digital pin with Pin structure and pin number.
pub struct DigitalPin {
    pub pin: Pin,
    pub pinno: usize,
}

impl Pins {
    /// Returns all pins at once as a single struct.
    pub fn new() -> Pins {
        Pins {
            analog: [
                AnalogPin {
                    pin: Pin::new(PortName::C, 0).unwrap(),
                    pinno: 0,
                },
                AnalogPin {
                    pin: Pin::new(PortName::C, 1).unwrap(),
                    pinno: 1,
                },
                AnalogPin {
                    pin: Pin::new(PortName::C, 2).unwrap(),
                    pinno: 2,
                },
                AnalogPin {
                    pin: Pin::new(PortName::C, 3).unwrap(),
                    pinno: 3,
                },
                AnalogPin {
                    pin: Pin::new(PortName::C, 4).unwrap(),
                    pinno: 4,
                },
                AnalogPin {
                    pin: Pin::new(PortName::C, 5).unwrap(),
                    pinno: 5,
                },
            ],
            digital: [
                DigitalPin {
                    pin: Pin::new(PortName::D, 0).unwrap(),
                    pinno: 0,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 1).unwrap(),
                    pinno: 1,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 2).unwrap(),
                    pinno: 2,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 3).unwrap(),
                    pinno: 3,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 4).unwrap(),
                    pinno: 4,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 5).unwrap(),
                    pinno: 5,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 6).unwrap(),
                    pinno: 6,
                },
                DigitalPin {
                    pin: Pin::new(PortName::D, 7).unwrap(),
                    pinno: 7,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 0).unwrap(),
                    pinno: 0,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 1).unwrap(),
                    pinno: 1,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 2).unwrap(),
                    pinno: 2,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 3).unwrap(),
                    pinno: 3,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 4).unwrap(),
                    pinno: 4,
                },
                DigitalPin {
                    pin: Pin::new(PortName::B, 5).unwrap(),
                    pinno: 5,
                },
            ],
        }
    }
}
