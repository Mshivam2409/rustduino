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
//! Refer to section 14,15,22 and 23 of ATMEGA328P datasheet.

// Crates to be used for the implementation.
use bit_field::BitField;
use volatile::Volatile;

use crate::atmega328p::hal::pin::{AnalogPin, DigitalPin};
// Source codes to be used here.
use crate::atmega328p::hal::sleep_mode::Sleep;

/// Selection of reference type for the implementation of Analog Pins.
#[derive(Clone, Copy)]
pub enum RefType {
    DEFAULT,
    INTERNAL1V1,
    EXTERNAL,
}

/// Selection of timer mode for Timer 8 type.
#[derive(Clone, Copy)]
pub enum TimerNo8 {
    Timer0,
    Timer2,
}

/// Selection of timer mode for Timer 16 type.
#[derive(Clone, Copy)]
pub enum TimerNo16 {
    Timer1,
}

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

/// Structure to control the timer of type 8 for Analog Write.
pub struct Timer8 {
    tccra: Volatile<u8>,
    tccrb: Volatile<u8>,
    _tcnt: Volatile<u8>,
    ocra: Volatile<u8>,
    ocrb: Volatile<u8>,
}

/// Structure to control the timer of type 16 for Analog Write.
pub struct Timer16 {
    tccra: Volatile<u8>,
    tccrb: Volatile<u8>,
    _tccrc: Volatile<u8>,
    _pad0: u8,
    _tcntl: Volatile<u8>,
    _tcnth: Volatile<u8>,
    _icrl: Volatile<u8>,
    _icrh: Volatile<u8>,
    ocral: Volatile<u8>,
    _ocrah: Volatile<u8>,
    ocrbl: Volatile<u8>,
    _ocrbh: Volatile<u8>,
}

/// Structure to control power settings for the timer/counter
#[repr(C, packed)]
pub struct Power {
    prr: Volatile<u8>,
}

impl Power {
    /// Create a memory mapped IO for Power type which will assist in enabling timer/counter.
    /// # Arguments
    /// * no parameters reqauired!
    /// # Returns
    /// * `a reference to Power object` - which will be used for further implementations.
    pub unsafe fn new() -> Power {
        &mut *(0x64 as *mut Timer8)
    }
}

// Structure to control the timer of type 8 for Analog Write.
impl Timer8 {
    /// Create a memory mapped IO for timer 8 type which will assist in analog write.
    /// # Arguments
    /// * `timer` - a `TimerNo8` object, which defines the Timer number for which object is to be made.
    /// # Returns
    /// * `a reference to Timer8 object` - which will be used for further implementations.
    pub fn new(timer: TimerNo8) -> &'static mut Timer8 {
        match timer {
            TimerNo8::Timer0 => unsafe { &mut *(0x44 as *mut Timer8) },
            TimerNo8::Timer2 => unsafe { &mut *(0xB0 as *mut Timer8) },
        }
    }
}

// Structure to control the timer of type 16 for Analog Write.
impl Timer16 {
    /// Create a memory mapped IO for timer 16 type which will assist in analog write.
    /// # Arguments
    /// * `timer` - a `TimerNo16` object, which defines the Timer number for which object is to be made.
    /// # Returns
    /// * `a reference to Timer16 object` - which will be used for further implementations.
    pub fn new(timer: TimerNo16) -> &'static mut Timer16 {
        match timer {
            TimerNo16::Timer1 => unsafe { &mut *(0x80 as *mut Timer16) },
        }
    }
}

impl AnalogComparator {
    /// New pointer object created for Analog Comparator Structure.
    /// # Returns
    /// * `a reference to AnalogComparator object` - which will be used for further implementations.
    pub unsafe fn new() -> &'static mut AnalogComparator {
        &mut *(0x50 as *mut AnalogComparator)
    }
}

impl Digital {
    /// Creates a pointer object for Digital Structure.
    /// # Returns
    /// * `a reference to Digital object` - which will be used further.
    pub unsafe fn new() -> &'static mut Digital {
        &mut *(0x7E as *mut Digital)
    }
}

impl AnalogPin {
    /// Read the signal input to the analog pin.
    /// Any analog pin can be freely used for this purpose.
    /// # Returns
    /// `a u32` - Value read from the analog pin.
    pub fn read(&mut self) -> u32 {
        let pin = self.pinno;
        unsafe {
            let analog = Analog::new();

            analog.power_adc_disable(); //To enable ADC

            analog.adc_enable();

            analog.adc_auto_trig();

            analog.analog_prescaler(2);

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
            a.set_bits(0..8, analog.adcl.read() as u32);

            a.set_bits(8..10, analog.adch.read() as u32); // check logic syntax correctness

            analog.adc_disable();

            a
        }
    }
}

