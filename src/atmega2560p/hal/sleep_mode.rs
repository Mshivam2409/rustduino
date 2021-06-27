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

//! Implementation of Sleep Modes of ATMEGA2560P
//! Section 11.10.1 of the manual
//! Also references from Section 11.4
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf
use core;

/// Various modes are
/// IDLE  : Idle sleep mode          
/// ADC   : ADC Noise Reduction
/// PD    : Power-down    
/// PS    : Power-save
/// SBY   : Standby      
/// ESBY  : Extended Standby
#[derive(Clone,Copy)]
pub enum Options {
    IDLE,   
    ADC,
    PD,     
    PS,
    SBY,    
    ESBY,
}

/// Contains registers to control the sleep modes
///
/// Bit 1 – SE: Sleep Enable
/// The SE bit must be written to logic one to make the MCU enter the sleep mode when the SLEEP instruction is executed.
/// To avoid the MCU entering the sleep mode unless it is the programmer’s purpose, it is recommended to
/// write the Sleep Enable (SE) bit to one just before the execution of the SLEEP instruction and to clear it immediately
/// after waking up.
///
/// Bits 3, 2, 1 – SM2:0: Sleep Mode Select Bits 2, 1, and 0
/// These bits select between the six available sleep modes as shown below.
///           SM2  SM1  SM0  Sleep Mode
///           0     0    0     Idle
///           0     0    1     ADC Noise Reduction
///           0     1    0     Power-down
///           0     1    1     Power-save
///           1     0    0     Reserved
///           1     0    1     Reserved
///           1     1    0     Standby
///           1     1    1     Extended Standby
#[repr(C, packed)]
pub struct Sleep { 
    SMCR:u8,
}

impl Sleep {
    /// Creates a new reference to the Sleep structure according to appropriate location 
    pub unsafe fn new() -> &'static mut Sleep {
        &mut *(0x53 as *mut Sleep)
    }

    /// The SE bit must be written to logic one to make the MCU enter the sleep mode when the SLEEP instruction is executed.
    /// To avoid the MCU entering the sleep mode unless it is the programmer’s purpose, it is recommended to
    /// write the Sleep Enable (SE) bit to one just before the execution of the SLEEP instruction and to clear it immediately
    /// after waking up.
    /// Set the last bit of SMCR register as 1 for enabling the sleep mode.
    pub fn enable(&mut self) {
        unsafe {            
            let mut smcr = core::ptr::read_volatile(&mut self.SMCR);
            smcr = smcr | 0x01;
            core::ptr::write_volatile(&mut self.SMCR, smcr);
        }
    }

    /// Set the last bit of SMCR register as 0 for disabling the sleep mode.
    pub fn disable(&mut self) {
        unsafe {
            let mut smcr = core::ptr::read_volatile(&mut self.SMCR);
            smcr = smcr & 0xFE;
            core::ptr::write_volatile(&mut self.SMCR, smcr);
        }
    }

    /// Set the bits of SMCR register according to the sleep mode required.
    /// The sleep mode to be set will be given as the standard name in the manual
    pub fn select_mode(&mut self,mode:Options) {
        unsafe {
            self.enable();                // Enable the Sleep mode
            let mut smcr = core::ptr::read_volatile(&mut self.SMCR);
            smcr = 0x0F;
            match mode {
                Options::IDLE => { smcr = smcr & 0xF1; }
                Options::ADC  => { smcr = smcr & 0xF3; }
                Options::PD   => { smcr = smcr & 0xF5; }
                Options::PS   => { smcr = smcr & 0xF7; }
                Options::SBY  => { smcr = smcr & 0xFD; }
                Options::ESBY => { smcr = smcr & 0xFF; }
                _ => unreachable!(),
            }
            core::ptr::write_volatile(&mut self.SMCR, smcr);
        }
    }
}