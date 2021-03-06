//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021 Kshitij Kaithal, Indian Institute of Technology Kanpur
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

//! ATMEGA328P has total only 1 USART.
//! This is the file which contains functions for initializing USART in various modes.
//! It has functions to check for the power reduction settings and start the USART in a user defined modes.
//! After setting into the USART the functions are available to generate the clock with given
//! frequency and baud rate. After which the frame for data tracking is set using various frame modes.
//! See the section 19 of ATMEGA328P datasheet.

// Standard crates to be used
use crate::delay::delay_ms;
use bit_field::BitField;
use core::ptr::write_volatile;
use core::{f64, u32, u8};
use volatile::Volatile;

// Source code crates required
use crate::atmega328p::hal::interrupts;
use crate::atmega328p::hal::port;
use crate::atmega328p::hal::power;

// Some useful constants regarding bit manipulation for USART.
// Position of clock mode adjuster (xck) bit.
const USART0_XCK: u8 = 4;
// System Clock Crystal Oscillator Frequency in mHz.
const F_OSC: f64 = 1.0000;
const MULTIPLY: f64 = 1000000.00;

/// Selection of which USART is to be used.
#[derive(Clone, Copy)]
pub enum UsartNum {
    Usart0,
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
/// through ATMEGA320P device.
/// USART0 is controlled by a total of 6 registers stored through this structure.
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

/// Various implementation functions for the USART protocol.
impl Usart {
    /// This creates a new memory mapped structure of the USART0 for it's control.
    /// # Arguments
    /// * `num` - a `UsartNum` object, which defines the USART for whom new reference is to be created.
    /// # Returns
    /// * `a reference to Usart` - which will be used to control the USART.
    pub unsafe fn new(num: UsartNum) -> &'static mut Usart {
        match num {
            UsartNum::Usart0 => &mut *(0xC0 as *mut Usart),
        }
    }
}

impl Usart {
    /// disables global interrupts for smooth non-interrupted functioning of USART.
    fn disable(&mut self) {
        unsafe {
            // Disable global interrupts.
            interrupts::Interrupt::disable(&mut interrupts::Interrupt::new());
        }
    }

    ///  re-enables global interrupts.
    fn enable(&mut self) {
        unsafe {
            // Enable global interrupts.
            interrupts::Interrupt::enable(&mut interrupts::Interrupt::new());
        }
    }

    ///  Returns the Number of the USART according to the address.
    /// # Returns
    /// * `a UsartNum object` - The number of the USART is returned which is in use.
    fn get_num(&mut self) -> UsartNum {
        let address = (self as *const Usart) as u8; // Gets address of usart structure.
        match address {
            // Return the number of USART used based on the address read.
            0xC0 => UsartNum::Usart0,
            _ => unreachable!(),
        }
    }

    /// Function to get the port containing bits to
    /// manipulate Recieve,Transmit and XCK bit of the particular USART.
    /// # Returns
    /// * `a tuple` - which contains -
    ///     * `a mutable reference to Port object` - The port which controls the given USART.
    ///     * `a u8` - The index location of XCK bit for mode specific implementation.
    fn get_port_xck(&mut self) -> (&mut port::Port, u8) {
        let num: UsartNum = self.get_num();

        match num {
            UsartNum::Usart0 => (port::Port::new(port::PortName::D), USART0_XCK),
        }
    }

    /// Checks the mode of the USART.
    /// # Returns
    /// `a boolean` - which is false for asynchronous and true for synchronous.
    fn get_mode(&mut self) -> bool {
        let mut src = self.ucsrc.read();
        src = src & (1 << 6);
        if src == 0 {
            return false;
        } else {
            return true;
        }
    }

    /// setting the clock polarity mode which is of use in the recieve and transmission implementation of USART.
    /// # Arguments
    /// * `mode` - a `UsartPolarity` object, which will be set for the USART.
    pub fn set_polarity(&mut self, mode: UsartPolarity) {
        if self.get_mode() == false {
            self.ucsrc.update(|src| {
                src.set_bit(0, false);
            });
        } else {
            match mode {
                UsartPolarity::Outputrise => {
                    self.ucsrc.update(|src| {
                        src.set_bit(0, false);
                    });
                }
                UsartPolarity::Inputrise => {
                    self.ucsrc.update(|src| {
                        src.set_bit(0, true);
                    });
                }
            }
        }
    }

