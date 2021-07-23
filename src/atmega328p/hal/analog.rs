//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Ayush Agarwal, Indian Institute of Technology Kanpur
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
//! This code is written taking into account the features available in ATMEGA328P.
//! This code implements the Analog Read function to read from the buffer using analog signals.
//! This code implements the Analog Write function to write into the buffer using analog signals.
//! Refer to section 22 and 23 of ATMEGA328P datasheet.
//! https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf

use crate::atmega328p::hal::port::*;
use crate::atmega328p::hal::power::Sleep;
/// Crates to be used for the implementation.
use bit_field::BitField;
use core::ptr::write_volatile;
use volatile::Volatile;

/// Structure to control the implementation of Integrated Analog Circuit.
#[repr(C, packed)]
pub struct AnalogComparator {
    acsr: Volatile<u8>,
}

/// Structure to control the digital signal access.
#[repr(C, packed)]
pub struct Digital {
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
    didr0: Volatile<u8>,
    didr1: Volatile<u8>,
}

pub enum RefType {
    DEFAULT,
    INTERNAL1V1,
    EXTERNAL,
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
        &mut *(0x7E as *mut Digital)
    }
}

impl Pin {
    /// Function to create a reference for Analog signals.
    pub fn analog_read(&mut self, pin: u32, reftype: RefType) -> u32 {
        unsafe {
            let analog = Analog::new();

            analog.power_adc_disable(); //To enable ADC
            
            analog.adc_enable();

            analog.adc_auto_trig();

            analog.analog_reference(reftype);

            match pin {
                0 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b000);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(0, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                1 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b001);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(1, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                2 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b010);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(2, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                3 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b011);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(3, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                4 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b100);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(4, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                5 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b101);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(5, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                6 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b110);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(6, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                7 => {
                    analog.admux.update(|admux| {
                        admux.set_bits(0..3, 0b111);
                    });
                    analog.didr0.update(|didr0| {
                        didr0.set_bit(7, true);
                    });
                    analog.adcsrb.update(|mux| {
                        mux.set_bit(3, false);
                    });
                }
                _ => unreachable!(),
            }

            analog.adc_con_start();

            // wait 25 ADC cycles
            let mut a: u32 = 0;
            let adcl = analog.adcl.read() as u32;
            a.set_bits(0..8, adcl.get_bits(0..8));

            let adch = analog.adch.read() as u32;
            a.set_bits(8..10, adch.get_bits(0..2));

            analog.adc_disable();

            a
        }
    }

    pub fn analog_write(&mut self, _pi: u32, _val: u32) {
        self.set_output();
    }
}

impl Analog {
    /// New pointer object created for Analog Structure.
    pub unsafe fn new() -> &'static mut Analog {
        &mut *(0x78 as *mut Analog)
    }

    /// Function to create a reference for Analog signals.
    pub fn analog_reference(&mut self, reftype: RefType) {
        match reftype {
            RefType::DEFAULT => {
                self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b01);
                });
            }
            RefType::INTERNAL1V1 => {
                self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b10);
                });
            }
            RefType::EXTERNAL => {
                self.admux.update(|admux| {
                    admux.set_bits(6..8, 0b00);
                });
            }
        }
    }

    ///Function to disable PRADC
    pub fn power_adc_disable(&mut self) {
        unsafe {
            let pow = Sleep::new();
            write_volatile(&mut pow.smcr, pow.smcr & (254));
        }
    }

    ///Function to enable PRADC
    pub fn power_adc_enable(&mut self) {
        unsafe {
            let pow = Sleep::new();
            write_volatile(&mut pow.smcr, pow.smcr | (1));
        }
    }

    ///Function is Used to enable the ADC
    pub fn adc_enable(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(7, true);
        });
    }

    ///Function is Used to start a conversion in the ADC
    pub fn adc_con_start(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(6, true);
        });
    }

    ///Function is Used to stop auto triggering in the ADC
    pub fn adc_auto_trig(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(5, false);
        });
    }

    ///Function is Used to disable the ADC
    pub fn adc_disable(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(7, false);
        });
    }
}
