use core;
use core::arch::arm::__nop;

// Section 13.4 of the manual

#[derive(Clone,Copy)]
pub enum PortName {
    // We will control all the important features of a given Port by the user.
    // In ATMEGA2560P we are having 11 ports from A to L leaving the character I
    A, B, C,
    D, E, F,
    G, H, J,  // No PORT I available be careful 
    K, L
}

#[repr(C, packed)]
pub struct Port {
    // For each port we have 3 controlling registers
    // they are represented in order as given below
    pin:u8,
    ddr:u8,
    port:u8 
}

pub struct Pin {
    port: *mut Port,
    pin: usize
}

#[repr(C,packed)]
pub struct Gpio {

}


impl Port {
     pub unsafe fn new(p:PortName) -> &'static mut Port {
         unsafe {
              &mut *match p {
                   PortName::A => &mut *(0x20 as *mut Port)
                   PortName::B => &mut *(0x23 as *mut Port)
                   PortName::C => &mut *(0x26 as *mut Port)
                   PortName::D => &mut *(0x29 as *mut Port)
                   PortName::E => &mut *(0x2C as *mut Port)
                   PortName::F => &mut *(0x2F as *mut Port)
                   PortName::G => &mut *(0x32 as *mut Port)
                   PortName::H => &mut *(0x100 as *mut Port)
                   PortName::J => &mut *(0x103 as *mut Port)
                   PortName::K => &mut *(0x106 as *mut Port)
                   PortName::L => &mut *(0x109 as *mut Port)
                   _ => unreachable!();
              }
         }
     }

     pub fn 
}

impl Pin {

}

impl Gpio {

}