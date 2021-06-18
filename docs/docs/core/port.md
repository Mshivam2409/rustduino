---
id: ports
slug: /ports
title: Ports and Pins
---

# Introduction

In computer programming a port is a communication endpoint. It completes the
destination or origination network address of a message. Specific port numbers
are reserved to identify specific services so that an arriving packet can be
easily forwarded to a running application.

Every microcontroller has pins in it to attach power connections, input and
output connections, and communications connections. Every microcontroller has
different configurations for its pins, and often one pin will have more than one
function.

Pins are grouped into ports, and all of a pin’s settings are controlled from the
port’s register block. We’d like each pin to be a self-contained struct, so that
ownership of it can be passed from one software module to another, and only the
owning module can mutate its pins. This follows Rust’s one-owner rule for pins,
but would require that each pin be able to mutate its settings in the Port
register block.

## Function Definitions - Port represents a struct containing the register definition for a Port.

```rust
  pub struct Port {/*feilds ommited*/}
```

- Pin represents struct corresponding to a pin

```rust
  pub struct Pin {/*feilds ommited*/}
```

## Implementations ### Impl `new` for `Port`

```rust
pub fn new(name: PortName) -> &'static mut Port
```

Return a struct containing register definition of the Port.

### Impl `set_pin_mode` for `Port`

```rust
   pub fn set_pin_mode(&mut self, p: usize, mut mode: u32)
```

Sets the pin mode, given the pin and requested mode.

### Impl `pin ` for `Port`

```rust
  pub fn pin(&mut self, p: usize) -> Pin
```

Returns a pin struct for given pin.

### Impl `name` for `Port`

```
      pub fn name(&self) -> PortName
```

Returns Port struct with the corresponding port address.

### Impl `make_gpio` for `Pin`

```
      pub fn make_gpio(self) -> Gpio
```

Sets pin to GPIO mode
