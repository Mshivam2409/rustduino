---
id: analog
slug: /analog
title: Analog
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


## Structure definition
---
* Structure to control the implementation of Integrated Analog Circuit which contain ACSR (Analog Comparator Control and Status Register)
```rust
pub struct AnalogComparator {
    acsr: Volatile<u8>,
}
```
* Structure to control data transfer from Analog to Digital signal conversions.
```rust
pub struct Analog {/*fields omitted */ }
```
* Structure to control the timer of type 8 for Analog Write.
```rust
pub struct Timer8 {/* fields omitted */ }
```  

* Structure to control the timer of type 16 for Analog Write.
```rust
pub struct Timer16 {/* fields omitted */ }
```   

### Impl `new` for `Timer8`
This creates a memory mapped IO for timer 8 type which will assist in analog write.
```rust
    pub fn new(timer: TimerNo8) -> &'static mut Timer8 {/* fields omitted */ } 
```

### Impl `new` for `Timer16`
This creates a memory mapped IO for timer 16 type which will assist in analog write.
```rust
    pub fn new(timer: TimerNo16) -> &'static mut Timer16 {/* fields omitted */}
```
### Impl `new` for `AnalogComparator`       
New pointer object created for Analog Comparator Structure.
```rust
    pub unsafe fn new() -> &'static mut AnalogComparator {
        &mut *(0x50 as *mut AnalogComparator)
    }
```

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
frequency. This is done by setting bits in ADCSRA register'S ADPS0,ADPS1,ADPS2.

```rust
pub fn analog_prescaler(&mut self, factor: u8) {/*fiels omitted */}
```
    pub fn adc_auto_trig(&mut self) {

When Bit 5 – ADATE: ADC Auto Trigger Enable is written to one, Auto Triggering of the ADC is enabled.
The ADC will start a conversion on a positive edge of the selected trigger signal. The trigger source
is selected by setting the ADC Trigger Select bits, ADTS in ADCSRB.

```rust
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
Writing this bit to one enables the ADC. By writing it to zero, the ADC is turned off. 
Turning the ADC off while a conversion is in progress, will terminate this conversion.

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

The Function to analog `read` is inside `AnalogPin` impl, it gives u32 as return value with 10 
of its bits as the result of conversion, all the upper functions are inside this read 
function and will automatically setup and convert the analog to digital.
```rust
pub fn read(&mut self) -> u32{/*fields omitted */}
```
## Analog Reference

We also need to select the voltage reference for your Analog to Digital Converter, it 
could be internal or external.depending on what you want to select you could by 
manipulating the REFS bits in the ADMUX Register. It takes parameter which Reference
we want DEFAULT, Internal 1.1V, Internal 2.56V or External. If these bits are changed during a
conversion, the change will not go in effect until this conversion is complete (ADIF in ADCSRA is set). The internal
voltage reference options may not be used if an external reference voltage is being applied to the AREF pin.

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

We have to output or write a PWM wave to a digital pin, which can be 2 to 13 or 44 
to 46 other pins will not work. Of these pins, 4 and 13 give output at 980 hertz 
while all pin except 4 and 13 are set to give output at 490 hertz.

The analog Write function takes in the value generated by mapping the 10 bit result
from analog Read to a 8 bit number.
```rust
pub fn 
```

The result is then given as a parameter to the analog Write function, which first 
check which pin to give output to and thenn accordingly set the tccra and tccrb 
register. The pins 4, 13, 9 and 10 feature timer of 8 bits while the others 
feature a 16 bit timer.

The we setup the Waveform Generation mode which is Fast PWM for our purpose, 
maximum counter value and the clock source to be used by the Timer/Counter we
set prescaling to obtain clock source.
```rust
 pub fn write(&mut self, value1: u8) {
        self.pin.output();

        let pin1 = self.pinno;

        match pin1 {
            4 | 13 => {
                let timer = Timer8::new(TimerNo8::Timer0);
                timer.tccra.update(|ctrl| {
                    ctrl.set_bits(0..2, 0b11);
                });
                timer.tccrb.update(|ctrl| {
                    ctrl.set_bits(0..4, 0b0011);
                });

                if pin1 == 4 {
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
                let timer = Timer8::new(TimerNo8::Timer2);
                timer.tccra.update(|ctrl| {
                    ctrl.set_bits(0..2, 0b11);
                });
                timer.tccrb.update(|ctrl| {
                    ctrl.set_bits(0..4, 0b0101);
                });
                if pin1 == 9 {
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
            11 | 12 => {
                let timer = Timer16::new(TimerNo16::Timer1);
                timer.tccra.update(|ctrl| {
                    ctrl.set_bits(0..2, 0b01);
                });
                timer.tccrb.update(|ctrl| {
                    ctrl.set_bits(0..5, 0b00011);
                });
                if pin1 == 12 {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(2..8, 0b001000);
                    });
                    timer.ocrbl.write(value1);
                } else {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(2..8, 0b100000);
                    });
                    timer.ocral.write(value1);
                }
            }
            2 | 3 | 5 => {
                let timer = Timer16::new(TimerNo16::Timer3);
                timer.tccra.update(|ctrl| {
                    ctrl.set_bits(0..2, 0b01);
                });
                timer.tccrb.update(|ctrl| {
                    ctrl.set_bits(0..5, 0b00011);
                });

                if pin1 == 2 {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(2..8, 0b001000);
                    });
                    timer.ocrbl.write(value1);
                } else if pin1 == 5 {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(2..8, 0b100000);
                    });
                    timer.ocral.write(value1);
                } else {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(2..8, 0b000010);
                    });
                    timer.ocrcl.write(value1);
                }
            }
            6 | 7 | 8 => {
                let timer = Timer16::new(TimerNo16::Timer4);
                timer.tccra.update(|ctrl| {
                    ctrl.set_bits(0..2, 0b01);
                });
                timer.tccrb.update(|ctrl| {
                    ctrl.set_bits(0..5, 0b00011);
                });

                if pin1 == 7 {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(2..8, 0b001000);
                    });
                    timer.ocrbl.write(value1);
                } else if pin1 == 6 {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(2..8, 0b100000);
                    });
                    timer.ocral.write(value1);
                } else {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(2..8, 0b000010);
                    });
                    timer.ocrcl.write(value1);
                }
            }
            44 | 45 | 46 => {
                let timer = Timer16::new(TimerNo16::Timer3);
                timer.tccra.update(|ctrl| {
                    ctrl.set_bits(0..2, 0b01);
                });
                timer.tccrb.update(|ctrl| {
                    ctrl.set_bits(0..5, 0b00011);
                });

                if pin1 == 45 {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(2..8, 0b001000);
                    });
                    timer.ocrbl.write(value1);
                } else if pin1 == 46 {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(2..8, 0b100000);
                    });
                    timer.ocral.write(value1);
                } else {
                    timer.tccra.update(|ctrl| {
                        ctrl.set_bits(2..8, 0b000010);
                    });
                    timer.ocrcl.write(value1);
                }
            }
            _ => unreachable!(),
        }
    }
```