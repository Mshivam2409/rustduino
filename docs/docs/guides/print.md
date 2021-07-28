---
id: print
slug: /print
title: Writing a string using USART Print
---





```rust
pub fn main() {
    // Disable watchdog
    let watchdog = unsafe { WatchDog::new() };
    watchdog.disable();

    // Send's the required message to the peripheral device at interval of 2 seconds.
    loop {
        // Print function to transmit string through USART.
        println_string("Hello World !!!");

        rustduino::delay::delay_ms(2000);
    }
}
```