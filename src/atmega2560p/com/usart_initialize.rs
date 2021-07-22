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

//! ATMEGA2560P has total of 4 USARTs.
//! This is the file which contains functions for initializing USART in various modes.
//! It has functions to check for the power reduction settings and start the USART in a user defined modes.
//! After setting into a particular USART the functions are available to generate the clock with given
//! frequency and baud rate. After which the frame for data tracking is set using various frame modes.
//! See the section 22 of ATMEGA2560P datasheet.
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf

/// Other source code files to be used.
use crate::atmega2560p::hal::interrupts;
use crate::atmega2560p::hal::port;
use crate::atmega2560p::hal::power;

/// Crates which would be used in the implementation.
/// We will be using standard volatile and bit_field crates now for a better read and write.
use crate::delay::delay_ms;
use bit_field::BitField;
use core::ptr::write_volatile;
use core::{f64, u32, u8};
use volatile::Volatile;

/// Some useful constants regarding bit manipulation for USART.
/// Position of clock mode adjuster (xck) bit.
const USART0_XCK: u8 = 2;
const USART1_XCK: u8 = 5;
const USART2_XCK: u8 = 2;
const USART3_XCK: u8 = 2;
/// System Clock Crystal Oscillator Frequency in mHz.
const F_OSC: f64 = 1.0000;
const MULTIPLY: f64 = 1000000.00;

/// Selection of which USART is to be used.
#[derive(Clone, Copy)]
pub enum UsartNum {
    Usart0,
    Usart1,
    Usart2,
    Usart3,
}

/// Selection of synchronous or asynchronous modes for USART.
#[derive(Clone, Copy)]
pub enum UsartModes {
    Normasync,
    Douasync,
    Mastersync,
    Slavesync,
}

/// Selection of the parity mode for USART.
#[derive(Clone, Copy)]
pub enum UsartParity {
    No,
    Even,
    Odd,
}

/// Selection of the Amount of Data Bits to be transferred or recieved through USART.
#[derive(Clone, Copy)]
pub enum UsartDataSize {
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

/// Selection of number of stop bits for USART data.
#[derive(Clone, Copy)]
pub enum UsartStop {
    One,
    Two,
}

/// Selection of the clock parity mode.
#[derive(Clone, Copy)]
pub enum UsartPolarity {
    Outputrise,
    Inputrise,
}

/// This structure contains various registers needed to control usart communication
/// through ATMEGA2560P device.
/// Each USARTn ( n=0,1,2,3 ) is controlled by a total of 6 registers stored through this structure.
#[repr(C, packed)]
pub struct Usart {
    pub ucsra: Volatile<u8>,
    pub ucsrb: Volatile<u8>,
    pub ucsrc: Volatile<u8>,
    _pad: u8, // Padding to look for empty memory space.
    pub ubrrl: Volatile<u8>,
    pub ubrrh: Volatile<u8>,
    pub udr: Volatile<u8>,
}

/// Contains the Usart as a Raw Pointer along with it's name.
#[repr(C, packed)]
pub struct UsartObject {
    pub usart: *mut Usart,
    pub name: UsartNum,
}

// new() functions to make the memory mapped IOs for both the structures.

/// USART mutable reference declaration.
impl Usart {
    /// This creates a new memory mapped structure of the type USART for it's control.
    pub unsafe fn new(num: UsartNum) -> &'static mut Usart {
        match num {
            UsartNum::Usart0 => &mut *(0xC0 as *mut Usart),
            UsartNum::Usart1 => &mut *(0xC8 as *mut Usart),
            UsartNum::Usart2 => &mut *(0xD0 as *mut Usart),
            UsartNum::Usart3 => &mut *(0x130 as *mut Usart),
        }
    }

    /// Returns the number (index) of the USART being used.
    /// Panics if the address is invalid.
    pub fn name(&self) -> UsartNum {
        let address = (self as *const Usart) as usize; // Gets address of port.
        match address {
            //  Return Usart Number based on the address read.
            0xC0 => UsartNum::Usart0,
            0xC8 => UsartNum::Usart1,
            0xD0 => UsartNum::Usart2,
            0x130 => UsartNum::Usart3,
            _ => unreachable!(),
        }
    }

