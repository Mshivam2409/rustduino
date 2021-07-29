---
id: generate
slug: /generate
title: Random Number Generation
---

We will use the math library functions to demonstrate 
the generation of random numbers:

- We first disable the watchdog for smooth functioning.
- Create a structure object to control the functions.
- Call the respective functions for number generation.

This all ends up being surprisingly short in main though the algorithm behind the implementation is
very big as could be seen in the source code:

```rust
pub fn main() {
    // Disable the watchdog.
    let wdog = unsafe { WatchDog::new() };
    wdog.disable();

    let mut rand = RandomNumberGenerator::new();

    loop {
        // Generate Random numbers using Analog pin inputs.
        // This number could be sent to peripheral device using USART.
        let _x: u8 = rand.generate_by_analog();

        // Generate Random numbers by MPU6050 gyroscopic sensor.
        // This number could be sent to peripheral device using USART.
        let _y: u8 = rand.generate_by_mpu();
    }
}
```
