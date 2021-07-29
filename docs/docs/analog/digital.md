---
id: digital
slug: /digital
title: Digital
---

 This source code creates a array for controlling all digital pins at one place in form
 Pins array which would be used so that we get meaningful functions to work upon and
 also the implementation of rustduino library is easier for the user.
 For more details see section 16,17,25 and 26 of ATMEGA2560P datasheet.

```rust
/// Include the required source codes before struct creation
use crate::hal::pin::*;
use core::ptr::{read_volatile, write_volatile};
```

## Structure 
This struct contain digital pin and its corresponding digital pin no.

```rust
pub struct DigitalPin {
    pub pin: Pin,
    pub pinno: u32,
}
```

## Implementations for `DigitalPin`

### impl `toggle`

 Toggles the appropriate bit in PINxn register so that the mode of the pin
 is changed from high to low or vice versa.

```rust
    pub fn toggle(&mut self) {
        unsafe { write_volatile(&mut (*self.pin.port).pin, 0x1 << self.pin.pin) }
    }
```
#### Usage

```rust
   use rustduino::atmega2560p::hal::pin::Pins;
   let mut pins = Pins::new();// This pins represents pin 7 of port B ( pin 13).
   pins.digital[13].toggle();   
```

### impl `high`
Set the pin to high output value.

```rust
   pub fn high(&mut self) { /* fields omitted */}
        
```
#### Usage
```rust
   pins.digital[13].high();   //This sets pin high.

```

### impl `low`
Sets the pin to low output value.
```rust
    pub fn low(&mut self) {/* fields omitted */}
```
#### Usage 
```rust
   pins.digital[13].low();   //This sets pin low.

```