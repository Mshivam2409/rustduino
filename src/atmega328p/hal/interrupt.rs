use core;
// #[no(main)]
pub struct Interrupt {
    SREG: u8;
}

impl Interrupt {
    pub unsafe fn get() -> &'static mut Interrupt {
        & mut *(0x5F as *mut Interrupt)
    }

    pub fn disable() {
        unsafe{
            /// getting address of the SREG register.
            let mut interrupt = get();
            /// reading value stored in SREG
            let mut ctrl_sreg = core::ptr::read_volatile(interrupt.SREG);
            /// toggling 8th bit to 0; 7F in binary is 01111111
            ctrl_sreg &= 0x7F; 
            /// writing value back in SREG
            core::ptr::write_volatile(&mut interrupt.SREG, ctrl_sreg);
        }
    }

    pub fn enable(&mut self) {
        unsafe{
            /// getting address of the SREG register.
            let mut interrupt = get();
            /// reading value stored in SREG
            let mut ctrl_sreg = core::ptr::read_volatile(interrupt.SREG);
            /// toggling 8th bit to 1; 7F in binary is 11111111
            ctrl_sreg &= 0xFF; 
            /// writing value back in SREG
            core::ptr::write_volatile(interrupt.SREG, ctrl_sreg);
        }
    }
    
}
