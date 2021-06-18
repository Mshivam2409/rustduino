---
id: gpio
slug: /gpio
title: GPIO
---
- A general-purpose input/output (GPIO) is an uncommitted digital signal pin on an   integrated circuit or electronic circuit board which may be used as an input or output, or both, and is controllable by the user at runtime.
GPIOs have no predefined purpose and are unused by default. If used, the purpose and behavior of a GPIO is defined and implemented by the designer of higher assembly-level circuitry.
- When a pin is in GPIO mode, software has control over the high/low state of an output pin and direct read access to the state of an input pin. This is in contrast to the pin being controlled by a dedicated hardware function, such as a serial port.
- A GPIO port is a platform-defined grouping of GPIO pins (often 4 or more pins). However, GPIO pins that are part of a GPIO port cannot be retrieved or controlled individually as GPIO pins.
- In order to use a specific pin or port, an application should first open and obtain a GPIO Pin or GPIOPort instance for the pin or port it wants to use, using its numerical ID, name, type (interface), or properties.

#### IMPLEMENTATION:
GPIO interfaces vary widely. In some cases, they are simple—a group of pins that can switch as a group to either input or output. In others, each pin can be set up to accept or source different logic voltages, with configurable drive strengths and pull ups/downs. Input and output voltages are usually, but not always, limited to the device's supply voltage with the GPIOs, and may be damaged by greater voltages.
A GPIO pin's state may be exposed to the software developer through a number of different interfaces, such as a memory-mapped I/O peripheral or through dedicated IO port instructions. 
A GPIO port is a group of GPIO pins (usually 8 GPIO pins) arranged in a group and controlled as a group.
GPIO abilities may include:
GPIO pins can be configured to be input or output
GPIO pins can be enabled/disabled
Input values are readable (usually high or low)
Output values are writable/readable
Input values can often be used as IRQs (usually for wakeup events)

# Function Definitions:
- GPIO struct basically represents a single pin and GPIObitband where pin and gpiobitband itself are structs.

```rust 
pub struct Gpio {
    gpio: *mut GpioBitband,
    pin: usize
}
```
### Implementations:
### Impl `make_gpio` for `GPIO`

```rust
impl Pin {
    pub fn make_gpio(self) -> Gpio {
        unsafe {
         // {some fields}
        }
    }
}
```
This returns  GPIO for a specific pin through its address in PCR arrays.