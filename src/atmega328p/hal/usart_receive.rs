

/// Other source code files to be used.
use crate::atmega328p::hal::usart_initialize::Usart;

/// Crates which would be used in the implementation.
/// We will be using standard volatile and bit_field crates now for a better read and write.
use crate::delay::delay_ms;
use bit_field::BitField;
use core::u32;