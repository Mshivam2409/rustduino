---


id: i2c
slug: /i2c
title: I2C
---

# Struct definition

```rust
pub struct Twi{/*files omitted*/}
```

Twi represents a struct containing registers for TWI.

## Function Definition

```rust
pub fn write_sda()
```

Sets DDRC to write direction.

```rust
pub fn read_sda()
```

Sets DDRC to write direction.

## Trait implementation

## Impl `new` for `Twi`

```rust
pub fn new() -> &'static mut Self
```

Returns  pointer to TWBR.

### Impl `wait_to_complete` for `Twi`

```rust
pub fn wait_to_complete(&mut self, operation: u8) -> bool 
```

Waits for the process to be complete.

Returns if process was successful.

#### Impl `init` for `Twi`

```rust
pub fn init(&mut self)
```

Initiates the Twi bus.

##### Impl `start` for `Twi`

```rust
pub fn start(&mut self) -> bool
```

Sends a start signal.

##### Impl `rep_start` for `Twi`

```rust
pub fn rep_start(&mut self) -> bool 
```

Sends a Repeat Start Signal

##### Impl `stop for `Twi`

```rust
pub fn stop(&mut self) 
```

Stops the Twi bus.

##### Impl `set_address` for `Twi`

```rust
pub fn set_address(&mut self, addr: u8) -> bool
```

Sets address of Slave.

Returns true if process is successful

##### Impl `address_read` for `Twi`

```rust
pub fn address_read(&mut self, address: u8) -> bool 
```

Checks if slave is acknowledged.

Returns true if process is successful

##### Impl `write` for `Twi`

```rust
pub fn write(&mut self, data: u8) -> bool
```

Writes one byte of data to the Slave. Need to set address first.

Returns true if process is successful

##### Impl `write_burst`for `Twi`

```rust
pub fn write_burst(&mut self, data: &FixedSliceVec<u8>) -> usize
```

Writes consecutive bytes of data to the Slave. Need to set address first.

 Returns number of bytes written

##### Impl `write_to_slave` for `Twi`

```rust
pub fn write_to_slave(&mut self, address: u8, data: &FixedSliceVec<u8>) -> bool 
```

 Writes consecutive Data bytes to slave.

 Returns true if process is completed and aborts if any of the steps.

##### Impl `start` for `Twi`

```rust
pub fn read_from_slave(
        &mut self,
        address: u8,
        length: usize,
        data: &mut FixedSliceVec<u8>,
    ) -> bool 
```

Reads consecutive Data bytes from slave.

Returns true if process is completed and aborts if any of the steps.
