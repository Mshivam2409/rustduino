
/// Crates which would be used in the implementation.
/// We will be using standard volatile and bit_field crates now for a better read and write.
use crate::atmega328p::com::usart_initialize::{Usart, UsartNum};

/// This struct contains  USART0 in ARDUINO MEGA arranged in a array.
/// First a new Serial is needed to be created to access all USARTs.

pub struct Serial {
    usart: [Usart; 1],
}

impl Serial {
    /// This function creates a new Serial struct.
    /// The struct serial will contain all the USARTs at one place.
    pub fn new() -> Serial {
        unsafe {
            Serial {
                usart: [
                    *Usart::new(UsartNum::Usart0),
                    
                ],
            }
        }
    }
}