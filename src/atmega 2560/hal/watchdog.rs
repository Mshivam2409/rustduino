use core;

// Section 12.5 of manual

#[repr(C,packed)]
pub struct Watchdog {
   MCUSR:u8,
   // memory addresses not clear from the Manual as of now
   // padding is not correct surely
   pad_1:[char;10],
   SREG:u8,            // This is the status register to be used in global interrupts
   WDTCSR:u8
}

// This is the implementation without any use of padding in case of doubts
/*
#[repr(C,packed)]
pub struct MCUSr {
    mcusr:u8,
}*/
/*
impl MCUSr {
    pub unsafe fn new()->&'static mut MCUSr{
      &mut *(0x34 as *mut MCUSr)
    }
    pub fn clear_WDRF(&mut self){
       unsafe{
             let mut mcusr=core::ptr::read_volatile(&self.mcusr);
             mcusr &= !(1<<)
       }
    }
}*/

use core::arch::arm::__nop;
mod interrupt;

impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {
        &mut *(0x54 as *mut Watchdog)    // memory address to check
        // &mut *(0x60 as *mut Watchdog)
    }


    pub fn interrupt_toggle(&mut self) {
        // If the WDIE bit is enabled it will be disabled otherwise enabled
        // A new instance of the structure is created first
        Watchdog *ptr=new();

        let mut wdtcsr = core::ptr::read_volatile(&mut self.WDTCSR);
        
        if wdtcsr & 0xBF == wdtcsr { wdtcsr = wdtcsr | 0x40; }
        else { wdtcsr = wdtcsr & 0xBF }
        
        core::ptr::write_volatile(&mut self.WDTCSR,wdtcsr);
    } 


    pub fn disable(&mut self) {
        unsafe {
           // old code
           
           /*let mcusr=MCUSr::new();
           mcusr.clear_WDRF();
           let mut WDTCSR=core::ptr::read_volatile(&self.WDTCSR);
           WDTCSR |= ((1<<4) |(1<<6));
           core::ptr::write_volatile(&mut self.WDTCSR,WDTCSR);
           core::ptr::write_volatile(&mut self.WDTCSR,0X00);*/

           // new code
           
           Watchdog *ptr = new();
           // Disable interrupts
           // disable watchdog interrupts
           ptr.interrupt_toggle();
           // disable global interrupts
           let itr = interrupt::Status::new();
           itr.disable();

           // No need of this it is done later
           /*
           // Reset the Watchdog to initial settings by clearing WDRF bit
           let ctrl_mcusr = core::ptr::read_volatile(&mut self.MCUSR);
           ctrl_mcusr = ctrl_mcusr & 0x17;
           core::ptr::write_volatile(&mut self.MCUSR,ctrl_mcusr);
           */

           // Then we set WDCE bit of wdtcsr register as 1
           let mut wdtcsr = core::ptr::read_volatile(&mut self.WDTCSR);
           wdtcsr = wdtcsr | 0x10;
           core::ptr::write_volatile(&mut self.WDTCSR,wdtcsr );
           // Then we change the WDRF bit of mcusr register as 0
           let mut mcusr = core::ptr::read_volatile(&mut self.MCUSR);
           mcusr = mcusr & 0xF7;
           core::ptr::write_volatile(&mut self.MCUSR,mcusr );
           // Then we have to change the WDE bit of wdtcsr register to 0
           wdtcsr = wdtcsr & 0xF7
           core::ptr::write_volatile(&mut self.WDTCSR,wdtcsr);
           
           // Enable interrupts
           // enable watchdog interrupts
           ptr.interrupt_toggle();
           // enable global interrupts
           itr.enable();
        }
    }
}
