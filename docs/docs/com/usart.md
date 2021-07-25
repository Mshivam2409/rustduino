---
id: usart
slug: /usart
title: USART
---


## USART in Atmega328p
---

### Serial
---
This struct contains USART0 in atmega328p. Each USART can be accesed through Serial.usart[n], where 0<= n <=3.
```rust
   pub struct Serial { pub usart: [&'static mut Usart; 1],}
```
#### impl `new` for `Serial`
```rust
   pub fn new() -> Serial {/* fields omitted */ }
```


### Initialization of USART
---

This structure contains various registers needed to control usart communication
through ATMEGA328P device. USART0 is controlled by a total of 6 registers stored through this structure.

```rust
   pub struct Usart {/* fields omitted */ }
```
#### impl `new` for `Usart`
```rust
   pub unsafe fn new(num: UsartNum) -> &'static mut Usart {/* fields omitted */ }
```
Following contains the Usart as a Raw Pointer along with it's name.

```rust
   pub struct UsartObject {/* fields omitted */ }
```
*Various implementation functions for the USART protocol*

 #### Impl `disable` & `enable`for `UsartObject`

For interrupt driven USART operation, the Global Interrupt Flag should be cleared (and interrupts globally disabled) when doing the
initialization.

```rust
   fn disable(&mut self) {
        unsafe {interrupt::Interrupt::disable(&mut interrupt::Interrupt::new());}
    }
   fn enable(&mut self) {
        unsafe {interrupt::Interrupt::enable(&mut interrupt::GlobalInterrupt::new());}
    }
```

#### Impl `set_clock` for `UsartObject`

Clock Generation is one of the initialization steps for the USART. If the USART is in Asynchronous mode or Master Synchronous mode then a internal clock generator is used while for Slave Synchronous mode we will use a external clock generator. Set the baud rate frequency for USART.Baud rate settings is used to set the clock for USART.

``` rust
   fn set_clock(&mut self,baud : i64,mode : UsartModes) {/* fields omitted */ }
```

#### Impl `set_size` for `UsartObject`

The UCSZn2 in UCSRnB bits combined with the UCSZn1 bit in UCSRnC sets the number of data bits in a
frame the Receiver and Transmitter use. UCSZn2 is 1 for 9 bit data else 0 for 5,6,7,8 bit data.
Function set the limit of data to be handled by USART.
``` rust
    fn set_size(&mut self,size : UsartDataSize) {/* fields omitted */ }
```
#### Impl `set_parity` for `UsartObject`

Parity is optional i.e. can be odd, even or no parity bit.

``` rust
    fn set_parity(&mut self,parity : UsartParity) {/* fields omitted */ }
```
#### Impl `set_stop` for `UsartObject`
Stop bit can be one bit or two bit.Function to set the number of stop bits in the USART.

 ``` rust
     fn set_stop(&mut self,stop : UsartStop) {/* fields omitted */ }

```
#### Impl `set_frame` for `UsartObject`
 Set the frame format for USART. A serial frame is defined to be one character of data bits with synchronization bits (start and stop bits), and optionally a parity bit for error checking. The USART accepts all 30 combinations of the following as valid frame formats.
   -  1 start bit
   -  5, 6, 7, 8, or 9 data bits
   -  no, even or odd parity bit
   -  1 or 2 stop bits
 
 ``` rust 
     fn set_frame(&mut self,stop : UsartStop,size : UsartDataSize,parity : UsartParity) {
        self.set_size(size);
        self.set_parity(parity);
        self.set_stop(stop);
     }
```