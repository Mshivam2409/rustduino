---
id: generate_mpu
slug: /generate_mpu
title: MPU Number Generation
---

We will use the math library functions to demonstrate 
the generation of random numbers:

- We first disable the watchdog for smooth functioning.
- Create a structure object to control the functions.
- Call the respective function for number generation by MPU6050 sensor read value.

This all ends up being surprisingly short in main though the algorithm behind the implementation is
very big as could be seen in the source code:

```rust
pub fn main() {
    // Disable the watchdog.
    let wdog = unsafe { WatchDog::new() };
    wdog.disable();

    let mut rand = RandomNumberGenerator::new(Generator::Mpu);

    loop {
        
        // Generate Random numbers by MPU6050 gyroscopic sensor.
        // This number could be sent to peripheral device using USART.
        let _y: u8 = rand.generate_by_mpu();
    }
}
```
