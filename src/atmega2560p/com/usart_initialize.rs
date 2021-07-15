//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Devansh Kumar Jha, Indian Institute of Technology Kanpur
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


/// Crates which would be used in the implementation.
/// We will be using standard volatile and bit_field crates now for a better read and write.
use bit_field::BitField;
use volatile::Volatile;

use crate::rustduino::atmega2560p::hal::interrupts;
use crate::rustduino::atmega2560p::hal::port;
use crate::rustduino::atmega2560p::hal::power;


/// Some useful constants regarding bit manipulation for USART.
const usart0_xck : u8 = 2;
const usart1_xck : u8 = 5;
const usart2_xck : u8 = 2;
const usart3_xck : u8 = 2;
const usart0_td  : u8 = 1;
const usart1_td  : u8 = 3;
const usart2_td  : u8 = 1;
const usart3_td  : u8 = 1;
const usart0_rd  : u8 = 0;
const usart1_rd  : u8 = 2;
const usart2_rd  : u8 = 0;
const usart3_rd  : u8 = 0;


/// Selection of which USART is to be used.
#[derive(Clone, Copy)]
pub enum UsartNum {
    usart0,
    usart1,
    usart2,
    usart3,
}


/// Selection of synchronous or asynchronous modes for USART.
#[derive(Clone, Copy)]
pub enum UsartModes {
    norm_async,
    dou_async,
    master_sync,
    slave_sync,
}

/// Selection of the type in which USART is to be used ( Reciever,Transmitter ).
#[derive(Clone, Copy)]
pub enum UsartTypes {
    recieve,
    transmit,
}


/// This structure contains various registers needed to control usart communication
/// through ATMEGA2560P device.
/// Each USARTn ( n=0,1,2,3 ) is controlled by a total of 6 registers stored through this structure. 
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
impl Usart {
    /// This creates a new memory mapped structure of the type USART for it's control.
    pub unsafe fn new(num : UsartNum) -> &'static mut Usart {
        match num {
           UsartNum::usart0 =>{ &mut *(0xC0 as *mut Usart)  },
           UsartNum::usart1 =>{ &mut *(0xC8 as *mut Usart)  },
           UsartNum::usart2 =>{ &mut *(0xD0 as *mut Usart)  },
           UsartNum::usart3 =>{ &mut *(0x130 as *mut Usart) },
        }
    }


    /// Function to disable global interrupts for smooth non-interrupted functioning of USART.
    fn disable(&mut self) {
        unsafe {
            // Disable global interrupts.
            interrupts::GlobalInterrupts::disable(&mut interrupts::GlobalInterrupts::new());
        }
    }

    /// Function to re-enable global interrupts.
    fn enable(&mut self) {
        unsafe {
            // Enable global interrupts.
            interrupts::GlobalInterrupts::enable(&mut interrupts::GlobalInterrupts::new());
        }
    }


    /// This function will return the Number of the USART according to the address.
    fn get_num(&self) -> UsartNum {
        let address = (self as *const Usart) as u8;             // Gets address of usart structure.
        match address {
            // Return the number of USART used based on the address read.
            0xC0  => UsartNum::usart0,
            0xC8  => UsartNum::usart1,
            0xD0  => UsartNum::usart2,
            0x130 => UsartNum::usart3,
            _     => unreachable!(),
        }
    }

    
    /// Function to get the port containing bits to
    /// manipulate Recieve,Transmit and XCK bit of the particular USART.
    fn get_port(&self) -> port::Port {
        let num : UsartNum = self.get_num();
        unsafe {
            match num {
                UsartNum::usart0 => { port::Port::new(E) },
                UsartNum::usart1 => { port::Port::new(D) },
                UsartNum::usart2 => { port::Port::new(H) },
                UsartNum::usart3 => { port::Port::new(J) },
            }
        }
    }

    /// Function to return the index of xck bit in the port.
    fn get_xck(&self) -> u8 {
        let num : UsartNum = self.get_num();
        match num {
            UsartNum::usart0 => { usart0_xck },
            UsartNum::usart1 => { usart1_xck },
            UsartNum::usart2 => { usart2_xck },
            UsartNum::usart3 => { usart3_xck },
        }
    }
 

    /// Function to return the index of transmit bit in the port.
    fn get_td(&self) -> u8 {
        let num : UsartNum = self.get_num();
        match num {
            UsartNum::usart0 => { usart0_td },
            UsartNum::usart1 => { usart1_td },
            UsartNum::usart2 => { usart2_td },
            UsartNum::usart3 => { usart3_td },
        }
    }

    /// Function to return the index of recieve bit in the port.
    fn get_rd(&self) -> u8 {
        let num : UsartNum = self.get_num();
        match num {
            UsartNum::usart0 => { usart0_rd },
            UsartNum::usart1 => { usart1_rd },
            UsartNum::usart2 => { usart2_rd },
            UsartNum::usart3 => { usart3_rd },
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
                    port.ddr.update( |ddr| {
                        ddr.set_bit(xck,true);
                    });       
            },
            UsartModes::slave_sync => {                              // Puts the USART into slave synchronous mode
                    let port : (port::Port) = self.get_port();
                    let xck : u8 = self.get_xck();
                    port.ddr.update( |ddr| {
                        ddr.set_bit(xck,false);
                    });
            },
        }
    }

    /// Function to set the power reduction register so that USART functioning is allowed.
    fn set_power(&self,num : UsartNum) {
        unsafe {
            let pow : (power::Power) = power::Power::new();
        }     
        match num {
            UsartNum::usart0 => { 
                pow.prr0.update( |prr| {
                    prr.set_bit(1,false);
                }); 
            },
            UsartNum::usart1 => { 
                pow.prr1.update( |prr| {
                    prr.set_bit(0,false);
                }); 
            },
            UsartNum::usart2 => { 
                pow.prr1.update( |prr| {
                    prr.set_bit(1,false);
                }); 
            },
            UsartNum::usart3 => { 
                pow.prr1.update( |prr| {
                    prr.set_bit(2,false);
                }); 
            },
        }
    }


    /// Sets the interrupt bits in UCSRB so that ongoing
    /// data transfers can be tracked.
    fn check(&self) {
        self.ucsrb.update( |srb| {
              
        });
    }

    /// This is the cumulative function for initializing a particular
    /// USART and it will take all the necessary details about the mode
    /// in which the USART pin is to be used.
    pub fn initialize(&mut self,mode : UsartModes,work : UsartTypes) {
        // Check that recieve and transmit buffers are completely cleared
        // and no transmission or recieve of data is already in process.
        self.check();
        
        self.disable();                                            //  Disable Global interrupts.
        
        let num : UsartNum = self.get_num();
        self.set_power(num);                                       //  Set Power reduction register.
        self.mode_select(mode);                                    //  Set the USART at the given mode.
           

        self.enable();                                             //  Enable Global interrupts.

    }
}