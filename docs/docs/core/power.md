---
id: power
slug: /power
title: Power Modes Fine Adjustments
---

*Let's adjust the star rating*

---

We can adjust specific settings in the power consumption of the AVR chip without depending upon the standard modes provided by the AVR corporation inbuilt as various sleep modes.The library also has various implementations to control those specific features and allow or dis-allow the functioning of different Peripherals (like USART) which could be attached to the micro-controller.

## Enum Description

```rust
   pub enum Peripherals {/*fields omitted */}  
```
The `Peripherals` correspond to real world as shown -
* `TWI`    :  *Power Reduction TWI*
* `ADC`    :  *Power Reduction ADC*
* `SPI`    :  *Power Reduction Serial Peripheral Interface*
* `TIMER2` :  *Power Reduction Timer/Counter2*
* `TIMER0` :  *Power Reduction Timer/Counter0*
* `TIMER1` :  *Power Reduction Timer/Counter1*
* `TIMER3` :  *Power Reduction Timer/Counter3*
* `TIMER4` :  *Power Reduction Timer/Counter4*
* `TIMER5` :  *Power Reduction Timer/Counter5*
* `USART0` :  *Power Reduction USART0*
* `USART3` :  *Power Reduction USART3*
* `USART2` :  *Power Reduction USART2*
* `USART1` :  *Power Reduction USART1*

**Note that USART1,USART2,USART3,TIMER3,TIMER4 and TIMER5 are not applicable for ATMEGA328P**


## Struct Definitions

```rust
   pub struct Power {/* fields omitted */}
```
Contains registers to control the functioning of clocks in the chip.
It would be used to control the power modes of the chip as mentioned
in the enum `Peripherals` above.

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
   pub fn disable_clocks(&mut self, mode: Peripherals) {/* fields omitted */}

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
   power::disable_clocks(Peripherals::/*mode*/);
   // here mode is from the given enum list.
```

### Impl `enable_clocks` for `Power`

```rust
   pub fn enable_clocks(&mut self, mode: Peripherals) {/* fields omitted */}
```

This is the function for enabling the clock system of your choice.

 ### Usage:

```rust
   power::disable_clocks(Peripherals::/*mode*/);
```
