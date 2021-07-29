---
id: shift
slug: /shift
title: Shift Register
---

## Enum definition

```rust
pub enum BitOrder{/*files omitted*/}
```

Helper enum for bit order of the value. 

* `LSBFIRST` : Least significant bit first.
* `MSBFIRST` : Most significant bit first.

## Function Definition

### `shift_in`

```rust
pub fn shift_in(datapin: u8, clockpin: u8, bit_order: BitOrder) -> u8 
```

#### Usage:

```rust
use rustduino::avr::shift::*;
let data:u8 = 2;
let clock:u8 = 3;
let bit_order = BitOrder::MSBFIRST;

let value:u8 = shift_in(data, clock, bit_order);
```

Returns the value stored in the shift register.



### `shift_out`

```rust
pub fn shift_out(datapin: u8, clockpin: u8, bit_order: BitOrder, mut value:u8)
```

#### Usage:

```rust
use rustduino::avr::shift::*;
let data:u8 = 2;
let clock:u8 = 3;
let bit_order = BitOrder::MSBFIRST;
let mut value:u8 = 0b11001010;

let value:u8 = shift_out(data, clock, bit_order, value);
```

Shifts out the 8 bits to the shift register.