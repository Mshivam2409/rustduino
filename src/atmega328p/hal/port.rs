//! General Digital I/O Implementation

pub enum PortName {
    B,
    C,
    D,
}

pub struct Port {
    pin: u8,
    ddr: u8,
    port: u8,
}

impl Port {
    pub fn new(port_name: PortName) -> &'static mut Port {
        //! Return mutable reference to a Port
        unsafe {
            &mut *match port_name {
                PortName::B => 0x23 as *mut Port,
                PortName::C => 0x26 as *mut Port,
                PortName::D => 0x29 as *mut Port,
            }
        }
    }

    pub fn name(&self) -> PortName {
        //! Returns PortName of the port based on its address.
        // Get address of port as usize
        let addr = (self as *const Port) as usize;

        // Return PortName based on address
        match addr {
            0x23 => PortName::B,
            0x26 => PortName::C,
            0x29 => PortName::D,
            _ => unreachable!(),
        }
    }
}
