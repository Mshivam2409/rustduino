//! This file contains functions to enable transmission through the USART and do the transmission.
//! Flushing data in case of error and writing string are some complex implementations provided.
//! See the section 19 of ATMEGA328P datasheet.
//! https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf

use crate::atmega320p::hal::usart_initialize::{Usart, UsartDataSize};
use crate::delay::delay_ms;
/// Crates which would be used in the implementation.
/// We will be using standard volatile and bit_field crates now for a better read and write.
use bit_field::BitField;
use fixed_slice_vec::FixedSliceVec;
//This is a implementation for Usart
impl Usart {
    /// Initialization setting begin function
    /// This function is to enable the Transmitter
    /// Once it is enabled it takes control of the TXDn pin as a transmitting output.   
    pub fn transmit_enable(&mut self) {
        unsafe {
            self.ucsrb.update(|srb| {
                srb.set_bit(3, true);
            });
        }
    }
}