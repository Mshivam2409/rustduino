---
id: analog
slug: /analog
title: Analog Integrated Circuit
---

# Introduction 

The ATmega 2560p features a 10-bit Analog to digital converter.The ADC being 
connected to 8/16-channel Analog mltiplexer allow 8/16 single ended voltage inputs.

The device also supports 16/32 differential voltage input combinations.Four of which
are equipped with programmable gain stage, providing amplification steps of 0 dB, 20 
dB, 46 dB on the differential input voltage before the ADC conversion.The 16 channels 
are split into two sections of 8 in each of which 7 differential analog input channel 
share a common negative terminal, while any other ADC input in that section can be 
selected as the positive input terminal.

The ADC contains a sample and hold circuit which ensures that the input volatge to
the ADC is held at a constant level during conversion, it also has a seperate analog 
supply pin, AVCC.Internal Reference voltages of nominally 1.1V, 2.56V or AVCC are 
provided On-chip.

**Note** that the Power Reduction ADC bit, PRADC, must be disabled to enable ADC.

## Operation

The ADC converts the voltage on the analog input to 10 bit digital value by 
approximation, where minimum value is GND and maximum is voltage on AREF pin minus 1
LSB. The internal reference voltage may be connected by writing to the REFSn bits in
ADMUX Register. 

The analog input channel is selected by writing to the MUX bits in ADMUX and ADCSRB. 
Any of the ADC input pins, as well as GND and a fixed bandgap voltage reference, can
be selected as single ended inputs to the ADC.The ADC generates a 10-bit result which
is presented in the ADC Data Registers, ADCH and ADCL.

## Analog Read

Before enabling ADC, set the PRADC(Power Reduction ADC bit) to zero in prr0 register 
to disable it.

```rust
    pub fn power_adc_disable(&mut self) {
        unsafe {
            let pow = Power::new();
            write_volatile(&mut pow.prr0, pow.prr0 & (254));
        }
    }
```

Then to enable the Analog to digital converter, write one to the ADEN bit in the 
ADCSRA register.

```rust
    pub fn adc_enable(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(7, true);
        });
    }

```
Also we need to make sure that auto triggering for the ADC is off, and setup the 
prescaler by choosing the division factor between the XTAL frequency and ADC
frequency.

```rust
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

    pub fn adc_auto_trig(&mut self) {
        self.adcsra.update(|aden| {
            aden.set_bit(5, false);
        });
    }    
```
Finally setup the ADC input pins by using the MUX bits and also make digital input 
buffer on that pin to be disabled to reduce power consumption. Depending on which
pin you want, you can set the MUX bits and DIDR bit. 
```rust                   
        analog.admux.update(|admux| {
            admux.set_bits(0..3,**);
        });
        analog.didr2.update(|didr2| {
            didr2.set_bit(**, true);
        });
        analog.adcsrb.update(|mux| {
            mux.set_bit(3,**);
        });
```
Then we can start the conversion by writing one to the aden bit in ADCSRA register.
```rust
        self.adcsra.update(|aden| {
            aden.set_bit(6, true);
        });
```

Since we are doing first conversion it takes 25 ADC cycles as first conversion also
include setting ADC, then give output as a u32 whose first 10 bits represent the 
final result which is stored in the ADCH and ADCL register. then finally you can 
disable the ADC by writing 0 to ADEN bit.
```rust
            let mut i: i32 = 25;
            let adcsra = analog.adcsra.read();

            while adcsra.get_bit(4) == true {
                if i != 0 {
                    i = i - 1;
                    __nop();
                    __nop(); //add delay of system clock
                } else {
                    unreachable!()
                }
            }
            let mut a: u32 = 0;
            a.set_bits(0..8, analog.adcl.read() as u32);

            a.set_bits(8..10, analog.adch.read() as u32);

        self.adcsra.update(|aden| {
            aden.set_bit(7, false);
        });
```

The Function to analog read is inside AnalogPin impl, it gives u32 as return value with 10 
of its bits as the result of conversion.
```rust
pub fn read(&mut self) -> u32
```
## Analog Reference

We also need to select the voltage reference for your Analog to Digital Converter, it 
could be internal or external.depending on what you want to select you could by 
manipulating the REFS bits in the ADMUX Register. It takes parameter which Reference
we want DEFAULT, Internal 1.1V, Internal 2.56V or External. 
```rust
pub fn analog_reference(reftype: RefType) {
    let analog = unsafe { Analog::new() };
    match reftype {
        RefType::DEFAULT => {
            analog.admux.update(|admux| {
                admux.set_bits(6..8, 0b01);
            });
        }
        RefType::INTERNAL1V1 => {
            analog.admux.update(|admux| {
                admux.set_bits(6..8, 0b10);
            });
        }
        RefType::INTERNAL2V56 => {
            analog.admux.update(|admux| {
                admux.set_bits(6..8, 0b11);
            });
        }
        RefType::EXTERNAL => {
            analog.admux.update(|admux| {
                admux.set_bits(6..8, 0b00);
            });
        }
    }
}
```

## Analog write