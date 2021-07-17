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

/// Some useful constants regarding bit manipulation for USART.
/// Position of clock mode adjuster (xck) bit.
const usart0_xck : u8 = 4;
/// Position of Transmission bit for various USART.
const usart0_td  : u8 = 1;
/// Position of Reciever bit for various USART.
const usart0_rd  : u8 = 0;
/// System Clock Crystal Oscillator Frequency in mHz.
const f_osc : f64 = 1.0000;
const multiply : i32 = 1000000;


/// Selection of which USART is to be used.
#[derive(Clone, Copy)]
pub enum UsartNum {
    usart0,
}


/// Selection of synchronous or asynchronous modes for USART.
#[derive(Clone, Copy)]
pub enum UsartModes {
    norm_async,
    dou_async,
    master_sync,
    slave_sync,
}


/// Selection of the parity mode for USART.
#[derive(Clone, Copy)]
pub enum UsartParity {
    no,
    even,
    odd,
}

/// Selection of the Amount of Data Bits to be transferred or recieved through USART.
#[derive(Clone, Copy)]
pub enum UsartDataSize {
    five,
    six,
    seven,
    eight,
    nine,
}

/// Selection of number of stop bits for USART data.
#[derive(Clone, Copy)]
pub enum UsartStop {
    one,
    two,
}

/// Selection of the clock parity mode.
#[derive(Clone, Copy)]
pub enum UsartPolarity {
    output_rise,
    input_rise,
}


