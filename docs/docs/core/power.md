---
id: power
slug: /power
title: Power Modes
---

## Enum Description

```rust
pub enum Options {/*fields omitted */}  // "Options" corresponds to "Peripherals" enum in atmega328p code
```
The `Options` correspond to real world as shown -
* `TWI`    :  *Power Reduction TWI*
* `TIMER2` :  *Power Reduction Timer/Counter2*
* `TIMER0` :  *Power Reduction Timer/Counter0*
* `TIMER1` :  *Power Reduction Timer/Counter1*
* `SPI`    :  *Power Reduction Serial Peripheral Interface*
* `USART0` :  *Power Reduction USART0*
* `ADC`    :  *Power Reduction ADC*
* `TIMER5` :  *Power Reduction Timer/Counter5*
* `TIMER4` :  *Power Reduction Timer/Counter4*
* `TIMER3` :  *Power Reduction Timer/Counter3*
* `USART3` :  *Power Reduction USART3*
* `USART2` :  *Power Reduction USART2*
* `USART1` :  *Power Reduction USART1*

## Struct Definitions

```rust
pub struct Power {/* fields omitted */}
```
Contains registers to control the functioning of clocks in the chip.
It would be used to control the power modes of the chip as mentioned
in the enum `Options` above.

* `PRR` – Power Reduction Register contains control bits for power control through clock gating.

### Trait Implementations

### Impl `new` for `Power`

```rust
    pub unsafe fn new() -> &'static mut Power {
        &mut *(0x64 as *mut Power)
    }
```
 Creates a new reference to the Sleep structure at a specified location.

### Impl `disable_clocks` for `Power`

 ```rust
    pub fn disable_clocks(&mut self, mode: Options) {/* fields omitted */}

 ```

 This is the function for disabling the clock system of your choice.
 It would create a new element of the structure power
 which would be used to control various clock gating features of the chip.
 All the clock features are implemented in this function using match cases.
 Please specify the type of power reduction mode to be used as the mode,
 use the standard keywords.

 ### Usage:

 ```rust
    use rustduino::hal::power;
    power::disable_clocks(Options::/*mode*/);
    // here mode is from the given enum list.
 ```

### Impl `enable_clocks` for `Power`

 ```rust
    pub fn enable_clocks(&mut self, mode: Options) {/* fields omitted */}
 ```

 This is the function for enabling the clock system of your choice.

 ### Usage:

 ```rust
    power::disable_clocks(Options::/*mode*/);
 ```
