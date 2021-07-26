---
id: aht10
slug: /aht10
title: AHT10 Sensor
---

# Struct definition

```rust
pub struct AHT10<'a> {/*fields omitted*/}
```

## Trait implementation

### Impl `new` for `AHT10`

```ru
pub fn new(&mut self) -> &'static mut Self
```

Returns pointer to the struct .

#### Impl `initialise` for `AHT10`

```rust
pub fn initialise(&mut self) -> bool
```

Initiates the transmission by self initiating the sensor.

Returns true if done otherwise false.

#### Impl `soft_reset` for `AHT10`

```rust
pub fn soft_reset(&mut self)
```

Restart sensor, without power off.

#### Impl `read_to_buffer` for `AHT10`

```rust
pub fn read_to_buffer(&mut self)
```

Reads data from slave.

#### Impl `trigger_slave` for `AHT10`

```rust
pub fn trigger_slave(&mut self)
```

Triggers the AHT10 to read temperature/humidity.

#### Impl `wait_for_idle` for `AHT10`

```rust
pub fn wait_for_idle(&mut self)
```

Causes delay of 5ms when status bit is 0 and sensor is busy.

#### Impl `perform_measurement` for `AHT10`

```rust
pub fn perform_measurement(&mut self)
```

Performs measurement .

#### Impl `status` for `AHT10`

```rust
pub fn status(&mut self) -> u8
```

Reads status bit returned by the slave.

#### Impl `relative_humidity` for `AHT10`

```rust
pub fn relative_humidity(&mut self) -> f64
```

Reads 20 bit raw humidity data and returns relative humidity in percentage.

#### Impl `temperature` for `AHT10`

```rust
pub fn temperature(&mut self) -> f64
```

Reads 20 bit raw temperature data and returns temperature in degree celsius .



