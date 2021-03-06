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

//! Generic implementation of power control through clock gating in ATMEGA2560P.
//! Section 11.10.2 and 11.10.3 of the manual.
//! Also references from Section 11.8.

// Crates required in the code for reading and writing to registers.
use core::ptr::{read_volatile, write_volatile};

/// The options correspond to real world as shown -
///  `TWI    :  Power Reduction TWI`     
///  `TIMER2 :  Power Reduction Timer/Counter2`
///  `TIMER0 :  Power Reduction Timer/Counter0`
///  `TIMER1 :  Power Reduction Timer/Counter1`
///  `SPI    :  Power Reduction Serial Peripheral Interface`
///  `USART0 :  Power Reduction USART0`
///  `ADC    :  Power Reduction ADC`
///  `TIMER5 :  Power Reduction Timer/Counter5`
///  `TIMER4 :  Power Reduction Timer/Counter4`
///  `TIMER3 :  Power Reduction Timer/Counter3`
///  `USART3 :  Power Reduction USART3`
///  `USART2 :  Power Reduction USART2`
///  `USART1 :  Power Reduction USART1`
#[derive(Clone, Copy)]
pub enum Peripherals {
    TWI,
    TIMER2,
    TIMER0,
    TIMER1,
    SPI,
    USART0,
    ADC,
    TIMER5,
    TIMER4,
    TIMER3,
    USART3,
    USART2,
    USART1,
}

/// Contains registers to control the functioning of clocks in the chip.
/// It would be used to control the power modes of the ATMEGA2560P chip as mentioned
/// in the enum `Options` above.
#[repr(C, packed)]
pub struct Power {
    pub prr0: u8,
    pub prr1: u8,
}

impl Power {
    /// Creates a new reference to the Sleep structure at a specified location.
    /// # Returns
    /// * `a reference Power` - used for further power implementations.
    pub unsafe fn new() -> &'static mut Power {
        &mut *(0x64 as *mut Power)
    }

    /// This is the function for disabling the clock system of your choice.
    /// It would create a new element of the structure power
    /// which would be used to control various clock gating features of the
    /// chip ATMEGA2560P.
    /// All the clock features are implemented in this function using match cases.
    /// Please specify the type of power reduction mode to be used as the mode,
    /// use the standard keywords.
    /// For more details please refer to the lines before the enum `Options`.
    /// # Arguments
    /// * `mode` - a `Options` object, to set the power mode to disable clocks in a specific defined mode.
    pub fn disable_clocks(&mut self, mode: Peripherals) {
        let mut prr;
        match mode {
            Peripherals::TWI
            | Peripherals::TIMER2
            | Peripherals::TIMER0
            | Peripherals::TIMER1
            | Peripherals::SPI
            | Peripherals::USART0
            | Peripherals::ADC => {
                prr = unsafe { read_volatile(&mut self.prr0) };
            }

            Peripherals::TIMER5
            | Peripherals::TIMER4
            | Peripherals::TIMER3
            | Peripherals::USART3
            | Peripherals::USART2
            | Peripherals::USART1 => {
                prr = unsafe { read_volatile(&mut self.prr1) };
            }
        }
        match mode {
            Peripherals::TWI => {
                prr = prr | 0x80;
            }
            Peripherals::TIMER2 => {
                prr = prr | 0x40;
            }
            Peripherals::TIMER0 => {
                prr = prr | 0x20;
            }
            Peripherals::TIMER1 => {
                prr = prr | 0x08;
            }
            Peripherals::SPI => {
                prr = prr | 0x04;
            }
            Peripherals::USART0 => {
                prr = prr | 0x02;
            }
            Peripherals::ADC => {
                prr = prr | 0x01;
            }
            Peripherals::TIMER5 => {
                prr = prr | 0x20;
            }
            Peripherals::TIMER4 => {
                prr = prr | 0x10;
            }
            Peripherals::TIMER3 => {
                prr = prr | 0x08;
            }
            Peripherals::USART3 => {
                prr = prr | 0x04;
            }
            Peripherals::USART2 => {
                prr = prr | 0x02;
            }
            Peripherals::USART1 => {
                prr = prr | 0x01;
            }
        }
        match mode {
            Peripherals::TWI
            | Peripherals::TIMER2
            | Peripherals::TIMER0
            | Peripherals::TIMER1
            | Peripherals::SPI
            | Peripherals::USART0
            | Peripherals::ADC => unsafe {
                write_volatile(&mut self.prr0, prr);
            },

            Peripherals::TIMER5
            | Peripherals::TIMER4
            | Peripherals::TIMER3
            | Peripherals::USART3
            | Peripherals::USART2
            | Peripherals::USART1 => unsafe {
                write_volatile(&mut self.prr1, prr);
            },
        }
    }

    /// This is the function for enabling the clock system of your choice.
    /// Here we would create a new element of the structure power
    /// and it would be used to control various clock gating features of the
    /// chip ATMEGA2560P.
    /// All the clock features are implemented in this function using match cases.
    /// Please specify the type of power reduction mode to be used as the mode,
    /// use the standard keywords.
    /// For more details please refer to the comment lines before the enum `Options`.
    /// # Arguments
    /// * `mode` - a `Options` object, to set the power mode to enable clocks in a specific defined mode.
    pub fn enable_clocks(&mut self, mode: Peripherals) {
        let mut prr;
        match mode {
            Peripherals::TWI
            | Peripherals::TIMER2
            | Peripherals::TIMER0
            | Peripherals::TIMER1
            | Peripherals::SPI
            | Peripherals::USART0
            | Peripherals::ADC => {
                prr = unsafe { read_volatile(&mut self.prr0) };
            }

            Peripherals::TIMER5
            | Peripherals::TIMER4
            | Peripherals::TIMER3
            | Peripherals::USART3
            | Peripherals::USART2
            | Peripherals::USART1 => {
                prr = unsafe { read_volatile(&mut self.prr1) };
            }
        }
        match mode {
            Peripherals::TWI => {
                prr = prr & 0x7F;
            }
            Peripherals::TIMER2 => {
                prr = prr & 0xBF;
            }
            Peripherals::TIMER0 => {
                prr = prr & 0xDF;
            }
            Peripherals::TIMER1 => {
                prr = prr & 0xF7;
            }
            Peripherals::SPI => {
                prr = prr & 0xFB;
            }
            Peripherals::USART0 => {
                prr = prr & 0xFD;
            }
            Peripherals::ADC => {
                prr = prr & 0xFE;
            }
            Peripherals::TIMER5 => {
                prr = prr & 0xDF;
            }
            Peripherals::TIMER4 => {
                prr = prr & 0xEF;
            }
            Peripherals::TIMER3 => {
                prr = prr & 0xF7;
            }
            Peripherals::USART3 => {
                prr = prr & 0xFB;
            }
            Peripherals::USART2 => {
                prr = prr & 0xFD;
            }
            Peripherals::USART1 => {
                prr = prr & 0xFE;
            }
        }
        match mode {
            Peripherals::TWI
            | Peripherals::TIMER2
            | Peripherals::TIMER0
            | Peripherals::TIMER1
            | Peripherals::SPI
            | Peripherals::USART0
            | Peripherals::ADC => unsafe {
                write_volatile(&mut self.prr0, prr);
            },

            Peripherals::TIMER5
            | Peripherals::TIMER4
            | Peripherals::TIMER3
            | Peripherals::USART3
            | Peripherals::USART2
            | Peripherals::USART1 => unsafe {
                write_volatile(&mut self.prr1, prr);
            },
        }
    }
}
