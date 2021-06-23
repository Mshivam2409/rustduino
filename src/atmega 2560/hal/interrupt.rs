use core;
use core::arch::arm::__nop;

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
