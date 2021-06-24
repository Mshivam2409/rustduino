use core;
use core::arch::arm::__nop;

#[repr(C, packed)]
pub struct Sim { 
    clkpr:u8,
    // memory addresses not clear from the Manual as of now
    // padding is not correct surely
    pad_1:[char;4],
    osccal:u8
}

