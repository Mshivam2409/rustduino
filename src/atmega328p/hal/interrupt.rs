use core;
// #[no(main)]

/// section 6.3.1 of manual
pub struct Interrupt {
    sreg: u8,
}

impl Interrupt {
    pub unsafe fn get() -> &'static mut Interrupt {
        &mut *(0x5F as *mut Interrupt)
    }

    /// Getting address of the sreg register.
    /// Reading value stored in sreg.\
    /// Toggling 8th bit to 0; 7F in binary is 01111111.
    /// Writing value back in sreg
    pub fn disable(&mut self) {
        unsafe {
            // let mut interrupt = get();
            let mut ctrl_sreg = core::ptr::read_volatile(&self.sreg);
            ctrl_sreg &= 0x7F;
            core::ptr::write_volatile(&mut self.sreg, ctrl_sreg);
        }
    }

    /// Getting address of the sreg register.
    /// Reading value stored in sreg
    /// Toggling 8th bit to 1; 7F in binary is 11111111
    /// Writing value back in sreg
    pub fn enable(&mut self) {
        unsafe {
            // let mut interrupt = get();
            let mut ctrl_sreg = core::ptr::read_volatile(&self.sreg);
            ctrl_sreg &= 0xFF;
            core::ptr::write_volatile(&mut self.sreg, ctrl_sreg);
        }
    }
}
