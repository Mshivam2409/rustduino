use core;
use crate::atmega328p::hal::interrupt;

/// Struct to control the watchdog timer 10.9 of the manual.
/// Consists of two 8 bit registers
pub struct Watchdog { 
   mcusr: u8,
   wdtcsr: u8, 
}

impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {
        &mut *(0x55 as *mut Watchdog)
    }

    // pub fn reset_watchdog(&mut self) {
    //     unsafe {
    //         // let mut watchdog = new();
    //         let mut ctrl_mcusr = core::ptr::read_volatile(&self.mcusr);
    //         ctrl_mcusr &= 0x7;
    //         core::ptr::write_volatile(&mut self.mcusr, ctrl_mcusr);
    //     }
        
    // }

    /// disabling interrupt 
    /// reseting watchdog timer
    /// disabling watchdog
    /// enabling interrupts again
    pub fn disable(&mut self) {
        unsafe {
            interrupt::Interrupt::disable(&mut interrupt::Interrupt::get());
            // reset_watchdog(); 
            let mut ctrl_mcusr = core::ptr::read_volatile(&self.mcusr);
            ctrl_mcusr &= 0x7;
            core::ptr::write_volatile(&mut self.mcusr, ctrl_mcusr);
            // let mut watchdog = new();
            let mut ctrl_wdtcsr = core::ptr::read_volatile(&self.wdtcsr);
            ctrl_wdtcsr |= 0x18;
            core::ptr::write_volatile(&mut self.wdtcsr, ctrl_wdtcsr);
            core::ptr::write_volatile(&mut self.wdtcsr, 0x00); 
            interrupt::Interrupt::enable(&mut interrupt::Interrupt::get());
        }
    }
}
