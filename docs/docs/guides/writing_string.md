---
id: writing_string
slug: /writing_string
title: Continuous Write using USART
---

To write string continuously using USART, our code will perform the following opertions.
 * Disable watchdog
 * Create a new Serial struct to access all USART's
 * initialize USART0 and makes it ready to transmit and recieve.
 * Loop to send a string using the USART multiple times.This sends string from arduino through TxD0 pin.
 * Disable USART0.


```rust
pub fn main() {
    let watchdog = unsafe { WatchDog::new() };
    watchdog.disable();

    let mut serial = unsafe { Serial::new() };

    unsafe { serial.usart[0].begin() };

    let mut i: u8 = 100;
    while i != 0 {
        serial.usart[0].write_string("Hello World!");

        rustduino::delay::delay_ms(1000);

        i = i - 1;
    }
    
    unsafe { serial.usart[0].end() };
}
```