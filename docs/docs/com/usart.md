---
id: usart
slug: /usart
title: USART
---
---
The Universal Synchronous and Asynchronous serial Receiver and Transmitter (USART) is a highly flexible serial
communication device. By definition, USART is a hardware communication protocol that uses synchronous or asynchronous
serial communication with configurable speed and Data Format. Asynchronous communication does not use a clock to validate data.

## Serial
---
This struct contains USARTs arranged in a array. Each USART can be accesed through Serial.usart[n], where 0<= n <=3 in atmega2560p. And
in atmega328p it contains on USART0 i.e n=0.

```rust
   pub struct Serial {/* fields omitted */}
```
### impl `new` for `Serial`

```rust
   pub fn new() -> Serial {/* fields omitted */ }
```

## Initialization of USART
---

This structure contains various registers needed to control usart communication
through ATMEGA2560P/ATMEGA328P device. Each USARTn is controlled by a total of 6 registers stored through this structure.

```rust
   pub struct Usart {/* fields omitted */ }
```
### impl `new` for `Usart`

```rust
   pub unsafe fn new(num: UsartNum) -> &'static mut Usart {/* fields omitted */ }
```
Following contains the Usart as a Raw Pointer along with it's name.

```rust
   pub struct UsartObject {/* fields omitted */ }
```
_Various implementation functions for the USART protocol_

 ### Impl `disable` & `enable`for `UsartObject`

For interrupt driven USART operation, the Global Interrupt Flag should be cleared (and interrupts globally disabled) when doing the
initialization.

```rust
   fn disable(&mut self) {/* fields omitted */ }
        
   fn enable(&mut self) {/* fields omitted */ }
        
```


### Impl `set_clock` for `UsartObject`

Clock Generation is one of the initialization steps for the USART. If the USART is in Asynchronous mode or Master Synchronous mode then a internal clock generator is used while for Slave Synchronous mode we will use a external clock generator. Set the baud rate frequency for USART.Baud rate settings is used to set the clock for USART.

``` rust
   fn set_clock(&self,baud : i64,mode : UsartModes) {/* fields omitted */ }
```

### Impl `set_size` for `UsartObject`

The UCSZn2 in UCSRnB bits combined with the UCSZn1 bit in UCSRnC sets the number of data bits in a
frame the Receiver and Transmitter use. UCSZn2 is 1 for 9 bit data else 0 for 5,6,7,8 bit data.
Function set the limit of data to be handled by USART. 

``` rust
    fn set_size(&self,size : UsartDataSize) {/* fields omitted */ }
```

### Impl `set_parity` for `UsartObject`

Parity is optional i.e. can be odd, even or no parity bit.

``` rust
    fn set_parity(&self,parity : UsartParity) {/* fields omitted */ }
```
### Impl `set_stop` for `UsartObject`
Stop bit can be one bit or two bit.Function to set the number of stop bits in the USART.

 ``` rust
     fn set_stop(&self,stop : UsartStop) {/* fields omitted */ }

 ```
### Impl `set_frame` for `UsartObject`
 Set the frame format for USART. A serial frame is defined to be one character of data bits with synchronization bits (start and stop bits), and optionally a parity bit for error checking. The USART accepts all 30 combinations of the following as valid frame formats.
   -  1 start bit
   -  5, 6, 7, 8, or 9 data bits
   -  no, even or odd parity bit
   -  1 or 2 stop bits

``` rust 
     fn set_frame(&self,stop : UsartStop,size : UsartDataSize,parity : UsartParity) {
        self.set_size(size);
        self.set_parity(parity);
        self.set_stop(stop);
     }
```

### Impl `initialize` for `UsartObject`

Following is the cumulative function for initializing a particular USART and it will 
take all the necessary details about the mode in which the USART pin is to be used

``` rust
pub fn initialize
 (&mut self,mode : UsartModes,baud : i64,stop : UsartStop,size : UsartDataSize,parity : UsartParity)
        {/* fields omitted */ }

```

In case, You want to re-initialize the USART make sure that no transmission is on 
going during the peroid the registers are changed for USART, which can be done using
the TXCn(for the transmission) and the RXC (to check if the recieve buffer is
empty). 

**Note** that the TXCn Flag must be cleared before each transmission (before UDRn
is written) if it is used for this purpose. 

## Data Transmission
----

To Transmit data the you need to enable the USART Transmitter by
setting TXEN bit to 1 in the UCSRnB Register. When the Transmitter
is enabled the TxDn functions as the Transmitter's serial output. 

### Impl `transmit_enable` for `UsartObject`
``` rust
    pub unsafe fn transmit_enable(&mut self) {
        (*self.usart).ucsrb.update(|srb| {
            srb.set_bit(3, true);
        });
    }
```
### Impl `transmit_data` for `UsartObject`
For sending Frames with 5-8 Data bits Transmission is initiated by loading the
data to the transmit buffer,
For sending Frames with 9 Data Bits we have to store the 9th bit in the TXB8 in
UCSRnB, before the low byte character is written to the UDRn.

