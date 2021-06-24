use core;
 ///struct for global interrupts
pub struct GlobalInterrupts {  
   sreg:u8,                 
}

///in section 2-(Overview) point 7.4 about (SREG) 
impl GlobalInterrupts {
   ///returns new Global_Interrupts
    pub unsafe fn new() -> &'static mut GlobalInterrupts {
        &mut *(0x5F as *mut Global_Interrupts)   //sets address for SREG register
    }


///disable global interrupts
///also known as CLI
 ///sets I-bit of SREG 0
    pub fn disable(&mut self) {
        unsafe {
            
           let mut ctrl_sreg=core::ptr::read_volatile(&self.sreg);
           ctrl_sreg &= !(1<<7);                                   
           core::ptr::write_volatile(&mut self.sreg,ctrl_sreg);
        }
    }



    pub fn enable(&mut self){
        unsafe{
        let mut ctrl_sreg=core::ptr::read_volatile(&self.sreg);
        ctrl_sreg |=(1<<7);                                        //sets I_bit of SREG 1
        core::ptr::write_volatile(&mut self.sreg,ctrl_sreg);
        }
    }
    ///enable global interrupts
    ///also known as SEI
}