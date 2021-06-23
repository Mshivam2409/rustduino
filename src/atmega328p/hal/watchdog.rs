use core;

/// struct to control the watchdog timer
/// consists of two 8 bit registers

pub struct Watchdog { 
   MCUSR: u8,
   WDTCSR: u8,
}

impl Watchdog {
    pub unsafe fn new() -> &'static mut Watchdog {

        &mut *(0x55 as *mut Watchdog)
    }

    fn reset_watchdog() {
        unsafe (
            let mut watchdog = new();
            let mut ctrl_mcusr = core::ptr::read_volatile(watchdog.MCUSR);
            ctrl_mcusr &= 0x7;
            core::ptr::write_volatile(watchdog.MCUSR, ctrl_mcusr);
        )
        
    }

    pub fn disable(&mut self) {
        unsafe {
            /// disabling interrupt 
            Interrupt::disable(); 
            /// reseting watchdog timer
            reset_watchdog();
            /// disabling watchdog
            let mut watchdog = new();
            let mut ctrl_wdtcsr = core::ptr::read_volatile(watchdog.WDTCSR);
            ctrl_wdtcsr |= 0x18;
            core::ptr::write_volatile(watchdog.WDTCSR, ctrl_wdtcsr);
            core::ptr::write_volatile(watchdog.wdtcsr, 0x00);
            /// enabling interrupts again
            Interrupt::enable();
        }
    }
}
