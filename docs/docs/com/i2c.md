---
id: i2c
slug: /i2c
title: I2C
---

## Struct definition

```rust
   pub struct Twi{/*fields ommited*/}
```   

Twi represents a struct containing register definitions for Twi.

## Function Definition

```rust
   pub fn write_sda()
```
This function sets DDRC to write direction.

```rust
   pub fn read_sda()
```
This function sets DDRC to read direction.

## Trait implementation

### Implement `new` for `Twi`
```rust
    pub fn new() -> &'static mut Self
```
This function returns a pointer to TWBR.TWBR selects the division factor for the bit rate generator. The bit rate generator is a frequency divider which generates the SCL clock frequency in the Master modes.    

### Implement `wait_to_complete` for `Twi`
```rust
     pub fn wait_to_complete(&mut self, start: u8) -> bool
```
This function waits for TWI to be ready. It returns true if TWI is ready i.e. the process is complete, otherwise returns false.     

### Implement `init` for `Twi`
```rust
    pub fn init(&mut self)
```
This function initializes the TWI Bus.

### Implement `start` for `Twi`
```rust
    pub fn start(&mut self) -> bool
```
The first step in a TWI transmission is to transmit a START condition. This is done by writing a specific value
into TWCR, instructing the TWI hardware to transmit a START condition.Writing a one to TWINT
clears the flag. The TWI will not start any operation as long as the TWINT bit in TWCR is set. Immediately
after the application has cleared TWINT, the TWI will initiate transmission of the START condition.
This function sends a start signal.It returns true if the START condition has been transmitted, otherwise false.

### Implement `stop` for `Twi`
```rust
   pub fn stop(&mut self) 
```
This function transmits the Stop condition.

### Implement `rep_start` for `Twi`
```rust
   pub fn rep_start(&mut self) -> bool
```
A special case occurs when a new START condition is issued between a START and STOP condition. This is
referred to as a REPEATED START condition, and is used when the Master wishes to initiate a new transfer without relinquishing control of the bus. After a REPEATED START, the bus is considered busy until the next STOP.
This function sends the repeated start signal. .It returns true if the repeated START condition has been transmitted, otherwise false.

### Implement `address_read` for `Twi`
```rust
    pub fn address_write(&mut self, address: u8) -> bool 
```
This function loads the address of the slave device on SDA. The address argument passed into the function is a seven bit integer. It returns true if the process was successful, otherwise returns false. 

### Implement '

   