the code for transmitting data string, Integer and Floating point byte by byte.

``` rust
    pub fn transmit_data (&self,data: Volatile<u8>) {/* fields omitted */ }
```

### Impl `transmit_disable` for `UsartObject`
To disable the Transmitter, shift register and transmit buffer must not contain any data to be transmitted which is
checked by the TXCn and UDREn bit in UCSRnA register respectively.Once disabled, it will no longer override TxDn
pin.

``` rust 
    pub fn transmit_disable(&mut self) {/* fields omitted */}  
```

## Data Receiving
---
### Impl `recieve_enable` for `UsartObject`
To enable Data Receiver, write 1 to Receive Enable (RXENn) bit i.e BIT 4 in the
UCSRnB Register. This function enables the reciever function of microcontroller, whithout enabling it no communication is possible.
``` rust
    pub unsafe fn recieve_enable(&mut self) {
        (*self.usart).ucsrb.update(|ucsrb| {
            ucsrb.set_bit(4, true);
        });
    }
```

### Impl `recieve_data` for `UsartObject`
For Frames with 5-8 Data bits, the Data reception begins when a valid start bit is detected. For Frames 9 Data bits, the 9th bit is Read from the RXB8n bit in UCSRnB before reading the low bits from the
UDRn.
``` rust
    pub fn recieve_data(&mut self) -> Option<Volatile<u8>,Volatile<u32>> {/* fields omitted */    }    
```

### Impl `error_check` for `UsartObject`
It can be used to check frame error,Data OverRun and Parity errors & returns true if error occurs,else false.
``` rust  
    pub fn error_check(&mut self)->bool{/* fields omitted */  }
```

### Impl `parity_check` for `UsartObject`
This function can be used to check parity error, returns true if error occurs,else false.
```rust
   pub fn parity_check(&mut self)->bool{/* fields omitted */}
```

### Impl `recieve_disable` for `UsartObject`
This function disables the reciever function of microcontroller.
```rust
   pub unsafe fn recieve_disable(&mut self){/* fields omitted */}
```

### Impl `flush_recieve` for `UsartObject`
This function clears the unread data in the receive buffer by flushing it 
```rust
   pub unsafe fn flush_recieve (&self) {/* fields omitted */}
```

### Impl `read` for `UsartObject`
 This function is used to recieve data of one frame. But it only functions when already data is available for read which can be checked by available function. Either 5 to 8 bits and 9 bits of data can be recieved from this function. In case of 5 to 8 bits this function returns u8. In case of 9 bits it retuns u32 of which first 9 bits are data recieved and remaining bits are insignificant. In case ,if an frame error or parity error occurs, this function returns -1.
```rust
   pub fn read(&mut self) -> Option<Volatile<u8>,Volatile<u32>> {/* fields omitted */}
```

## `println()` Functions and `USAGE` of above functions
---

This file contains the println() functions in various versions which the
user will use to transmit data using USART on ATMEGA2560P/ATMEGA328P. This file combines 
all the functions in other USART source code to make useful functions.

### Impl `begin` for `UsartObject` 

Can be used to initialize with default settings.

```rust
    pub unsafe fn begin(&mut self) {
        self.disable();              // Disable global interrupts.
        self.transmit_enable();      // enabling the Transmitter
        self.recieve_enable();       // enabling receiver
        self.initialize(MODE, BAUD, STOP, SIZE, PARITY); // cumulative function for initialization
    }
```
Similarly we will define functoins to initialize `begin_set_baud` with given baud rate and remaining settings will be set to default.
`end` can be used to stop the functioning of USART.

```rust    
    pub unsafe fn begin_set_baud(&mut self, baud1: i64) {/* fields omitted */}
    pub unsafe fn end(&mut self) {/* fields omitted */}
```

### `println_string` function

Main println() function for using USART according to default used values.
Transmitter mode is first enabled for the default usart.
Then the function takes the usart and initializes it.
Then the string given by the user is transmitted through the USART.
This will be used to transmit string data.

```rust
pub fn println_string(data: &'static str) {
    unsafe {
        let mut u: UsartObject = UsartObject::new(NUM);
        u.disable();
        u.transmit_enable();
        u.initialize(MODE, BAUD, STOP, SIZE, PARITY);
        u.write_string(data);
        u.transmit_disable();
        u.reset();
        u.enable();
    }
}
```
similarly in `println_integer` replace **u.write_string(data)** with **u.write_integer(data)** and with **u.write_float(data, precision)** in `println_float`

```rust
pub fn println_integer(data: u32) {/* fields omitted */}
pub fn println_float(data: f64, precision: u32) {/* fields omitted */}
```