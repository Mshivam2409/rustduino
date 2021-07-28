---
id: Temperature Sensor
slug: /Temperature Sensor
title: Reading Values from a Temperature Sensor.
---

To get temperature and humidity reading from the sensor, our code will perform the following opertions.

* Triggers the slave.
* Waits for it to be idle.
* Reads data to the buffer.
* Reads 20 bit raw humidity data and returns relative humidity.
* Same are the steps for Tempereature reading too.

> Note: *I2C Protocol is used for communication between master and slave.*

All these steps are implemented in just few lines of driver code.

```rust
fn main() {
    let sensor = AHT10::new(&mut AHT10::get());

    loop {
        // Get relative humidity.
        sensor.relative_humidity();

        // Get temperature
        sensor.temperature();

        // Waiting for 2 seconds.
        rustduino::delay::delay_ms(2000);
    }
} // Getting temperature and relative humidity at 2 second interval
```

