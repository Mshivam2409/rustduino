use core;

mod interrupts;
//use interrupts to enable/disable global interrupts

//struct for Watchdog
#[repr(C,packed)]
pub struct WatchDog {  
    WDTCSR:u8,
}

#[repr(C,packed)]
pub struct MCUSR {
    mcusr:u8,
}

impl MCUSR {
    pub unsafe fn new()->&'static mut MCUSR{
      &mut *(0x54 as *mut MCUSR)
    }

    //clear WDRF bit in MCUSR register 
    pub fn clear_WDRF(&mut self){
       unsafe{
             let mut mcusr=core::ptr::read_volatile(&self.mcusr);
             mcusr &= !(1<<3);
             core::ptr::write_volatile(&mut self.mcusr,mcusr & !(1<<3));
       }
    }
}

use core::arch::arm::__nop;

impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog { //returns watchdog's ref
         &mut *(0x60 as *mut Watchdog)
    }

//disables watchdog 
    pub fn disable(&mut self) {
        unsafe {
            //disable global interrupts
           let inter=interrupts::Global_Interrupts::new();
           inter.disable_interrupts();


           __nop();

          //clears WDRF in MCUSR
           let mcusr=MCUSR::new();
           mcusr.clear_WDRF();

           
           let mut WDTCSR=core::ptr::read_volatile(&self.WDTCSR);

           WDTCSR |= ((1<<4) |(1<<3));
          //sets WDCE for changing WDE
           core::ptr::write_volatile(&mut self.WDTCSR,WDTCSR);

           //sets every bit to 0 including WDE and WDIE
           core::ptr::write_volatile(&mut self.WDTCSR,0X00);
           __nop();

           //enable interrupts
           inter.enable_interrupts();
        }
    }
}
