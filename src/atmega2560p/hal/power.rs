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
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf

/// Crates required in the code for reading and writing to registers.
use core;

/// The options correspond to real world as shown -
///  ```TWI    :  Power Reduction TWI```     
///  ```TIMER2 :  Power Reduction Timer/Counter2```
///  ```TIMER0 :  Power Reduction Timer/Counter0```
///  ```TIMER1 :  Power Reduction Timer/Counter1```
///  ```SPI    :  Power Reduction Serial Peripheral Interface```
///  ```USART0 :  Power Reduction USART0```
///  ```ADC    :  Power Reduction ADC```
///  ```TIMER5 :  Power Reduction Timer/Counter5```
///  ```TIMER4 :  Power Reduction Timer/Counter4```
///  ```TIMER3 :  Power Reduction Timer/Counter3```
///  ```USART3 :  Power Reduction USART3```
///  ```USART2 :  Power Reduction USART2```  
///  ```USART1 :  Power Reduction USART1```
#[derive(Clone, Copy)]
pub enum Options {
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

/// Contains registers to control the functioning of clocks in the chip -
/// PRR0 : Contains 7 Writable bits which will be used to control some of the clocks modes.
/// PRR1 : Contains 6 Writable bits which will be used to control some of the clocks modes.
#[repr(C, packed)]
pub struct Power {
    prr0: u8,
    prr1: u8,
}

impl Power {
    /// Creates a new reference to the Sleep structure at a specified location.
    pub unsafe fn new() -> &'static mut Power {
        &mut *(0x64 as *mut Power)
    }

    /// This is the function for disabling the clock system of your choice.
    /// It would create a new element of the structure power
    /// which would be used to control various clock gating features of the
    /// chip ATMEGA2560P.
    /// All the clock features are implemented in this function using match cases.
    pub fn disable_clocks(&mut self, mode: Options) {
        unsafe {
            let mut prr;
            match mode {
                Options::TWI
                | Options::TIMER2
                | Options::TIMER0
                | Options::TIMER1
                | Options::SPI
                | Options::USART0
                | Options::ADC => {
                    prr = core::ptr::read_volatile(&mut self.prr0);
                }

                Options::TIMER5
                | Options::TIMER4
                | Options::TIMER3
                | Options::USART3
                | Options::USART2
                | Options::USART1 => {
                    prr = core::ptr::read_volatile(&mut self.prr1);
                }
            }
            match mode {
                Options::TWI => {
                    prr = prr | 0x80;
                }
                Options::TIMER2 => {
                    prr = prr | 0x40;
                }
                Options::TIMER0 => {
                    prr = prr | 0x20;
                }
                Options::TIMER1 => {
                    prr = prr | 0x08;
                }
                Options::SPI => {
                    prr = prr | 0x04;
                }
                Options::USART0 => {
                    prr = prr | 0x02;
                }
                Options::ADC => {
                    prr = prr | 0x01;
                }
                Options::TIMER5 => {
                    prr = prr | 0x20;
                }
                Options::TIMER4 => {
                    prr = prr | 0x10;
                }
                Options::TIMER3 => {
                    prr = prr | 0x08;
                }
                Options::USART3 => {
                    prr = prr | 0x04;
                }
                Options::USART2 => {
                    prr = prr | 0x02;
                }
                Options::USART1 => {
                    prr = prr | 0x01;
                }
            }
            match mode {
                Options::TWI
                | Options::TIMER2
                | Options::TIMER0
                | Options::TIMER1
                | Options::SPI
                | Options::USART0
                | Options::ADC => {
                    core::ptr::write_volatile(&mut self.prr0, prr);
                }

                Options::TIMER5
                | Options::TIMER4
                | Options::TIMER3
                | Options::USART3
                | Options::USART2
                | Options::USART1 => {
                    core::ptr::write_volatile(&mut self.prr1, prr);
                }
            }
        }
    }

    /// This is the function for enabling the clock system of your choice.
    /// Here we would create a new element of the structure power
    /// and it would be used to control various clock gating features of the
    /// chip ATMEGA2560P.
    /// All the clock features are implemented in this function using match cases.
    pub fn enable_clocks(&mut self, mode: Options) {
        unsafe {
            let mut prr;
            match mode {
                Options::TWI
                | Options::TIMER2
                | Options::TIMER0
                | Options::TIMER1
                | Options::SPI
                | Options::USART0
                | Options::ADC => {
                    prr = core::ptr::read_volatile(&mut self.prr0);
                }

                Options::TIMER5
                | Options::TIMER4
                | Options::TIMER3
                | Options::USART3
                | Options::USART2
                | Options::USART1 => {
                    prr = core::ptr::read_volatile(&mut self.prr1);
                }
            }
            match mode {
                Options::TWI => {
                    prr = prr & 0x7F;
                }
                Options::TIMER2 => {
                    prr = prr & 0xBF;
                }
                Options::TIMER0 => {
                    prr = prr & 0xDF;
                }
                Options::TIMER1 => {
                    prr = prr & 0xF7;
                }
                Options::SPI => {
                    prr = prr & 0xFB;
                }
                Options::USART0 => {
                    prr = prr & 0xFD;
                }
                Options::ADC => {
                    prr = prr & 0xFE;
                }
                Options::TIMER5 => {
                    prr = prr & 0xDF;
                }
                Options::TIMER4 => {
                    prr = prr & 0xEF;
                }
                Options::TIMER3 => {
                    prr = prr & 0xF7;
                }
                Options::USART3 => {
                    prr = prr & 0xFB;
                }
                Options::USART2 => {
                    prr = prr & 0xFD;
                }
                Options::USART1 => {
                    prr = prr & 0xFE;
                }
            }
            match mode {
                Options::TWI
                | Options::TIMER2
                | Options::TIMER0
                | Options::TIMER1
                | Options::SPI
                | Options::USART0
                | Options::ADC => {
                    core::ptr::write_volatile(&mut self.prr0, prr);
                }

                Options::TIMER5
                | Options::TIMER4
                | Options::TIMER3
                | Options::USART3
                | Options::USART2
                | Options::USART1 => {
                    core::ptr::write_volatile(&mut self.prr1, prr);
                }
            }
        }
    }
}
