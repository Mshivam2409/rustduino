//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Devansh Kumar Jha,Indian Institute of Technology Kanpur
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

//! Implementation of Sleep Modes of ATMEGA2560P.
//! Section 11.10.1 of the manual.
//! Also references from Section 11.4.
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf

/// Crates required in the code for reading and writing to registers.
use core::ptr::{read_volatile, write_volatile};

/// Various modes are
/// `IDLE  : Idle sleep mode`          
/// `ADC   : ADC Noise Reduction`
/// `PD    : Power-down`    
/// `PS    : Power-save`
/// `SBY   : Standby`      
/// `ESBY  : Extended Standby`
#[derive(Clone, Copy)]
pub enum Options {
    IDLE,
    ADC,
    PD,
    PS,
    SBY,
    ESBY,
}

/// Contains registers to control the sleep modes.
/// These bits select between the six available sleep modes in ATMEGA2560P.
#[repr(C, packed)]
pub struct Sleep {
    pub smcr: u8,
}

impl Sleep {
    /// Creates a new reference to the Sleep structure according to appropriate location.
    pub unsafe fn new() -> &'static mut Sleep {
        &mut *(0x53 as *mut Sleep)
    }

    /// Write appropriate value to register for enabling the sleep mode.
    pub fn enable(&mut self) {
        let mut smcr = unsafe { read_volatile(&mut self.smcr) };
        smcr = smcr | 0x01;
        unsafe {
            write_volatile(&mut self.smcr, smcr);
        }
    }

    /// Write appropriate value to register for disabling the sleep mode.
    pub fn disable(&mut self) {
        let mut smcr = unsafe { read_volatile(&mut self.smcr) };
        smcr = smcr & 0xFE;
        unsafe {
            write_volatile(&mut self.smcr, smcr);
        }
    }

    /// Set the bits of SMCR register according to the sleep mode required.
    /// The sleep mode to be set will be given as the standard name.  
    /// For more details about the available Options for the sleep mode please check the
    /// comment given above the enum `Options` in the code.
    pub fn select_mode(&mut self, mode: Options) {
        self.enable(); // Enable the Sleep mode
        let mut smcr = 0x0F;
        match mode {
            Options::IDLE => {
                smcr = smcr & 0xF1;
            }
            Options::ADC => {
                smcr = smcr & 0xF3;
            }
            Options::PD => {
                smcr = smcr & 0xF5;
            }
            Options::PS => {
                smcr = smcr & 0xF7;
            }
            Options::SBY => {
                smcr = smcr & 0xFD;
            }
            Options::ESBY => {
                smcr = smcr & 0xFF;
            }
        }
        unsafe {
            write_volatile(&mut self.smcr, smcr);
        }
    }
}
