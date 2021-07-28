---
id: analog_io
slug: /analog_io
title: Analog Input and Output
---

This example shows how to receive analog input and give output in wave form via the arduino our code will perform the following opertions.
 * Disabling watchdog.
 * Creates a array object consisting of all the pins.
 * Executing infinite loop for read and write continuously through the I/O pins.
 * In the loop, Take input into the zeroth analog pin.
 * Make the input value ready to be sent through a digital pin.
 * Give output from the 13th digital pin.

 All these steps are implemented in just few lines of code

```rust
pub fn main() {
    
    let watchdog = unsafe { WatchDog::new() };
    watchdog.disable();

    let mut pins = Pins::new();

    loop {
        
        let a: u32 = pins.analog[0].read();

        let b: u8 = map(a as u64, 0, 255, 0, 1023) as u8;

        rustduino::delay::delay_ms(1000);

        pins.digital[13].write(b);

        rustduino::delay::delay_ms(1000);
    }
}
```