---
id: analog
slug: /analog
title: Analog
---

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


## Structure definitions

---

- Structure to control the implementation of Integrated Analog Circuit which contain ACSR (Analog Comparator Control and Status Register)

```rust
pub struct AnalogComparator {
    acsr: Volatile<u8>,
}
```

- Structure to control data transfer from Analog to Digital signal conversions.

```rust
pub struct Analog {/*fields omitted */ }
```

- Structure to control the timer of type 8 for Analog Write.

```rust
pub struct Timer8 {/* fields omitted */ }
```

- Structure to control the timer of type 16 for Analog Write.

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

## Analog

### Impl `new`

New pointer object created for Analog Structure.

```rust
    pub unsafe fn new() -> &'static mut Analog {/* fields omitted */}
```

#### Usage

```rust
use rustduino::hal::analog;
let analog = Analog::new();
```

### Impl `power_adc_disable`

Before enabling ADC, set the PRADC(Power Reduction ADC bit) to zero in prr0 register
to disable it.

```rust
    pub fn power_adc_disable(&mut self) {/* fields omitted */}
```

#### Usage

```rust
use rustduino::hal::analog;
let analog = Analog::new();
analog.power_adc_disable();  //PRADC disable to enable ADC
```

### Impl `adc_enable`

Then to enable the Analog to digital converter, write one to the ADEN bit in the
ADCSRA register.

```rust
    pub fn adc_enable(&mut self) {/* fields omitted */}
```

#### Usage

```rust
use rustduino::hal::analog;
let analog = Analog::new();
analog.adc_enable();
```

### Impl `analog_prescaler`

Also we need to make sure that auto triggering for the ADC is off, and setup the
prescaler by choosing the division factor between the XTAL frequency and ADC
frequency. This is done by setting bits in ADCSRA register'S ADPS0,ADPS1,ADPS2.

```rust
pub fn analog_prescaler(&mut self, factor: u8) {/*fiels omitted */}
```

#### Usage

```rust
use rustduino::hal::analog;
let analog = Analog::new();
analog.analog_prescaler(2);
```

### impl `adc_auto_trig`

When Bit 5 – ADATE: ADC Auto Trigger Enable is written to one, Auto Triggering of the ADC is enabled.
The ADC will start a conversion on a positive edge of the selected trigger signal. The trigger source
is selected by setting the ADC Trigger Select bits, ADTS in ADCSRB.

```rust
 pub fn adc_auto_trig(&mut self) {/* fields omitted */}
```

#### Usage

```rust
use rustduino::hal::analog;
let analog = Analog::new();
analog.adc_auto_trig();
```

### impl `adc_con_start`

Then we can start the conversion by writing one to the aden bit in ADCSRA register.
Writing this bit to one enables the ADC. By writing it to zero, the ADC is turned off.
Turning the ADC off while a conversion is in progress, will terminate this conversion.

```rust
    pub fn adc_con_start(&mut self) {/* fields omitted */}
```

#### Usage

```rust
use rustduino::hal::analog;
let analog = Analog::new();
analog.adc_con_start();
```

## Analog Pin

### Impl `read` for `AnalogPin`

The Function to analog `read` is inside `AnalogPin` impl, it gives u32 as return value with 10
of its bits as the result of conversion, all the upper functions are inside this read
function and will automatically setup and convert the analog to digital.

```rust
pub fn read(&mut self) -> u32{/*fields omitted */}

```

#### Usage

```rust
    let mut pins = Pins::new();
    let a: u32 = pins.analog[0].read(); // Take input into the zeroth analog pin.
```

## Analog Reference

We also need to select the voltage reference for your Analog to Digital Converter, it
could be internal or external.depending on what you want to select you could by
manipulating the REFS bits in the ADMUX Register. It takes parameter which Reference
we want DEFAULT, Internal 1.1V, Internal 2.56V or External. If these bits are changed during a
conversion, the change will not go in effect until this conversion is complete (ADIF in ADCSRA is set). The internal
voltage reference options may not be used if an external reference voltage is being applied to the AREF pin.

```rust
pub fn analog_reference(reftype: RefType) {/* fields omitted */}

```