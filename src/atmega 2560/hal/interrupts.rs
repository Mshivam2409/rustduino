use core;

pub struct Global_Interrupts {  //struct for global interrupts
   SREG:u8,                 
}

//in section 2-(Overview) point 7.4 about (SREG) 
impl Global_Interrupts {
   //returns new Global_Interrupts
    pub unsafe fn new() -> &'static mut Global_Interrupts {
        &mut *(0x5F as *mut Global_Interrupts)   //sets address for SREG register
    }


//disable global interrupts
//also known as CLI
    pub fn disable_interrupts(&mut self) {
        unsafe {
            
           let mut ctrl_sreg=core::ptr::read_volatile(&self.SREG);
           ctrl_sreg &= !(1<<7);                                    //sets I-bit of SREG 0
           core::ptr::write_volatile(&mut self.SREG,ctrl_sreg);
        }
    }



    pub fn enable_interrupts(&mut self){
        unsafe{
        let mut ctrl_sreg=core::ptr::read_volatile(&self.SREG);
        ctrl_sreg |=(1<<7);                                        //sets I_bit of SREG 1
        core::ptr::write_volatile(&mut self.SREG,ctrl_sreg);
        }
    }
    //enable global interrupts
    //also known as SEI
}