    /// Creates a instance of UsartObject from the Usart instance available.
    pub fn create_object(&mut self) -> UsartObject {
        UsartObject {
            usart: self,
            name: self.name(),
        }
    }
}

/// USART raw pointers declaration.
impl UsartObject {
    /// This creates a raw pointer for formation of the serial structure ahead
    /// to control all the USARTs of ATMEGA2560P at one place.
    pub unsafe fn new(num: UsartNum) -> UsartObject {
        Usart::new(num).create_object()
    }
}

/// Various implementation functions for the USART protocol.
impl UsartObject {
    /// Disable global interrupts for smooth non-interrupted functioning of USART.
    fn disable(&self) {
        unsafe {
            // Disable global interrupts.
            interrupts::GlobalInterrupts::disable(&mut interrupts::GlobalInterrupts::new());
        }
    }

    /// Re-enable global interrupts.
    fn enable(&self) {
        unsafe {
            // Enable global interrupts.
            interrupts::GlobalInterrupts::enable(&mut interrupts::GlobalInterrupts::new());
        }
    }

    /*
    /// This function will return the Number of the USART according to the address.
    fn get_num(&mut self) -> UsartNum {
        let address = (self as *const Usart) as usize; // Gets address of usart structure.
        match address {
            // Return the number of USART used based on the address read.
            0xC0 => UsartNum::Usart0,
            0xC8 => UsartNum::Usart1,
            0xD0 => UsartNum::Usart2,
            0x130 => UsartNum::Usart3,
            _ => unreachable!(),
        }
    }
    */

    /// Returns the port containing bits to
    /// manipulate Recieve,Transmit and XCK bit of the particular USART.
    fn get_port_xck(&mut self) -> (&mut port::Port, u8) {
        let num: UsartNum = unsafe { (*self.usart).name() };
        match num {
            UsartNum::Usart0 => (port::Port::new(port::PortName::E), USART0_XCK),
            UsartNum::Usart1 => (port::Port::new(port::PortName::D), USART1_XCK),
            UsartNum::Usart2 => (port::Port::new(port::PortName::H), USART2_XCK),
            UsartNum::Usart3 => (port::Port::new(port::PortName::J), USART3_XCK),
        }
    }

    /// Returns 0 for asynchronous and 1 for synchronous.
    fn get_mode(&mut self) -> bool {
        let mut src = unsafe { (*self.usart).ucsrc.read() };
        src = src & (1 << 6);
        if src == 0 {
            return false;
        } else {
            return true;
        }
    }

    /// Set clock polarity mode according to input from user.
    pub unsafe fn set_polarity(&mut self, mode: UsartPolarity) {
        if self.get_mode() == false {
            (*self.usart).ucsrc.update(|src| {
                src.set_bit(0, false);
            });
        } else {
            match mode {
                UsartPolarity::Outputrise => {
                    (*self.usart).ucsrc.update(|src| {
                        src.set_bit(0, false);
                    });
                }
                UsartPolarity::Inputrise => {
                    (*self.usart).ucsrc.update(|src| {
                        src.set_bit(0, true);
                    });
                }
            }
        }
    }

