# Introduction 

The Universal Synchronous and Asynchronous serial Receiver and Transmitter (USART) is a highly flexible serial
communication device. Embedded systems, microcontrollers, and computers mostly use USART as a form of device-to-device hardware communication protocol. By definition, USART is a hardware communication protocol that uses synchronous or asynchronous
serial communication with configurable speed and Data Format. Asynchronous communication does not use a clock to validate data.

## Introduction to Serial and Asynchronous Communication

In Serial Communication, the data which is recieved in bytes is transmitted 
bit by bit in a sequenced manner, through the Bus or communication channel, and on
reaching the destination UART it is re assembled in bytes.

UART uses asynchronous communication for recieving data as it is not validated by a 
clock, or in other words, data is transmitted intermittently rather than in a steady
stream. In asynchronous transmission, data is sent one byte at a time and each byte is
preceded by start bit and stop bit.

To ensure the correct transmission the transmission and the reciever pins need to 
have same settings like Transmission Speed, Data length in bits, how is the Start and
Stop bit. To understand UART in more Detail click [here](https://www.analog.com/en/analog-dialogue/articles/uart-a-hardware-communication-protocol.html#) to access online resources. Read the Atmel manual [here](https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf) .

## Functioning of USART in atmega 2560p

### Initialization of USART

In atmega2560p we have four USART's which are USART0, USART1, USART2, and USART3. The
USART has to be initialized before any communication can take place. To initialize
we have to set Baud Rate, Set the Frame Format and enable Transmitter or Receiver
depending on the usage. For interrupt driven USART operation, the Global Interrupt
Flag should be cleared (and interrupts globally disabled) when doing the
initialization. 

```rust
 /// Function to disable global interrupts for smooth non-interrupted functioning of USART.
    fn disable(&mut self) {
        unsafe {
            // Disable global interrupts.
            interrupts::GlobalInterrupts::disable(&mut interrupts::GlobalInterrupts::new());
        }
    }
/// Function to re-enable global interrupts.
    fn enable(&mut self) {
        unsafe {
            // Enable global interrupts.
            interrupts::GlobalInterrupts::enable(&mut interrupts::GlobalInterrupts::new());
        }
    }
```

In case, You want to re-initialize the USART make sure that no transmission is on 
going during the peroid the registers are changed for USART, which can be done using
the TXCn(for the transmission) and the RXC (to check if the recieve buffer is
empty). 

**Note** that the TXCn Flag must be cleared before each transmission (before UDRn
is written) if it is used for this purpose. 

Clock Generation is one of the initialization steps for the USART. If the USART is in Asynchronous mode or Master Synchronous mode then a internal clock generator is used while for Slave Synchronous mode we will use a external clock generator. Set the baud rate frequency for USART.Baud rate settings is used to set the clock for USART.

``` rust
fn set_clock(&self,baud : i64,mode : UsartModes) {
        match mode {
            UsartModes::norm_async => { 
                let mut ubrr : u32 = ((f_osc*multiply)/(16*baud))-1; 
            },
            UsartModes::dou_async => {
                let mut ubrr : u32 = ((f_osc*multiply)/(8*baud))-1;
            },
            UsartModes::master_sync => {
                let mut ubrr : u32 = ((f_osc*multiply)/(2*baud))-1;
            },
            _ => unreachable!(),
        }
        self.ubrrl.set_bits(0..8,ubrr);
        self.ubrrh.set_bits(0..4,(ubrr>>8));
    }
```

The UCSZn2 in UCSRnB bits combined with the UCSZn1 bit in UCSRnC sets the number of data bits in a
frame the Receiver and Transmitter use. UCSZn2 is 1 for 9 bit data else 0 for 5,6,7,8 bit data. See sction 22.10.3 and 22.10.4 in Atmel manual.

 | UCSZn2 | UCSZn1 | UCSZn0 |  Character Size |
 |--------|--------|--------|-----------------|
 |  0     |   0    |   0    |  5-bit          |
 |  0     |   0    |   1    |  6-bit          |
 |  0     |   1    |   0    |  7-bit          |
 |  0     |   1    |   1    |  8-bit          |
 |  1     |   1    |   1    |  9-bit          |

<details>
  <summary>Click to expand code</summary>

``` rust
fn set_size(&self,size : UsartDataSize) {
        match size {
            UsartDataSize::five
            | UsartDataSize::six
            | UsartDataSize::seven
            | UsartDataSize::eight => { 
                self.ucsrb.update( |srb| {
                    srb.set_bit(2,false);
                });       
            },
            UsartDataSize::nine => { 
                self.ucsrb.update( |srb| {
                    srb.set_bit(2,true);
                });
            },
        }
        match size {
            UsartDataSize::five
            | UsartDataSize::six => { 
                self.ucsrc.update( |src| {
                    src.set_bit(2,false);
                });       
            },
            UsartDataSize::nine
            | UsartDataSize::seven
            | UsartDataSize::eight => { 
                self.ucsrc.update( |src| {
                    src.set_bit(2,true);
                });
            },
        }
        match size {
            UsartDataSize::five
            | UsartDataSize::seven => { 
                self.ucsrc.update( |src| {
                    src.set_bit(1,false);
                });       
            },
            UsartDataSize::nine
            | UsartDataSize::six
            | UsartDataSize::eight => { 
                self.ucsrc.update( |src| {
                    src.set_bit(1,true);
                });
            },
        }
    }
```
</details>

Parity is optional i.e. can be odd, even or no parity bit. Check section 22.10.4 in atmael manual.

``` rust
 fn set_parity(&self,parity : UsartParity) {
        match parity {
            UsartParity::no => { 
                self.ucsrc.update( |src| {
                    src.set_bit(4,false);
                    src.set_bit(5,false);
                });
            },
            UsartParity::even => { 
                self.ucsrc.update( |src| {
                    src.set_bit(4,false);
                    src.set_bit(5,true);
                });
            },
            UsartParity::odd => { 
                self.ucsrc.update( |src| {
                    src.set_bit(4,true);
                    src.set_bit(5,true);
                });
            },
        }
    }
```
Stop bit can be one bit or two bit.

 ``` rust
  /// Function to set the number of stop bits in the USART.
    fn set_stop(&self,stop : UsartStop) {
        match stop {
            UsartStop::one => { 
                self.ucsrc.update( |src| {
                    src.set_bit(3,false);
                });
            },
            UsartStop::two => { 
                self.ucsrc.update( |src| {
                    src.set_bit(3,true);
                });
            },
        }
    }
```
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
Following is the cumulative function for initializing a particular USART and it will take all the necessary details about the mode in which the USART pin is to be used.

```rust
    pub fn initialize(&mut self,mode : UsartModes,baud : i64,stop : UsartStop,size : UsartDataSize,parity : UsartParity) {
        // Check that recieve and transmit buffers are completely cleared
        // and no transmission or recieve of data is already in process.
        self.enable();                                             //  Enable Global interrupts.
        self.check();
        
        self.disable();                                            //  Disable Global interrupts.
        let num : UsartNum = self.get_num();
        
        self.set_power(num);                                       //  Set Power reduction register.
        
        self.mode_select(mode);                                    //  Set the USART at the given mode.
        
        //  Set the clock for USART according to user input.
        if( mode == UsartModes::slave_sync )  { }
        else { self.set_clock(baud,mode) }                         
        
        //  Set the frame format according to input.
        self.set_frame(stop,size,parity);                                     

        self.enable();                                             //  Enable Global interrupts.
    }
```


### Data Transmission

To Transmit data the you need to enable the USART Transmitter, which is enabled by
setting the Transmit Enable(TXEN) bit in the UCSRnB Register. When the Transmitter
is enabled the TxDn functions as the Transmitter's serial output. Initialization
should be done before transmission.

For sending Frames with **5-8 Data bits** Transmission is initiated by loading the
data to the transmit buffer,which can be done by writing to the UDRn I/O location.
Buffered Data will be moved to Shift Register if the shift Register is in Idle state
or just after the stop bit of an transmitting frame. Upon filling the shift Register
it transmits one frame at the Baud rate, U2X bit.
For sending Frames with **9 Data Bits** we have to store the 9th bit in the TXB8 in
UCSRnB, before the low byte character is written to the UDRn.

Usart Transmitter has two Flags, **USART Data Register Empty**(UDREn) and 
**Tranfer Complete** (TXCn), to indicate its state. The **UDREn** Flag indicates the
state of Transfer Buffer, it is set when the Transfer Buffer is empty and is cleared
when the transfer buffer has data that has not yet been transffered to shift register.
The **TXCn** is Flag bit is set 1 when entire frame in the transmit shift register
has been shifted out and no new data is present in the transmit buffer it can
automatically br cleared if a interrupt is set up or by writing 1 to its bit.

Parity generator calculates the parity for a serial data Frame and if parity bit is
set 1 transmitter control logic inserts parity in serial frame.To disable the
Transmitter, shift and buffer must not contain any data to be transmitted.Once
disabled, it will no longer override TxDn pin.

### Data Receiving

USART Receiver function similar to Transmitter except for some features like error
detection. To enable Data Receiver, write 1 to Receive Enable (RXENn) bit i.e BIT 4 in the
UCSRnB Register. Then the RxDn pin functions as Receiver's serial input.

``` rust
impl Usart{
   ///This function enables the reciever function of microcontroller, whithout enabling it no communication is possible.
   pub fn recieve_enable(&mut self){
    unsafe {
        self.ucsrb.update(|ucsrb| {
            ucsrb.set_bit(4, true);
        });
    }
   }
```

**Note**  Initialization should be done before any reception can take place.

For Frames with **5-8 Data bits**, the Data reception begins when a **VALID** start
bit is detected. Each bit following start bit will be sampled at baud rate and
shifted to Receive Shift Register until first stop bit is detected, the second stop
bit is Ignored. Once the first stop bit is recieved then the data of Shift Register 
is send to Recieve Buffer which Can be Read from UDRn. For Frames **9 Data bits**,
the 9th bit is Read from the RXB8n bit in UCSRnB before reading the low bits from the
UDRn.
``` rust
   pub fn recieve_data(&mut self)->Option<Volatile<u8>,Volatile<u32>>{
       self.recieve_enable();
    unsafe{
        let  ucsrc=read_volatile(&self.ucsrc);
        let  ucsrb=read_volatile(&self.ucsrb);
        if ucsrc.gets_bits(1..3)==0b11 && ucsrb.get_bit(2){

            while !(self.available()){};
            let ucsra=read_volatile(&self.ucsra);
            let ucsrb=read_volatile(&self.ucsrb);
            let mut udr=read_volatile(&mut self.udr);
            if ucsra.get_bits(2..5)!=0b000{
                Some(-1)
            }
            else{
                let rxb8=ucsrb.get_bits(1..2);
                udr=udr.set_bits(8..9,rxb8);
                Some(udr);
            }
        }
        else{
            while !(self.available()){
                let ucsra=read_volatile(&self.ucsra);
            };
            let ucsra=read_volatile(&self.ucsra);
            let mut udr=read_volatile(&mut self.udr);
            if ucsra.get_bits(2..5)!=0b000{
                Some(-1)
            }
            else{
                Some(udr);
            }
            
        }
    }
    self.recieve_disable();
 } 
 ```
The Receiver has one flag to indicate its state, which is Receive complete(RXCn), it
is one if unread data exist in Receive buffer and zero if no unread data in Receive
buffer. If Receiver is disabled so the Receive buffer is flushed and this flag
becomes 0. 
The USART Receiver has three error flags which are Frame Error(FEn), Data OverRun
(DORn) and Parity Error(UPEn). The **Frame Error** indicates the state of the first
stop bit in the next readable Frame in ppresent in the receive buffer, it is one when
stop bit read is zero and it is zero when stop bit read is 1. The **Data OverRun**
occurs due to receive buffer full condition. If the DORn Flag is set there was one or
more serial frame lost between the frame last read from UDRn, and the next frame read
from UDRn. The DORn Flag is cleared when the frame received was successfully moved
from the Shift Register to the receive buffer.The **Parity Error** (UPEn) bit is set
if next character to be read from the recieve buffer had parity error and Parity
checking is enabled at that point (UPMn1 = 1). This bit is valid until the receive
buffer (UDRn) is read.

<details>
  <summary>Click to expand code</summary>

``` rust  
 pub fn error_check(&mut self)->bool{
      unsafe{
      let ucsra=read_volatile(&self.ucsra);
      if ucsra.get_bits(2..4)!=0b00{
          true
      }
      else{
          false
      }
    }
  }
  ///This function can be used to check parity error.
  ///It returns true if error occurs,else false.
  pub fn parity_check(&mut self)->bool{
      unsafe{
    let ucsra=read_volatile(&self.ucsra);
    if ucsra.get_bit(4)==0b1{
        true
    } 
    else{
        false
    }
   }
  }
 ///This function disables the reciever function of microcontroller.
  pub fn recieve_disable(&mut self){
    unsafe {
        self.ucsrb.update(|ucsrb| {
            ucsrb.set_bit(4, false);
        });
    }
   }
   ///This function checks if the data is avialable for readig or not.
   pub fn available(&mut self)->bool{
    let ucsra=read_volatile(&self.ucsra);
    if ucsra.get_bit(7){
        true
    }
    else{
        false
    }
   }
   ///This function clears the unread data in the receive buffer by flushing it 
   pub fn flush (){
    unsafe {
        self.ucsra.update(|ucsra| {
            ucsra.set_bit(7, false);
        });
    }
   }
```
</details>

Unlike Transmitter, Disabling Receiver is immediate. Data from ongoing Reception is
lost forever, and disabled receiver will no longer override the RxD pin.
In case ,if an frame error or parity error occurs, this function returns -1.

<details>
  <summary>Click to expand code</summary>

``` rust
   pub fn read(&mut self)->Option<Volatile<u8>,Volatile<u32>>{

       unsafe{
        let  ucsrc=read_volatile(&self.ucsrc);
        let  ucsrb=read_volatile(&self.ucsrb);
        if ucsrc.gets_bits(1..3)==0b11 && ucsrb.get_bit(2){

         let ucsra=read_volatile(&self.ucsra);
         let ucsrb=read_volatile(&self.ucsrb);
         let mut udr=read_volatile(&mut self.udr);
           if ucsra.get_bits(2..5)!=0b000{
            Some(-1)
           }
           else{
            let rxb8=ucsrb.get_bits(1..2);
            udr=udr.set_bits(8..9,rxb8);
            Some(udr);
          }
        }
        else {
         let ucsra=read_volatile(&self.ucsra);
         let udr=read_volatile(&mut self.udr);
          if ucsra.get_bits(2..5)!=0b000{
            Some(-1)
          }
          else{
            Some(udr);
          }  
        }
       }
    }
}
```
</details>

### Asynchronous Data Reception

USART includes a clock and Data recovery unit for handling the asynchronous data
reception. The **clock recovery** logic synchronizes internal clock to the incoming
serial frames. It detects the first high to low transition and check if the signal was
a valid signal or just a noise spike. In **Data recovery** unit, the decision of the
logic level of the received bit is taken by doing a majority voting of the logic
value to the three samples in the center of the received bit. A majority voting
process acts as a low pass filter for the incoming signal on the RxDn pin.
The recovery process is then repeated until a complete frame is received. Including
the first stop bit. A new high to low transition indicating the start bit of a new
frame can come right after the last of the bits used for majority voting. 

Operational Range is the maximum and minimum mismatch in the Baud Rate which Receiver
can tolerate. There are two sources of Errors one is due to clock synchronization and
other is due to Baud Rate Generator error in calculation in the baud rate, however
latter can be minimized by using a low error baud rate in UBRR.

The more detailed information for the USART can be found in chapter 22-23 in the
Manual for atmega 2560p.