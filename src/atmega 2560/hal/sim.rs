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

impl Sim {
    pub unsafe fn new() -> &'static mut Sim {
        // Creates a reference to the mutable structure to control the hardware of SIM.
        &mut *(0x61 as *mut Sim)
    }

    pub fn enable_clock(&mut self) {
        unsafe {
            // Creates a new instance for the Sim structure.
            Sim *ptr = new();
            
            
        }
    }
}