    /// Set's various modes of the USART which is activated.
    pub unsafe fn mode_select(&mut self, mode: UsartModes) {
        match mode {
            UsartModes::Normasync                                  // Puts the USART into asynchronous mode.
            | UsartModes::Douasync => {
                    (*self.usart).ucsrc.update( |src| {
                        src.set_bit(6,false);
                        src.set_bit(7,false);
                    });
            },
            UsartModes::Mastersync
            | UsartModes::Slavesync => {                           // Puts the USART into synchronous mode.
                    (*self.usart).ucsrc.update( |src| {
                        src.set_bit(6,true);
                        src.set_bit(7,false);
                    });
                    (*self.usart).ucsra.update( |sra| {
                        sra.set_bit(1,false);
                    });
            },
        }
        match mode {
            UsartModes::Normasync => {
                // Keeps the USART into normal asynchronous mode.
                (*self.usart).ucsra.update(|sra| {
                    sra.set_bit(1, false);
                });
            }
            UsartModes::Douasync => {
                // Puts the USART into double speed asynchronous mode.
                (*self.usart).ucsra.update(|sra| {
                    sra.set_bit(1, true);
                });
            }
            UsartModes::Mastersync => {
                // Puts the USART into master synchronous mode
                let (port, xck) = self.get_port_xck();
                write_volatile(&mut port.ddr, port.ddr | 1 << xck);
                // port.ddr.update( |ddr| {
                //     ddr.set_bit(xck,true);
                // });
            }
            UsartModes::Slavesync => {
                // Puts the USART into slave synchronous mode
                let (port, xck) = self.get_port_xck();
                write_volatile(&mut port.ddr, port.ddr & !(1 << xck));
                // port.ddr.update( |ddr| {
                //     ddr.set_bit(xck,false);
                // });
            }
        }
    }

    /// Set's the power reduction register so that USART functioning is allowed.
    fn set_power(&self, num: UsartNum) {
        let pow: &mut power::Power;
        unsafe {
            pow = power::Power::new();
        }
        match num {
            UsartNum::Usart0 => {
                unsafe {
                    write_volatile(&mut pow.prr0, pow.prr0 & !(1 << 1));
                }
                // pow.prr0.update( |prr| {
                //     prr.set_bit(1,false);
                // });
            }
            UsartNum::Usart1 => {
                unsafe {
                    write_volatile(&mut pow.prr1, pow.prr1 & !(1));
                }
                // pow.prr1.update( |prr| {
                //     prr.set_bit(0,false);
                // });
            }
            UsartNum::Usart2 => {
                unsafe {
                    write_volatile(&mut pow.prr1, pow.prr1 & !(1 << 1));
                }
                // pow.prr1.update( |prr| {
                //     prr.set_bit(1,false);
                // });
            }
            UsartNum::Usart3 => {
                unsafe {
                    write_volatile(&mut pow.prr1, pow.prr1 & !(1 << 2));
                }
                // pow.prr1.update( |prr| {
                //     prr.set_bit(2,false);
                // });
            }
        }
    }

    /// Sets the interrupt bits in UCSRB so that ongoing
    /// data transfers can be tracked.
    unsafe fn _check(&mut self) {
        (*self.usart).ucsrb.update(|srb| {
            srb.set_bit(6, true);
            srb.set_bit(7, true);
        });
    }

    /// Return 1 if no ongoing transmission or recieval from the USART.
    /// Return 0 if their is some transfer going on.
    unsafe fn check_ongoing(&self) -> bool {
        let ucsra = (*self.usart).ucsra.read();
        if ucsra.get_bit(6) == true && ucsra.get_bit(7) == false {
            true
        } else {
            false
        }
    }

    /// Clock Generation is one of the initialization steps for the USART.
    /// If the USART is in Asynchronous mode or Master Synchronous mode then a internal
    /// clock generator is used while for Slave Synchronous mode we will use a external
    /// clock generator.
    /// Set the baud rate frequency for USART.
    /// Baud rate settings is used to set the clock for USART.
    fn set_clock(&mut self, baud: i64, mode: UsartModes) {
        let ubrr: u32;
        match mode {
            UsartModes::Normasync => {
                ubrr = (((F_OSC * MULTIPLY) / (16.00 * baud as f64)) - 1.00) as u32;
            }
            UsartModes::Douasync => {
                ubrr = (((F_OSC * MULTIPLY) / (8.00 * baud as f64)) - 1.00) as u32;
            }
            UsartModes::Mastersync => {
                ubrr = (((F_OSC * MULTIPLY) / (2.00 * baud as f64)) - 1.00) as u32;
            }
            _ => unreachable!(),
        }
        unsafe {
            (*self.usart).ubrrl.update(|ubrrl| {
                for i in 0..8 {
                    ubrrl.set_bit(i, ubrr.get_bit(i));
                }
            });
            (*self.usart).ubrrh.update(|ubrrh| {
                for i in 0..4 {
                    ubrrh.set_bit(i, ubrr.get_bit(i + 8));
                }
            });
        }
    }

