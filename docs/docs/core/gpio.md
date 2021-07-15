---
id: gpio
slug: /gpio
title: GPIO
---

## Introduction

---

- A general-purpose input/output (GPIO) is an uncommitted digital signal pin on
  an integrated circuit or electronic circuit board which may be used as an
  input or output, or both, and is controllable by the user at runtime. GPIOs
  have no predefined purpose and are unused by default.
- When a pin is in GPIO mode, software has control over the high/low state of an
  output pin and direct read access to the state of an input pin. This is in
  contrast to the pin being controlled by a dedicated hardware function, such as
  a serial port.
- A GPIO port is a platform-defined grouping of GPIO pins. However, GPIO pins
  that are part of a GPIO port cannot be retrieved or controlled individually as
  GPIO pins.
- In order to use a specific pin or port, you should first open and obtain a
  GPIO Pin or GPIO Port instance for the pin or port you want to use.

### IMPLEMENTATION:

---

GPIO interfaces are simple—a group of pins that can switch as a group to either
input or output. A GPIO port is a group of GPIO pins arranged in a group and
pins in that can't be contolled individually. GPIO abilities may include:

- GPIO pins can be configured to be input or output
- GPIO pins can be enabled/disabled
- Input values are readable (usually high or low)
- Output values are writable/readable

## Function Definitions:

---

GPIO struct represents a single pin and GPIObitband _( ARM Bit Band Technology)_

```rust
pub struct Gpio {
    gpio: *mut GpioBitband,
    pin: usize
}
```

### Implementations:

---

### Impl `make_gpio` for `Pin`

```rust
pub fn make_gpio(self) -> Gpio
```

Converts a Pin into GPIO mode.
