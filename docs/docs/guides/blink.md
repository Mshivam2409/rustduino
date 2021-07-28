---
id: blink
slug: /blink
title: Blinking an Led
---

Following the core conepts, to blink an led, our application will do the
following:

- Disable the watchdog
- Turn on the clock gate for Port C
- Grab pin 5 from that port, and make it a GPIO
- Set that GPIO high to light the LED, and low to turn it off.

This all ends up being surprisingly short in main:

```rust
pub fn main() {
    let (wdog,sim,pin) = unsafe {
        (watchdog::Watchdog::new(),
         sim::Sim::new(),
         port::Port::new(port::PortName::C).pin(5))
    };

    wdog.disable();
    sim.enable_clock(sim::Clock::PortC);

    let mut gpio = pin.make_gpio();

    gpio.output();

    loop {
       gpio.high();
       gpio.low();
    }
}
```
