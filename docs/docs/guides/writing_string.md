---
id: writing_string
slug: /writing_string
title: Continuous Write using USART
---





```rust
pub fn main() {
    // Disable watchdog
    let watchdog = unsafe { WatchDog::new() };
    watchdog.disable();

    // Create a new Serial struct to access all USART's
    let mut serial = unsafe { Serial::new() };

    // This initializes USART0 and makes it ready to transmit and recieve.
    unsafe { serial.usart[0].begin() };

    // Loop to send a string using the USART multiple times.
    let mut i: u8 = 100;
    while i != 0 {
        // This sends string from arduino through TxD0 pin.
        serial.usart[0].write_string("Hello World!");

        rustduino::delay::delay_ms(1000);

        i = i - 1;
    }

    // This disables USART0.
    unsafe { serial.usart[0].end() };
}
```