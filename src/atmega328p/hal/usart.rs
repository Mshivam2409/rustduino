//! USART implementation


/// Crates which would be used in the implementation.
/// We will be using standard volatile and bit_field crates now for a better read and write.
use core::ptr::{read_volatile,write_volatile};
use bit_field::BitField;
use volatile::Volatile;
use crate::avr::__nop;
use crate::rustduino::atmega328p::hal::interrupts;
use crate::rustduino::atmega328p::hal::port;
use crate::rustduino::atmega328p::hal::power;
use crate::delay::{delay_s,delay_ms,delay_us};

/// Some useful constants regarding bit manipulation for USART.
/// Position of clock mode adjuster (xck) bit.
const usart0_xck : u8 = 4;
/// Position of Transmission bit for various USART.
const usart0_td  : u8 = 1;
/// Position of Reciever bit for various USART.
const usart0_rd  : u8 = 0;
/// System Clock Crystal Oscillator Frequency in mHz.
const f_osc : f64 = 1.0000;
const multiply : i32 = 1000000;


/// Selection of which USART is to be used.
#[derive(Clone, Copy)]
pub enum UsartNum {
    usart0,
}


/// Selection of synchronous or asynchronous modes for USART.
#[derive(Clone, Copy)]
pub enum UsartModes {
    norm_async,
    dou_async,
    master_sync,
    slave_sync,
}


/// Selection of the parity mode for USART.
#[derive(Clone, Copy)]
pub enum UsartParity {
    no,
    even,
    odd,
}

/// Selection of the Amount of Data Bits to be transferred or recieved through USART.
#[derive(Clone, Copy)]
pub enum UsartDataSize {
    five,
    six,
    seven,
    eight,
    nine,
}

/// Selection of number of stop bits for USART data.
#[derive(Clone, Copy)]
pub enum UsartStop {
    one,
    two,
}

/// Selection of the clock parity mode.
#[derive(Clone, Copy)]
pub enum UsartPolarity {
    output_rise,
    input_rise,
}


/// This structure contains various registers needed to control usart communication
/// through ATMEGA320P device.
/// USART0 is controlled by a total of 6 registers stored through this structure. 
#[repr(C, packed)]
pub struct Usart {
    ucsra : Volatile<u8>,
    ucsrb : Volatile<u8>,
    ucsrc : Volatile<u8>,
    _pad  : Volatile<u8>,                                    // Padding to look for empty memory space.
    ubrrl : Volatile<u8>,
    ubrrh : Volatile<u8>,
    udr   : Volatile<u8>,    
}


/// Various implementation functions for the USART protocol.
impl Usart 
{
    /// This creates a new memory mapped structure of the USART0 for it's control.
    pub unsafe fn new(num : UsartNum) -> &'static mut Usart
     {
        match num 
        {
           UsartNum::usart0 =>{ &mut *(0xC0 as *mut Usart)  },
        }
    }

    /// Function to disable global interrupts for smooth non-interrupted functioning of USART.
    fn disable(&mut self) 
    {
        unsafe 
        {
            // Disable global interrupts.
            interrupt::Interrupt::disable(&mut interrupts::Interrupt::new());
        }
    }

    /// Function to re-enable global interrupts.
    fn enable(&mut self) 
    {
        unsafe
        {
            // Enable global interrupts.
            interrupt::Interrupt::enable(&mut interrupt::Interrupt::new());
        }
    }

     /// This function will return the Number of the USART according to the address.
     fn get_num(&self) -> UsartNum {
        let address = (self as *const Usart) as u8;             // Gets address of usart structure.
        match address {
            // Return the number of USART used based on the address read.
            0xC0  => UsartNum::usart0,
            _     => unreachable!(),
        }
    }

    /// Function to get the port containing bits to
    /// manipulate Recieve,Transmit and XCK bit of the particular USART.
    fn get_port(&self) -> port::Port {
        let num : UsartNum = self.get_num();
        unsafe {
            match num {
                UsartNum::usart0 => { port::Port::new(D) },
            }
        }
    }

    /// Function to return the index of xck bit in the port.
    fn get_xck(&self) -> u8 {
        let num : UsartNum = self.get_num();
        match num {
            UsartNum::usart0 => { usart0_xck },
        }
    }
    
    /// Function to return the index of transmit bit in the port.
    fn get_td(&self) -> u8 {
        let num : UsartNum = self.get_num();
        match num {
            UsartNum::usart0 => { usart0_td },
        }
    }

    /// Function to return the index of recieve bit in the port.
    fn get_rd(&self) -> u8 {
        let num : UsartNum = self.get_num();
        match num {
            UsartNum::usart0 => { usart0_rd },
        }
    }

    /// Function to check the mode of the USART.
    /// Returns 0 for asynchronous and 1 for synchronous.
    fn get_mode(&self) -> bool {
        let num : UsartNum = self.get_num();
        let mut src = self.ucsrc.read();
        src = src & (1<<6);
        if src==0 { return false; }
        else { return true; }
    }

    /// Function to set the clock polarity mode which is of use in the
    /// recieve and transmission implementation of USART.
    pub fn set_polarity(&self,mode : UsartPolarity) {
        if(self.get_mode()==false) { 
            self.ucsrc.update( |src| {
                src.set_bit(0,false);
            }); 
        }
        else {
            match mode {
                UsartPolarity::output_rise => { 
                    self.ucsrc.update( |src| {
                        src.set_bit(0,false);
                    });
                },
                UsartPolarity::input_rise => { 
                    self.ucsrc.update( |src| {
                        src.set_bit(0,true);
                    });
                },
            }
        }
    }
     /// Function to set various modes of the USART which is activated.
     pub fn mode_select(&mut self,mode : UsartModes) {
        match mode {
            UsartModes::norm_async                                  // Puts the USART into asynchronous mode.
            | UsartModes::dou_async => {
                    self.ucsrc.update( |src| {
                        src.set_bit(6,false);
                        src.set_bit(7,false);
                    }); 
            },
            UsartModes::master_sync
            | UsartModes::slave_sync => {                           // Puts the USART into synchronous mode.
                    self.ucsrc.update( |src| {
                        src.set_bit(6,true);
                        src.set_bit(7,false);
                    });
                    self.ucsra.update( |sra| {
                        sra.set_bit(1,false);
                    });
            },
        }
        match mode {
            UsartModes::norm_async => {                              // Keeps the USART into normal asynchronous mode.
                    self.ucsra.update( |sra| {
                        sra.set_bit(1,false);
                    });
            },
            UsartModes::dou_async => {                               // Puts the USART into double speed asynchronous mode.
                    self.ucsra.update( |sra| {
                        sra.set_bit(1,true);
                    });
            },
            UsartModes::master_sync => {                             // Puts the USART into master synchronous mode
                    let port : (port::Port) = self.get_port();
                    let xck : u8 = self.get_xck();
                    unsafe {
                        write_volatile(&mut port.ddr,port.ddr |= (1 << xck));
                    }
                    // port.ddr.update( |ddr| {
                    //     ddr.set_bit(xck,true);
                    // });       
            },
            UsartModes::slave_sync => {                              // Puts the USART into slave synchronous mode
                    let port : (port::Port) = self.get_port();
                    let xck : u8 = self.get_xck();
                    unsafe {
                        write_volatile(&mut port.ddr,port.ddr &= !(1 << xck));
                    }    
                    // port.ddr.update( |ddr| {
                    //     ddr.set_bit(xck,false);
                    // });
            },
        }
    }

}