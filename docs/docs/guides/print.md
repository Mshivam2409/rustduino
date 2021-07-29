---
id: print
slug: /print
title: Writing a string using USART Print
---

For Writing a string using USART Print, our code will perform the following opertions.
 - Disabling the watchdog.
 - Executing the loop for sending the required message to the peripheral device at interval of 2 seconds.
 - Print function to transmit string through USART.

All these steps are implemented in just few lines of code.
```rust

pub fn main() {
   
    let watchdog = unsafe { WatchDog::new() };
    watchdog.disable();

    loop {
        println_string("Hello World !!!");

        rustduino::delay::delay_ms(2000);
    }
}
```