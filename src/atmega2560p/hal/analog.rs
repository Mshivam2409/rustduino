//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021 Aniket Sharma, Indian Institute of Technology Kanpur
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

//! This is the implementation for Analog Referencing in Integrated circuit of AVR Chips.  
//! This code is written taking into account the features available in ATMEGA2560P.
//! This code implements the Analog Read function to read from the buffer using analog signals.
//! This code implements the Analog Write function to write into the buffer using analog signals.
//! Refer to section 25 and 26 of ATMEGA2560P datasheet.
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf

use bit_field::BitField;
/// Crates to be used for the implementation.
use volatile::Volatile;
use crate::atmega2560p::hal::port::{Port,Pin};
/// Structure to control the implementation of Integrated Analog Circuit.
#[repr(C, packed)]
pub struct AnalogComparator {
    acsr: Volatile<u8>,
}

/// Structure to control the digital signal access.
#[repr(C, packed)]
pub struct Digital {
    didr2: Volatile<u8>,
    didr0: Volatile<u8>,
    didr1: Volatile<u8>,
}

/// Structure to control data transfer from Analog to Digital signal conversions.
#[repr(C, packed)]
pub struct Analog {
    adcl: Volatile<u8>,
    adch: Volatile<u8>,
    adcsra: Volatile<u8>,
    adcsrb: Volatile<u8>,
    admux: Volatile<u8>,
}
pub enum RefType {
    DEFAULT,
    INTERNAL1V1,
    INTERNAL2V56,
    EXTERNAL,
}
pub enum PIN{
    ADC0,
    ADC1,
    ADC2,
    ADC3,
    ADC4,
    ADC5,
    ADC6,
    ADC7,
    ADC8,
    ADC9,
    ADC10,
    ADC11,
    ADC12,
    ADC13,
    ADC14,
    ADC15,
}

pub enum Aldar{
    L,
    R,
}

impl AnalogComparator {
    /// New pointer object created for Analog Comparator Structure.
    pub unsafe fn new() -> &'static mut AnalogComparator {
        &mut *(0x50 as *mut AnalogComparator)
    }
}

impl Digital {
    /// New pointer object created for Digital Structure.
    pub unsafe fn new() -> &'static mut Digital {
        &mut *(0x7D as *mut Digital)
    }
}

impl Analog {
    /// New pointer object created for Analog Structure.
    pub unsafe fn new() -> &'static mut Analog {
        &mut *(0x78 as *mut Analog) // check correct address
    }

    /// Function to create a reference for Analog signals.
    pub fn analog_reference(&mut self, reftype: RefType) {
        match reftype {
            RefType::DEFAULT => {
                self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b00);
                });
            }
            RefType::INTERNAL1V1 => {
                self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b10);
                });
            }
            RefType::INTERNAL2V56 => {
                self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b11);
                });
            }
            RefType::EXTERNAL => {
                self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b01);
                });
            }
        }
    }
    
    ///Function is Used to enable the ADC
    pub fn adc_enable (&mut self){

        self.adcsra.update(|aden| {
            aden.set_bit(7,true);
        });
    
    }

    ///Function is Used to start a conversion in the ADC
    pub fn adc_con_start (&mut self){

        self.adcsra.update(|aden| {
            aden.set_bit(6,true);
        });
    
    }    

    ///Function is Used to stop auto triggering in the ADC
    pub fn adc_auto_trig (&mut self){

        self.adcsra.update(|aden| {
            aden.set_bit(5,false);
        });
    
    }    

    /// Function to read data which is got as input to Analog Pins.
    pub fn analog_read(&mut self,pin: PIN,aldar: Aldar) {

    // Write 1 to analog pins' DIDR bit of corresponding analog pin

    match aldar{
        Aldar::L=>{self.admux.update(|admux| {
        admux.set_bit(5,true);
        });
        }
        Aldar::R=>{self.admux.update(|admux| {
        admux.set_bit(5,false);
        });
        }
    }

    match pin{
        PIN::ADC0=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b000);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3, false);
            });
        }
        PIN::ADC1=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b001);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,false);
            });
        }
        PIN::ADC2=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b010);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,false);
            });
        }
        PIN::ADC3=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b011);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,false);
            });
        }
        PIN::ADC4=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b100);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,false);
            });
        }
        PIN::ADC5=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b101);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,false);
            });
        }
        PIN::ADC6=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b110);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,false);
            });
        }
        PIN::ADC7=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b111);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,false);
            });
        }
        PIN::ADC8=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b000);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,true);
            });
        }
        PIN::ADC9=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b001);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,true);
            });
        }
        PIN::ADC10=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b010);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,true);
            });
        }
        PIN::ADC11=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b011);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,true);
            });
        }
        PIN::ADC12=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b100);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,true);
            });
        }
        PIN::ADC13=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b101);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,true);
            });
        }
        PIN::ADC14=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b110);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,true);
            });
        }
        PIN::ADC15=>{
            self.admux.update(|admux| {
                admux.set_bits(0..3,0b111);
            });
            
            self.adcsrb.update(|mux| {
                mux.set_bit(3,true);
            });
        }
    }

    }

    ///Function is Used to disable the ADC
    pub fn adc_disable (&mut self){

        self.adcsra.update(|aden| {
            aden.set_bit(7,false);
        });
    
    }

    /// Function to write data as an output through Analog Pins.
    pub fn analog_write() {}
}
