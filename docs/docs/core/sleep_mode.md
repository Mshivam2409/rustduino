---
id: sleep_mode
slug: /sleep_mode
title: Power Modes Global Adjustments
---

_Let it rest for sometime_

---

Power consumption is a critical issue for a device running continuously for a long time without being turned off. So to overcome this problem almost every controller comes with a sleep mode, which help developers to design electronic gadgets for optimal power consumption. Sleep mode puts the device in power saving mode by turning off the unused module.

# Enum Description

```rust
    pub enum SleepMode {/*fields ommited*/}
```

Contains the following Power Modes:

- `Idle` : This mode makes the MCU enter idle mode, stopping the CPU but allowing the SPI, USART, analog comparator, ADC, 2-wire serial interface, Timer/Counters, watchdog, and the interrupt system to continue operating. This sleep mode basically halts clkCPU and clkFLASH, while allowing the other clocks to run.
- `ADCNR` : ADC Noise Reducion mode makes the MCU enter ADC noise reduction mode, stopping the CPU but allowing the ADC, the external interrupts, the 2-wire serial interface address watch, Timer/Counter2, and the watchdog to continue operating (if enabled). This sleep mode basically halts clkI/O, clkCPU, and clkFLASH, while allowing the other clocks to run.
- `Power Down` : Power Down mode makes the MCU enter power-down mode. In this mode, the external oscillator is stopped, while the external interrupts, the 2-wire serial interface address watch, and the watchdog continu operating (if enabled). Only an external reset, a watchdog system reset, a watchdog interrupt, a brown-out reset, a 2-wire serial interface address match, an external level interrupt on INT0 or INT1 or a pin change interrupt can wake up the MCU. This sleep mode basically halts all generated clocks, allowing operation of asynchronous modules only.
- `Power Save` : Power Save mode is identical to Power Down, with one exception, If Timer/Counter2 is enabled, it will keep running during sleep. The device can wake up from either timer overflow or output compare event from Timer/Counter2 if the corresponding Timer/Counter2 interrupt enable bits are set in TIMSK2, and the global interrupt enable bit in SREG is set.
- `Standby` : It is identical to Power Down, with one exception, If Timer/Counter2 is enabled, it will keep running during sleep. The device can wake up from either timer overflow or output compare event from Timer/Counter2 if the corresponding Timer/Counter2 interrupt enable bits are set in TIMSK2, and the global interrupt enable bit in SREG is set.
- `ExtStandby` : Extendend Standby mode is identical to Power Save with the exception that the oscillator is kept running. From extended standby mode, the device wakes up in six clock cycles.

## Struct Definitions

```rust
    pub struct Sleep {/*feilds ommited*/}
```

Struct containing registers to control sleep modes namely :

- `SMCR` : The sleep mode control register contains control bits for power management. More about this register and sleep modes can be found out at [Section 9.11 of ATmega328p Datasheet](https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf) or [Section 11.10.1 of ATmega2560p Datasheet](https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf).

### Trait Implementations

### Impl `enable_mode` for `Sleep`

```rust
pub fn enable_mode(mode: SleepMode)
```

#### Usage:

```rust
use rustdiono::hal::power;

power::enable_mode(SleepMode::/*mode*/);
// here mode is a power from the given list.
```

Used to enable one of the sleep modes.

- _Idle_
- _ADCNR_
- _PowerDown_
- _Standby_
- _ExtStandby_
- _Disable_

Description about all the power modes can be found at [SleepModes](#Enum Description).
