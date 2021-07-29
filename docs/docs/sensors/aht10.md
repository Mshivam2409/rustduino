---
id: aht10
slug: /aht10
title: AHT10 Sensor
---

AHT10, the new generation of temperature and humidity sensors sets a new standard in size and intelligence: it is embedded for reflow soldering. This sensor gives a calibrated digital signal outputs in standard I2C format. The AHT10 is equipped with a newly designed ASIC-specific chip, an improved MEMS semiconductor capacitive humidity sensing element and a standard on-chip temperature sensing element. Its performance has been greatly improved beyond the reliability level of previous generation sensors.
More about the sensor and its functions can be found out at [Aosong AHT 10 Datasheet](https://server4.eca.ir/eshop/AHT10/Aosong_AHT10_en_draft_0c.pdf).

# Struct definition

```rust
pub struct AHT10<'a> {/*fields omitted*/}
```

#### Usage:

```rust
use rustduino::sensors::aht10;
let mut aht10 = aht10::AHT10;

// Here aht10 is a mutable variable of Type aht10.
```

## Trait implementation

### Impl `new` for `AHT10`

```rust
pub fn new(&mut self) -> &'static mut Self
```

Returns pointer to the struct .

#### Usage:

```rust
use rustduino::sensors::aht10;
let mut aht10 = aht10::new()

// Here aht10 is the pointer to the struct AHT10.
// This aht10 pointer is used further for different functions.
```

### Impl `initialise` for `AHT10`

```rust
pub fn initialise(&mut self) -> bool
```

#### Usage:

```rust
let aht10 = rustduino::sensors::AHT10::new();
aht10.initialise();
```

Initiates the transmission by self initiating the sensor.

Returns true if done otherwise false.

### Impl `soft_reset` for `AHT10`

```rust
pub fn soft_reset(&mut self)
```

#### Usage:

```rust
let aht10 = rustduino::sensors::AHT10::new();
aht10.soft_reset();
```

- Restart sensor, without power off.
- The sensor system begins to reinitialize and restores the default settings after this command
- The function takes less then 20 ms

### Impl `read_to_buffer` for `AHT10`

```rust
pub fn read_to_buffer(&mut self)
```

#### Usage:

```rust
let aht10 = rustduino::sensors::AHT10::new();
aht10.read_to_buffer();
```

- Reads data from slave.

### Impl `trigger_slave` for `AHT10`

```rust
pub fn trigger_slave(&mut self)
```

#### Usage

```rust
let aht10 = rustduino::sensors::AHT10::new();
aht10.trigger_slave();
```

- Triggers the AHT10 to read temperature/humidity.

### Impl `wait_for_idle` for `AHT10`

```rust
pub fn wait_for_idle(&mut self)
```

#### Usage:

```rust
let aht10 = rustduino::sensors::AHT10::new();
aht10.wait_for_idle();
```

- Causes delay of 5ms when status bit is 0 and sensor is busy.

### Impl `perform_measurement` for `AHT10`

```rust
pub fn perform_measurement(&mut self)
```

#### Usage:

```rust
let aht10 = rustduino::sensors::AHT10::new();
aht10.perform_measurement();
```

- Performs measurement .

### Impl `status` for `AHT10`

```rust
pub fn status(&mut self) -> u8
```

#### Usage:

```rust
let aht10 = rustduino::sensors::AHT10::new();
aht10.status();
```

- Reads status bit returned by the slave.

### Impl `relative_humidity` for `AHT10`

```rust
pub fn relative_humidity(&mut self) -> f32
```

#### Usage:

```rust
let aht10 = rustduino::sensors::AHT10::new();
aht10.relative_humidity();
```

- Reads 20 bit raw humidity data and returns relative humidity.

### Impl `temperature` for `AHT10`

```rust
pub fn temperature(&mut self) -> f32
```

#### Usage:

```rust
let aht10 = rustduino::sensors::AHT10::new();
aht10.temperature();
```

- Reads 20 bit raw temperature data and returns temperature.
