---
id: port
slug: /port
title: Ports and Pins
---

*Our Communicators*

---

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

## Struct Definitions

## Port

```rust
  pub struct Port {/*feilds ommited*/}
```

Port represents a struct containing the register definition for a Port namely:

- `pin`: _Port input pins_. Writing a logic one to PINxn toggles the value of PORTxn, independent on the value of DDRxn.

- `ddr`: _Data direction register_. The DDxn bit in the DDRx register selects the direction of this pin. If DDxn is written logic one, Pxn is configured as an output pin. If DDxn is written logic zero, Pxn is configured as an input pin.

- `port`: _Data register_. If PORTxn is written logic one when the pin is configured as an input pin, the pull-up resistor is activated. To switch the pull-up resistor off, PORTxn has to be written logic zero or the pin has to be configured as an output pin. The port pins are tri-stated when reset condition becomes active, even if no clocks are running. If PORTxn is written logic one when the pin is configured as an output pin, the port pin is driven high (one). If PORTxn is written logic zero when the pin is configured as an output pin, the port pin is driven low (zero).

More about these registers can be found in Section 13.2.1 and 13.2.2 of ATmega328P datasheet and Section 13.2 to 13.4 of ATMEGA2560P datasheet.

## Pin

```rust
  pub struct Pin {/*feilds ommited*/}
```

Pin represents struct corresponding to a pin namely:

- `port` : pointer of type struct Port
- `pin` : Pin number of type usize

## Enums

## PortName

```rust
  pub enum Portname {/*feilds ommited*/}
```

Lists all ports available on the chip.

- Ports _A to D ***except A***_ are available on atmega328p.
- Ports _A to L ***except I***_ are available on atmega2560p.

## Trait Implementations

### Impl `new` for `Port`

```rust
  pub fn new(name: PortName) -> &'static mut Port
```

#### Usage:

```rust
  use rustduino::hal::port::*;
  let mut port = Port::new(Portname::B) // here B is the port name.

  // port is the pointer to the struct.
  // we will use the port variable for demonstration of other functions also.
```

Matches a valid PortName and returns a struct containing register definition of the chosen Port.

### Impl `pin ` for `Port`

```rust
pub fn pin(&mut self, p: usize) -> Pin
```

#### Usage:

```rust
port.pin(pin number:u8) // here pin number can range from 0 to 7
```

Returns a pin struct for given pin.

### Impl `set_pin_mode` for `Port`

```rust
pub fn set_pin_mode(&mut self, p: usize, mut mode: u32)
```

Sets the pin mode, given the pin and requested mode.

### Impl `make_gpio` for `Pin`

```rust
pub fn make_gpio(self) -> Gpio
```

Sets pin to GPIO mode
