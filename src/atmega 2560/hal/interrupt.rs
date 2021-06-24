use core;
use core::arch::arm::__nop;

// Section 7.4 of the manual

// WE NEED TO DISABLE THE GLOBAL INTERRUPTS NOT THE INTERRUPTS RELATED TO WATCHDOG
/*
#[repr(C,packed)]
pub struct Watchdog {
   MCUSR:u8,
   // memory addresses not clear from the Manual as of now
   // padding is not correct surely
   pad_1:[char;4],
   WDTCSR:u8
}

impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {
        &mut *(0x55 as *mut Watchdog)    // memory address to check
        // &mut *(0x60 as *mut Watchdog)
    }

    pub fn interrupt_toggle(&mut self) {
        // If the WDIE bit is enabled it will be disabled otherwise enabled
        // A new instance of the structure is created first
        Watchdog *ptr=new();

        let wdtcsr = core::ptr::read_volatile(&mut self.WDTCSR);
        
        if wdtcsr & 0xBF == wdtcsr { wdtcsr = wdtcsr | 0x40; }
        else { wdtcsr = wdtcsr & 0xBF }
        
        core::ptr::write_volatile(&mut self.WDTCSR,wdtcsr);
    }
}
*/

#[repr(C,packed)]
pub struct Status {
   SREG:u8
}

impl Status {
    pub unsafe fn new() -> &'static mut Status {
        // Return a mutable static reference to a instance of structure  
        &mut *(0x5F as *mut Status) 
    }

    pub fn disable(&mut self) {
        // Create a new instance of Status registers
        Status *ptr = new();
        // Set the global interrupt bit as 0
        let mut sreg = core::ptr::read_volatile(&mut self.SREG);
        sreg = sreg & 0x7F; 
        core::ptr::write_volatile(&mut self.SREG, sreg); 
    }

    pub fn enable(&mut self) {
        // Create a new instance of Status registers
        Status *ptr = new();
        // Set the global interrupt bit as 0
        let mut sreg = core::ptr::read_volatile(&mut self.SREG);
        sreg = sreg | 0x80; 
        core::ptr::write_volatile(&mut self.SREG, sreg); 
    }
}
