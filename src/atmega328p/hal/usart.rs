//! USART implementation


/// Crates which would be used in the implementation.
/// We will be using standard volatile and bit_field crates now for a better read and write.
use core::ptr::{read_volatile,write_volatile};
use bit_field::BitField;
use volatile::Volatile;
use crate::avr::__nop;
use crate::rustduino::atmega328p::hal::interrupts;
use crate::rustduino::atmega328p::hal::port;
use crate::rustduino::atmega328p::hal::power;
use crate::delay::{delay_s,delay_ms,delay_us};