impl DigitalPin {
    /// This is used to write a PWM wave to a digital pin.
    /// Only 2-13 and 44-46 digital pins can be used in this function, other pins will lead to crash.
    /// All pin except 4 and 13 are set to give output at 490 hertz.
    /// pin 4 and 13 will give output at 980 hertz.
    /// # Arguments
    /// * `value1` - a u8, value to be written on the analog pin for output.
    pub fn write(&mut self, value1: u8) {
        self.pin.set_output();
        let pin1 = self.pinno;
        match pin1 {
            5 | 6 => {
                unsafe {
                    let pow = Power::new();
                    write_volatile(&mut pow.prr, pow.prr & (247));
                }
                let timer = Timer8::new(TimerNo8::Timer0);
                timer.tccra.update(|ctrl| {
                    ctrl.set_bits(0..2, 0b11);
                });
                timer.tccrb.update(|ctrl| {
                    ctrl.set_bits(0..4, 0b0011);
                });

                if pin1 == 5 {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(4..8, 0b0010);
                    });
                    timer.ocrb.write(value1);
                } else {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(4..8, 0b1000);
                    });
                    timer.ocra.write(value1);
                }
            }
            11 | 3 => {
                unsafe {
                    let pow = Power::new();
                    write_volatile(&mut pow.prr, pow.prr & (247));
                }
                let timer = Timer8::new(TimerNo8::Timer2);
                timer.tccra.update(|ctrl| {
                    ctrl.set_bits(0..2, 0b01);
                });
                timer.tccrb.update(|ctrl| {
                    ctrl.set_bits(0..4, 0b0100);
                });

                if pin1 == 11 {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(4..8, 0b0010);
                    });
                    timer.ocrb.write(value1);
                } else {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(4..8, 0b1000);
                    });
                    timer.ocra.write(value1);
                }
            }
            9 | 10 => {
                unsafe {
                    let pow = Power::new();
                    write_volatile(&mut pow.prr, pow.prr & (247));
                }
                let timer = Timer16::new(TimerNo16::Timer1);
                timer.tccra.update(|ctrl| {
                    ctrl.set_bits(0..2, 0b01);
                });
                timer.tccrb.update(|ctrl| {
                    ctrl.set_bits(0..5, 0b00011);
                });

                if pin1 == 9 {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(4..8, 0b0010);
                    });
                    timer.ocrbl.write(value1);
                } else if pin1 == 10 {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(4..8, 0b1000);
                    });
                    timer.ocral.write(value1);
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Analog {
    /// New pointer object created for Analog Structure.
    /// # Returns
    /// * `a reference to Analog object` - which will be used for further implementations.
    pub unsafe fn new() -> &'static mut Analog {
        &mut *(0x78 as *mut Analog)
    }

    /// Used to enable the Analog to Digital Converter.
    pub fn adc_enable(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(7, true);
        });
    }

    /// Function to enable power after using ADC.
    pub fn power_adc_enable(&mut self) {
        {
            let pow = Power::new();
            self.prr.update(|aden| {
                aden.set_bit(0, true);
            });
        }
    }

    /// Function to disable power after using ADC.
    pub fn power_adc_disable(&mut self) {
        {
            let pow = Power::new();
            self.prr.update(|aden| {
                aden.set_bit(0, false);
            });
        }
    }

    /// Used to start a conversion in the ADC.
    pub fn adc_con_start(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(6, true);
        });
    }

    /// Used to stop auto triggering in the ADC.
    pub fn adc_auto_trig(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(5, false);
        });
    }

    /// Used to disable the ADC.
    pub fn adc_disable(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(7, false);
        });
    }

    /// Set prescaler for the ADC.
    /// # Arguments
    /// * `factor` - a u8, the prescaler power frequency factor to be set.
    pub fn analog_prescaler(&mut self, factor: u8) {
        match factor {
            2 => {
                self.adcsra.update(|adcsra| {
                    adcsra.set_bits(0..3, 0b000);
                });
            }
            4 => {
                self.adcsra.update(|adcsra| {
                    adcsra.set_bits(0..3, 0b010);
                });
            }
            8 => {
                self.adcsra.update(|adcsra| {
                    adcsra.set_bits(0..3, 0b011);
                });
            }
            16 => {
                self.adcsra.update(|adcsra| {
                    adcsra.set_bits(0..3, 0b100);
                });
            }
            32 => {
                self.adcsra.update(|adcsra| {
                    adcsra.set_bits(0..3, 0b101);
                });
            }
            64 => {
                self.adcsra.update(|adcsra| {
                    adcsra.set_bits(0..3, 0b110);
                });
            }
            128 => {
                self.adcsra.update(|adcsra| {
                    adcsra.set_bits(0..3, 0b111);
                });
            }
            _ => unreachable!(),
        }
    }
}

/// Function to create a reference for Analog signals.
/// # Arguments
/// * `reftype` - a `RefType` object, the type of reference setup required for the analog pins.
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
