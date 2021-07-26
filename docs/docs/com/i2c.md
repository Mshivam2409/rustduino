---


id: i2c
slug: /i2c
title: I2C
---

# I2C

The I2C protocol also known as the two wire interface is a simple serial communication protocol that uses just two pins of a microcontroller namely SCL (serial clock) and SDA (serial data). This is a very popular protocol that can be used to address a large number of slave devices that are connected to the same bus. This protocol comes in handy when there is scarcity of available pins in the microcontroller. Each slave device has a slave address or a name for which they respond. This is usually a 7-bit binary number. Once a master sends a valid slave address, that slave alone will respond to the master’s queries and all other slaves will ignore any conversation between the master and that particular slave.

There are a number of conditions that can be made over the I2C bus such as start and stop sequence. The data line does not change when the clock line is HIGH. If the data line changes when the clock line is High, the slave device interprets it as a command and not as data. This is the only feature in the interface that puts a distinct line between the command and data.



# Struct definition

```rust
pub struct Twi{/*files omitted*/}
```

Twi represents a struct containing registers for TWI namely:

* `TWBR`: *TWI Bit Rate Register*.
* `TWSR`: *TWI Status Register*.
* `TWAR`: *TWI (Slave) Address Register*.
* `TWDR`: *TWI Data Register*.
* `TWSR`: *TWI Status Register*.
* `TWAMR`: *TWI Address Mask*.

More about there registers and their functions can be found out at [Section 21.9 of ATmega328P datasheet.](t.ly/dBh5).

#### Usage:

```rust
use crate::com::i2c;
let mut i2c = i2c::Twi;

// Here i2c is a mutable variable of Type i2c.
```



## Function Definition

These functions set the data direction of port C to either read or write.

```rust
pub fn write_sda()
```

Sets DDRC to write direction.

```rust
pub fn read_sda()
```

Sets DDRC to write direction.

#### Usage:

```rust
use crate::com::i2c;

i2c::write_sda() // to set the data direction to write.

i2c::read_sda() // to set the data direction to read.
```



## Trait implementation

## Impl `new` for `Twi`

```rust
pub fn new() -> &'static mut Sel
```

#### Usage:

```rust
use rustduino::com::i2c;
let mut twi = i2c::new()

// Here twi is the pointer to the struct Twi.
// This twi pointer is used further for different functions.
```

Returns a pointer to the struct Twi.



### Impl `wait_to_complete` for `Twi`

```rust
pub fn wait_to_complete(&mut self, operation: u8) -> bool 
```

Waits for the ongoing process to be complete. List of processes include:

* `Start`
* `Repeat Start`
* `Master Trasmitter Slave Acknoledgement`
* `Master Trasmitter Data Acknoledgement`
* `Master Receiver Slave Acknoledgement`
* `Master Receiver Data Acknoledgement`
* `Master Receiver Slave No-Acknoledgement`

Returns `True` if process was successful.



### Impl `init` for `Twi`

```rust
pub fn init(&mut self)
```

#### Usage:

```rust
twi.init();
```

Initiates the Twi bus.



### Impl `start` for `Twi`

```rust
pub fn start(&mut self) -> bool
```

#### Usage:

```rust
twi.start();
```

Sends a start signal to the slave and returns true if operation is successful.



### Impl `rep_start` for `Twi`

```rust
pub fn rep_start(&mut self) -> bool 
```

#### Usage:

```rust
twi.rep_start();
```

Sends a Repeat Start Signal to the Slave and returns `True` is process is successful.

Repeated Start condition is one which allows a master to continue the current transaction without losing atomicity. This is achieved by *NOT* sending a stop after the transaction but sending a Start in its place.



### Impl `stop` for `Twi`

```rust
pub fn stop(&mut self) 
```

#### Usage:

```rust
twi.stop();
```

Stops the data transfer between master and slave and sets slave as free. Marks the end of transaction with the slave device.



### Impl `set_address` for `Twi`

```rust
pub fn set_address(&mut self, addr: u8) -> bool
```

#### Usage:

```rust
let addr = // 7 bit address of the slave;
twi.set_address(addr);
```

Sets address of Slave and returns `True` if operation is successful.

Here, the slave address with the write specifier is sent after the Start sequence. The slave sends an Acknowledge to the master (MCU). Then the next byte of data written to the slave device is the address of the register to write to.



### Impl `address_read` for `Twi`

```rust
pub fn address_read(&mut self, address: u8) -> bool 
```

#### Usage:

```rust
let addr = // 7 bit address of the slave;
twi.address_read(addr);
```

Checks if slave is acknowledged and returns `True` if operation is successful.



### Impl `write` for `Twi`

```rust
pub fn write(&mut self, data: u8) -> bool
```

#### Usage:

```rust
let addr = // 7 bit address of the slave;
let data = // 8 bit integer;
twi.init();             // Sets prescaler bits
twi.start();			// Starts the TWI bus.
twi.set_address(addr);  // Sets the address of the slave.
twi.write(data);		// Writes one byte data to the slave
twi.stop();				// Sends a stop signal
```



### Impl `write_burst`for `Twi`

```rust
pub fn write_burst(&mut self, data: &FixedSliceVec<u8>) -> usize
```

#### Usage:

```rust
let addr = // 7 bit address of the slave;
let data = // FixedSliceVec<u8>;
twi.init();             	// Sets prescaler bits
twi.start();				// Starts the TWI bus.
twi.set_address(addr);  	// Sets the address of the slave.
twi.write_burst(& data);	// Writes consecutive data bytes to the slave.
twi.stop();					// Sends a stop signal
```

Writes consecutive bytes of data to the Slave and returns number of bytes written.



### Impl `write_to_slave` for `Twi`

```rust
pub fn write_to_slave(&mut self, address: u8, data: &FixedSliceVec<u8>) -> bool 
```

#### Usage:

```rust
let addr = // 7 bit address of the slave;
let data = // FixedSliceVec<u8>;

twi.write_to_slave(addr, & data);
```

 This function:

* *Starts the Twi Bus* : Returns false if start fails.
* *Sets slave address* : Returns false if slave does not acknowledge on address being set.
* *Writes all data bytes to the salve* : Aborts and returns false if all data bytes were not written.
* Returns true if all the above three steps are successful and also sends a stop singal to the slave.



### Impl `read_from_slave` for `Twi`

```rust
pub fn read_from_slave( &mut self, address: u8, length: usize, data: &mut FixedSliceVec<u8>) -> bool 
```

#### Usage:

```rust
let addr = 		// 7 bit address of the slave;
let length = 	// usize int corresponding to the data length wanted to be read from slave.
let data = 		// FixedSliceVec<u8>;

twi.read_from_slave(addr, length, &mut data);
```

 This function:

* *Starts the Twi Bus* : Returns false if start fails.
* *Reads slave address* : Returns false if slave does not acknowledge.
* *Reads data bytes from the salve* : Aborts and returns false if the length of data specified is not read.
* Returns true if all the above three steps are successful and also sends a stop singal to the slave.
