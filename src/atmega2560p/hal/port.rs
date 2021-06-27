use core::ptr::{read_volatile, write_volatile};
 
#[derive(Clone,Copy)]
/// Represents the name of the port , can vary from A-L.
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

///Each Port pin consists of 3 registers : PINx, DDRx, PORTx .
///These registors control the ports.


///Registers and bit references used here are written in general form.
///A lower case “x” represents the numbering letter for the port, and a lower case “n” represents the bit number.
///For example, PORTB3 for bit no. 3 in Port B, here documented generally as PORTxn

///     DDRx:*Data direction register*
///         The DDxn bit in the DDRx Register selects the direction of this pin.
///         DDxn = 1 => Pin x is configured as output.
///         DDxn = 0 => Pin x is configured as input.
/// 
///     PORTx: *Data register*
///     If PORTxn is written logic one when the pin is
///         configured as an input pin, the pull-up resistor is activated. To switch the
///         pull-up resistor off, PORTxn has to be written logic zero or the pin has to be
///         configured as an output pin. The port pins are tri-stated when reset condition
///         becomes active, even if no clocks are running.
/// 
///         PORTxn is written logic one when the pin is configured as an output pin,
///         the port pin is driven high (one). If PORTxn is written logic zero when the pin
///         is configured as an output pin, the port pin is driven low (zero).
/// 
///     PINx:*port input pins*
///         Writing a logic one to PINxn toggles the value of PORTxn, independent on the value of DDRxn.
///
/// Section 13.2.1 and 13.2.2 of ATmega2560P datasheet.
struct Port{
    pin:u8,
    ddr:u8,
    port:u8,  
}

impl Port {
    /// Returns mutable reference to the `Port` given `PortName`.
    /// Section 13.4 of ATmega328P datasheet.
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
pub struct Pin {
    port:*mut Port,
    pin:u8,
}
pub enum IOMode{
    Input,
    Output,
}

impl Pin {

    ///Return a pin for the given port name and pin number

    pub fn new(port: PortName,pin: u8)-> Option<Pin>
    {
        Port::new(port).pin(pin)
    }


    /// Change pin mode to input or output by changing the DDr pin.
    /// If DDxn is written logic one, Pxn is configured
    ///as an output pin.
    /// If DDxn is written logic zero, Pxn is configured as an input pin.
    /// Section 13.2 of Atmega2605 datasheet


    pub fn set_pin_mode(&mut self,mode: IOMode){

        //read the DDxn register

        let mut ddr_val=unsafe{
            read_volatile(&mut(*self.port).ddr)
        };

        //calculate the value to be written to DDxn register

        ddr_val&=!(0x1<<self.pin);
        ddr_val |=match mode{
            IOMode:: Input=>0x0,
            IOMode:: Output=>0x1<<self.pin,

        };

        // write the value to DDxn register
        unsafe{
            write_volatile(&mut(*self.port).ddr,ddr_val)
        }



    }


    ///Toggles the value of PORTxn by writing one to PINxn ,independent of the value of DDRxn.
    pub fn toggle(&mut self) {
        unsafe {
             write_volatile(&mut (*self.port).pin, 0x1 << self.pin) 
         }
    }
    ///set the pin to high
    pub fn high(&mut self) {
        // Check if pin number is valid.
        if self.pin >= 8 {
            return;
        }
        unsafe{
            //reading the value of PORTxn.
            let p = unsafe {
                 read_volatile(&mut (*self.port).port) 
            };
            // Read the DDRxn register.
            let mut ddr_value = unsafe {
                 read_volatile(&mut (*self.port).ddr)
            };
            //toggling the value of PORTxn, if it isn't set to high.
            if p == 0 && ddr_value == (0x1 << self.pin){
                self.toggle();
            }
        }
    }
    ///set the pin to low
    pub fn low(&mut self) {
        // Check if pin number is valid.
        if self.pin >= 8 {
            return;
        }
        unsafe{
            //reading the value of PORTxn.
            let p = unsafe {
                 read_volatile(&mut (*self.port).port) 
            };
            // Read the DDRxn register.
            let mut ddr_value = unsafe {
                 read_volatile(&mut (*self.port).ddr)
            };
            //toggling the value of PORTxn, if it isn't set to low.
            if p != 0 && ddr_value == (0x1 << self.pin) {
                self.toggle();
            }
        }
    }

    pub fn output(&mut self){
        self.set_pin_mode(IOMode::Output);
    }
}


