//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Richa Prakash Sachan and Kshitij Kaithal, Indian Institute of Technology Kanpur
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Affero General Public License as published
//     by the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Affero General Public License for more details.
//
//     You should have received a copy of the GNU Affero General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>

/// Other source code files to be used
use crate::rustduino::atmega328p::hal::interrupt;
use crate::rustduino::atmega328p::hal::port;
use crate::rustduino::atmega328p::hal::power;

/// Crates which would be used in the implementation.
/// We will be using standard volatile and bit_field crates now for a better read and write.
use crate::delay::delay_ms;
use bit_field::BitField;
use core::ptr::{read_volatile,write_volatile};
use core::{f64,u32,u8};
use volatile::Volatile;


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
                    
            },
            UsartModes::slave_sync => {                              // Puts the USART into slave synchronous mode
                    let port : (port::Port) = self.get_port();
                    let xck : u8 = self.get_xck();
                    unsafe {
                        write_volatile(&mut port.ddr,port.ddr &= !(1 << xck));
                    }    
                  
            },
        }
    }

    /// Function to set the power reduction register so that USART functioning is allowed. 
    fn set_power(&self,num : UsartNum) 
    {
        unsafe {
            let pow : (power::Power) = power::Power::new();
                }     
        match num {
            UsartNum::usart0 => { 
                unsafe {
                    write_volatile(&mut pow.prr0,pow.prr0 &= !(1 << 1));
                }
              
            },
            
        }
    }


    /// Sets the interrupt bits in UCSRB so that ongoing
    /// data transfers can be tracked.
    fn check(&self) {
        self.ucsrb.update( |srb| {
              srb.set_bit(6,true);
              srb.set_bit(7,true);
        });
    }


    /// Clock Generation is one of the initialization steps for the USART.
    /// If the USART is in Asynchronous mode or Master Synchronous mode then a internal
    /// clock generator is used while for Slave Synchronous mode we will use a external 
    /// clock generator.
    /// Set the baud rate frequency for USART.
    /// Baud rate settings is used to set the clock for USART.
    fn set_clock(&self,baud : i64,mode : UsartModes) {
        match mode {
            UsartModes::norm_async => { 
                let mut ubrr : u32 = ((f_osc*multiply)/(16*baud))-1; 
            },
            UsartModes::dou_async => {
                let mut ubrr : u32 = ((f_osc*multiply)/(8*baud))-1;
            },
            UsartModes::master_sync => {
                let mut ubrr : u32 = ((f_osc*multiply)/(2*baud))-1;
            },
            _ => unreachable!(),
        }
        self.ubrrl.set_bits(0..8,ubrr);
        self.ubrrh.set_bits(0..4,(ubrr>>8));
    }
    
/// Function to set the limit of data to be handled by USART.
fn set_size(&self,size : UsartDataSize) {
    match size {
        UsartDataSize::five
        | UsartDataSize::six
        | UsartDataSize::seven
        | UsartDataSize::eight => { 
            self.ucsrb.update( |srb| {
                srb.set_bit(2,false);
            });       
        },
        UsartDataSize::nine => { 
            self.ucsrb.update( |srb| {
                srb.set_bit(2,true);
            });
        },
    }
    match size {
        UsartDataSize::five
        | UsartDataSize::six => { 
            self.ucsrc.update( |src| {
                src.set_bit(2,false);
            });       
        },
        UsartDataSize::nine
        | UsartDataSize::seven
        | UsartDataSize::eight => { 
            self.ucsrc.update( |src| {
                src.set_bit(2,true);
            });
        },
    }
    match size {
        UsartDataSize::five
        | UsartDataSize::seven => { 
            self.ucsrc.update( |src| {
                src.set_bit(1,false);
            });       
        },
        UsartDataSize::nine
        | UsartDataSize::six
        | UsartDataSize::eight => { 
            self.ucsrc.update( |src| {
                src.set_bit(1,true);
            });
        },
    }
}


    /// Function to set the parity bit in the frame of USART.
    fn set_parity(&self,parity : UsartParity) {
        match parity {
            UsartParity::no => { 
                self.ucsrc.update( |src| {
                    src.set_bit(4,false);
                    src.set_bit(5,false);
                });
            },
            UsartParity::even => { 
                self.ucsrc.update( |src| {
                    src.set_bit(4,false);
                    src.set_bit(5,true);
                });
            },
            UsartParity::odd => { 
                self.ucsrc.update( |src| {
                    src.set_bit(4,true);
                    src.set_bit(5,true);
                });
            },
        }
    }

    /// Function to set the number of stop bits in the USART.
    fn set_stop(&self,stop : UsartStop) {
        match stop {
            UsartStop::one => { 
                self.ucsrc.update( |src| {
                    src.set_bit(3,false);
                });
            },
            UsartStop::two => { 
                self.ucsrc.update( |src| {
                    src.set_bit(3,true);
                });
            },
        }
    }

 /// Set the frame format for USART.
    /// A serial frame is defined to be one character of data bits with 
    /// synchronization bits (start and stop bits), and optionally
    /// a parity bit for error checking.
    /// The USART accepts all 30 combinations of the following as valid frame formats.
    fn set_frame(&self,stop : UsartStop,size : UsartDataSize,parity : UsartParity) {
        self.set_size(size);
        self.set_parity(parity);
        self.set_stop(stop);
    }
    

    /// This is the cumulative function for initializing a particular
    /// USART and it will take all the necessary details about the mode
    /// in which the USART pin is to be used.
    pub fn initialize(&mut self,mode : UsartModes,baud : i64,stop : UsartStop,size : UsartDataSize,parity : UsartParity) {
        // Check that recieve and transmit buffers are completely cleared
        // and no transmission or recieve of data is already in process.
        self.enable();                                             //  Enable Global interrupts.
        self.check();
        
        self.disable();                                            //  Disable Global interrupts.
        let num : UsartNum = self.get_num();
        
        self.set_power(num);                                       //  Set Power reduction register.
        
        self.mode_select(mode);                                    //  Set the USART at the given mode.
        
        //  Set the clock for USART according to user input.
        if( mode == UsartModes::slave_sync )  { }
        else { self.set_clock(baud,mode) }                         
        
        //  Set the frame format according to input.
        self.set_frame(stop,size,parity);                                     

        self.enable();                                             //  Enable Global interrupts.
    }

}
