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

  

pub fn name(&self)->PortName{
    let address = (self as *const Port) as usize;
    match address{
        0x20=> PortName::A,
        0x23=> PortName::B,
        0x26=> PortName::C,
        0x29=> PortName::D,
        0x2C=> PortName::E,
        0x2F=> PortName::F,
        0x32=> PortName::G,
        0x100=> PortName::H,
        0x103=> PortName::J,
        0x106=> PortName::K,
        0x109=> PortName::L,
        _=>unreachable!()
    
    }

}
}
