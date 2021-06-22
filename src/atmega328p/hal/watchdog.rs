use core;
use core::arch::arm::__nop;

#[repr(C, packed)]
pub struct Watchdog {
   char mcusr,
       //memory address is not clear, padding needs to be added
   char wdtcsr,

    
}

impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {

        &mut *(0x35 as *mut Watchdog)
    }

    pub fn disable(&mut self) {
        unsafe {
            cortex_m::interrupt::disable; //disable interrupts

            let mut ctrl_mcusr = core::ptr::read_volatile(&self.mcusr); //  clear wdrf of mcusr
            core::ptr::write_volatile(&mut self.mcusr, ctrl_mcusr & ~(1 << 3) );
        

            let mut ctrl_wdtcsr = core::ptr::read_volatile(&self.wdtcsr);
            core::ptr::write_volatile(&mut self.wdtcsr,ctrl_wdtcsr| (1<<4));

            let mut ctrl_wdtcsr = core::ptr::read_volatile(&self.wdtcsr);
            core::ptr::write_volatile(&mut self.wdtcsr,ctrl_wdtcsr| (1<<3));

            core::ptr::write_volatile(&mut self.wdtcsr, 0x00);
            cortex_m::interrupt::enable; //enable interrupts
        }
    }
}
