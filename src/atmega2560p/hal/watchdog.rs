use core;

use crate::atmega2560p::hal::interrupts;

///use interrupts to enable/disable global interrupts
///section 28.6 from manual
///struct for WatchDog
pub struct WatchDog {  
    mcusr:u8,
    _pad:[u8;5],
    wdtcsr:u8,
}


impl WatchDog {
    pub unsafe fn new() -> &'static mut WatchDog { //returns watchdog's ref
         &mut *(0x54 as *mut WatchDog)
    }

    ///disables watchdog 
    pub fn disable(&mut self) {
        unsafe {
            //disable global interrupts
          interrupts::GlobalInterrupts::disable(&mut interrupts::GlobalInterrupts::new());

          //clears WDRF in MCUSR
          let mut mcusr=core::ptr::read_volatile(&self.mcusr);
          mcusr &= !(1<<3);
          core::ptr::write_volatile(&mut self.mcusr,mcusr);
           

           let mut wdtcsr=core::ptr::read_volatile(&self.wdtcsr);
           wdtcsr |= (1<<4) |(1<<3);
           //sets WDCE for changing WDE
           core::ptr::write_volatile(&mut self.wdtcsr,wdtcsr);
           //sets every bit to 0 including WDE and WDIE
           core::ptr::write_volatile(&mut self.wdtcsr,0x00);
           //enable interrupts
           interrupts::GlobalInterrupts::enable(&mut interrupts::GlobalInterrupts::new());
        }
    }
}
