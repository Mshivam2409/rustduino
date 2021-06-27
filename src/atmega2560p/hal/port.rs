use core;
 
#[derive(Clone,Copy)]
pub enum PortName {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    J,
    K,
    L,
}
struct Port{
    pin:u8,
    ddr:u8,
    port:u8,  
}

impl Port {
    pub unsafe fn new(name: PortName) -> &'static mut Port {
       match name{
           PortName::A=> { &mut *(0x20 as *mut Port) }
           PortName::B=> { &mut *(0x23 as *mut Port) }
           PortName::C=> { &mut *(0x26 as *mut Port) }
           PortName::D=> { &mut *(0x29 as *mut Port) }
           PortName::E=> { &mut *(0x2C as *mut Port) }
           PortName::F=> { &mut *(0x2F as *mut Port) }
           PortName::G=> { &mut *(0x32 as *mut Port) }
           PortName::H=> { &mut *(0x100 as *mut Port) }
           PortName::J=> { &mut *(0x103 as *mut Port) }
           PortName::K=> { &mut *(0x106 as *mut Port) }
           PortName::L=> { &mut *(0x109 as *mut Port) }
           _ => unreachable!()
       } 
    }

    
}