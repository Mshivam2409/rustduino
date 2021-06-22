#[repr(C,packed)]
pub struct Watchdog {
   WDTCSR:u8,
}

#[repr(C,packed)]
pub struct MCUSr {
    mcusr:u8,
}

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
}

use core::arch::arm::__nop;

impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {
        &mut *(0x60 as *mut Watchdog)
    }


    pub fn disable(&mut self) {
        unsafe {
            let mcusr=MCUSr::new();
            mcusr.clear_WDRF();
           let mut WDTCSR=core::ptr::read_volatile(&self.WDTCSR);
           WDTCSR |= ((1<<4) |(1<<6));
           core::ptr::write_volatile(&mut self.WDTCSR,WDTCSR);
           core::ptr::write_volatile(&mut self.WDTCSR,0X00);

        }
    }
}
