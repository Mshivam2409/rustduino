---
id: shift
slug: /shift
title: Shift Register
---

# Enum definition

```rust
pub enum BitOrder{/*files omitted*/}
```

Helper enum for bit order of the value.

## Function definition

```rust
fn make_pin(pin: u8) -> Pin 
```

Makes pin struct , given the pin number.

```rust
pub fn shift_in(datapin: u8, clockpin: u8, bit_order: BitOrder) -> u8 
```

Returns the value stored in the shift register.

```rust
pub fn shift_out(datapin: u8, clockpin: u8, bit_order: BitOrder, mut value: u8)
```

Stores value in the Shift Register.