    /// Set the limit of data to be handled by USART.
    unsafe fn set_size(&mut self, size: UsartDataSize) {
        match size {
            UsartDataSize::Five
            | UsartDataSize::Six
            | UsartDataSize::Seven
            | UsartDataSize::Eight => {
                (*self.usart).ucsrb.update(|srb| {
                    srb.set_bit(2, false);
                });
            }
            UsartDataSize::Nine => {
                (*self.usart).ucsrb.update(|srb| {
                    srb.set_bit(2, true);
                });
            }
        }
        match size {
            UsartDataSize::Five | UsartDataSize::Six => {
                (*self.usart).ucsrc.update(|src| {
                    src.set_bit(2, false);
                });
            }
            UsartDataSize::Nine | UsartDataSize::Seven | UsartDataSize::Eight => {
                (*self.usart).ucsrc.update(|src| {
                    src.set_bit(2, true);
                });
            }
        }
        match size {
            UsartDataSize::Five | UsartDataSize::Seven => {
                (*self.usart).ucsrc.update(|src| {
                    src.set_bit(1, false);
                });
            }
            UsartDataSize::Nine | UsartDataSize::Six | UsartDataSize::Eight => {
                (*self.usart).ucsrc.update(|src| {
                    src.set_bit(1, true);
                });
            }
        }
    }

    /// Set the parity bit for initializing frame of USART.
    unsafe fn set_parity(&mut self, parity: UsartParity) {
        match parity {
            UsartParity::No => {
                (*self.usart).ucsrc.update(|src| {
                    src.set_bit(4, false);
                    src.set_bit(5, false);
                });
            }
            UsartParity::Even => {
                (*self.usart).ucsrc.update(|src| {
                    src.set_bit(4, false);
                    src.set_bit(5, true);
                });
            }
            UsartParity::Odd => {
                (*self.usart).ucsrc.update(|src| {
                    src.set_bit(4, true);
                    src.set_bit(5, true);
                });
            }
        }
    }

    /// Set the number of stop bits in the USART frame.
    unsafe fn set_stop(&mut self, stop: UsartStop) {
        match stop {
            UsartStop::One => {
                (*self.usart).ucsrc.update(|src| {
                    src.set_bit(3, false);
                });
            }
            UsartStop::Two => {
                (*self.usart).ucsrc.update(|src| {
                    src.set_bit(3, true);
                });
            }
        }
    }

    /// Set the frame format for USART.
    /// A serial frame is defined to be one character of data bits with
    /// synchronization bits (start and stop bits), and optionally
    /// a parity bit for error checking.
    /// The USART accepts all 30 combinations of the following as valid frame formats.
    unsafe fn set_frame(&mut self, stop: UsartStop, size: UsartDataSize, parity: UsartParity) {
        self.set_size(size);
        self.set_parity(parity);
        self.set_stop(stop);
    }

    /// This is the cumulative function for initializing a particular
    /// USART and it will take all the necessary details about the mode
    /// in which the USART pin is to be used.
    pub unsafe fn initialize(
        &mut self,
        mode: UsartModes,
        baud: i64,
        stop: UsartStop,
        size: UsartDataSize,
        parity: UsartParity,
    ) {
        // Check that recieve and transmit buffers are completely cleared
        // and no transmission or recieve of data is already in process.
        let mut i: i32 = 10;
        while self.check_ongoing() == false {
            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!()
            }
        }

        self.disable(); //  Disable Global interrupts.
        let num: UsartNum = (*self.usart).name();

        self.set_power(num); //  Set Power reduction register.

        self.mode_select(mode); //  Set the USART at the given mode.

        //  Set the clock for USART according to user input.
        match mode {
            UsartModes::Slavesync => {}
            _ => {
                self.set_clock(baud, mode);
            }
        }

        //  Set the frame format according to input.
        self.set_frame(stop, size, parity);

        self.enable(); //  Enable Global interrupts.
    }
}