    /// Set various modes of the USART which is activated.
    /// # Arguments
    /// * `mode` - a `UsartModes` object, which will be set for the USART.
    pub fn mode_select(&mut self, mode: UsartModes) {
        match mode {
            UsartModes::Normasync                                  // Puts the USART into asynchronous mode.
            | UsartModes::Douasync => {
                    self.ucsrc.update( |src| {
                        src.set_bit(6,false);
                        src.set_bit(7,false);
                    });
            },
            UsartModes::Mastersync
            | UsartModes::Slavesync => {                           // Puts the USART into synchronous mode.
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
            UsartModes::Normasync => {
                // Keeps the USART into normal asynchronous mode.
                self.ucsra.update(|sra| {
                    sra.set_bit(1, false);
                });
            }
            UsartModes::Douasync => {
                // Puts the USART into double speed asynchronous mode.
                self.ucsra.update(|sra| {
                    sra.set_bit(1, true);
                });
            }
            UsartModes::Mastersync => {
                // Puts the USART into master synchronous mode
                let (port, xck) = self.get_port_xck();
                unsafe {
                    write_volatile(&mut port.ddr, port.ddr | 1 << xck);
                }
            }
            UsartModes::Slavesync => {
                // Puts the USART into slave  synchronous mode
                let (port, xck) = self.get_port_xck();

                unsafe {
                    write_volatile(&mut port.ddr, port.ddr & !(1 << xck));
                }
            }
        }
    }

    ///  Set the power reduction register so that USART functioning is allowed.
    /// # Arguments
    /// * `num` - a `UsartNum` object, for which the power configurations of the USART will be set.
    pub fn set_power(&mut self, num: UsartNum) {
        let pow: &mut power::Power;

        pow = power::Power::new();

        match num {
            UsartNum::Usart0 => unsafe {
                write_volatile(&mut pow.prr, pow.prr & !(1 << 1));
            },
        }
    }

    /// Checks for any currently undergoing recieval or transmission in the USART.
    /// # Returns
    /// * `a boolean` - Which is false if USART is busy otherwise true.
    fn check_ongoing(&self) -> bool {
        let ucsra = self.ucsra.read();
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
    /// # Arguments
    /// * `mode` - a `UsartModes` object, which defines the mode of USART to use.
    /// * `baud` - a i64, the baud rate of USART the user wants to set.
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
        self.ubrrl.update(|ubrrl| {
            for i in 0..8 {
                ubrrl.set_bit(i, ubrr.get_bit(i));
            }
        });
        self.ubrrh.update(|ubrrh| {
            for i in 0..4 {
                ubrrh.set_bit(i, ubrr.get_bit(i + 8));
            }
        });
    }

    ///  Sets the limit of data to be handled by USART.
    /// # Arguments
    /// * `size` - a `UsartDatSize` object, the size of set of bits to transmit.
    fn set_size(&mut self, size: UsartDataSize) {
        match size {
            UsartDataSize::Five
            | UsartDataSize::Six
            | UsartDataSize::Seven
            | UsartDataSize::Eight => {
                self.ucsrb.update(|srb| {
                    srb.set_bit(2, false);
                });
            }
            UsartDataSize::Nine => {
                self.ucsrb.update(|srb| {
                    srb.set_bit(2, true);
                });
            }
        }
        match size {
            UsartDataSize::Five | UsartDataSize::Six => {
                self.ucsrc.update(|src| {
                    src.set_bit(2, false);
                });
            }
            UsartDataSize::Nine | UsartDataSize::Seven | UsartDataSize::Eight => {
                self.ucsrc.update(|src| {
                    src.set_bit(2, true);
                });
            }
        }
        match size {
            UsartDataSize::Five | UsartDataSize::Seven => {
                self.ucsrc.update(|src| {
                    src.set_bit(1, false);
                });
            }
            UsartDataSize::Nine | UsartDataSize::Six | UsartDataSize::Eight => {
                self.ucsrc.update(|src| {
                    src.set_bit(1, true);
                });
            }
        }
    }

    /// Set the parity bit in the frame of USART.
    /// # Arguments
    /// * `parity` - a `UsartParity` object, which gives the Parity bit mode for USART.
    fn set_parity(&mut self, parity: UsartParity) {
        match parity {
            UsartParity::No => {
                self.ucsrc.update(|src| {
                    src.set_bit(4, false);
                    src.set_bit(5, false);
                });
            }
            UsartParity::Even => {
                self.ucsrc.update(|src| {
                    src.set_bit(4, false);
                    src.set_bit(5, true);
                });
            }
            UsartParity::Odd => {
                self.ucsrc.update(|src| {
                    src.set_bit(4, true);
                    src.set_bit(5, true);
                });
            }
        }
    }

    /// Setting the number of stop bits in the USART.
    /// # Arguments
    /// * `stop` - a `UsartStop` object, which will be used to set the stop bits of data frame.
    fn set_stop(&mut self, stop: UsartStop) {
        match stop {
            UsartStop::One => {
                self.ucsrc.update(|src| {
                    src.set_bit(3, false);
                });
            }
            UsartStop::Two => {
                self.ucsrc.update(|src| {
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
    /// # Arguments
    /// * `size` - a `UsartDatSize` object, the size of set of bits to transmit.
    /// * `parity` - a `UsartParity` object, which gives the Parity bit mode for USART.
    /// * `stop` - a `UsartStop` object, which will be used to set the stop bits of data frame.
    fn set_frame(&mut self, stop: UsartStop, size: UsartDataSize, parity: UsartParity) {
        self.set_size(size);
        self.set_parity(parity);
        self.set_stop(stop);
    }

    /// This is the cumulative function for initializing a particular
    /// USART and it will take all the necessary details about the mode
    /// in which the USART pin is to be used.
    /// # Arguments
    /// * `mode` - a `UsartModes` object, which defines the mode of USART to use.
    /// * `baud` - a i64, the baud rate of USART the user wants to set.
    /// * `size` - a `UsartDatSize` object, the size of set of bits to transmit.
    /// * `parity` - a `UsartParity` object, which gives the Parity bit mode for USART.
    /// * `stop` - a `UsartStop` object, which will be used to set the stop bits of data frame.
    pub fn initialize(
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
        let num: UsartNum = self.get_num();

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
