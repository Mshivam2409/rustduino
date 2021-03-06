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
fn set_clock(&mut self, baud: i64, mode: UsartModes) {
        let ubrr: u32;
        match mode {
            UsartModes::Normasync => {
                ubrr = (((F_OSC * MULTIPLY) / (16.00 * baud as f64)) - 1.00) as u32;
            }
            UsartModes::Douasync => {
                ubrr = (((F_OSC * MULTIPLY) / (8.00 * baud as f64)) - 1.00) as u32;
            }
            UsartModes::Mastersync => {
                ubrr = (((F_OSC * MULTIPLY) / (2.00 * baud as f64)) - 1.00) as u32;
            }
            _ => unreachable!(),
        }
        self.ubrrl.update(|ubrrl| {
            for i in 0..8 {
                ubrrl.set_bit(i, ubrr.get_bit(i));
            }
        });
        self.ubrrh.update(|ubrrh| {
            for i in 0..4 {
                ubrrh.set_bit(i, ubrr.get_bit(i + 8));
            }
        });
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
fn set_size(&mut self, size: UsartDataSize) {
        match size {
            UsartDataSize::Five
            | UsartDataSize::Six
            | UsartDataSize::Seven
            | UsartDataSize::Eight => {
                self.ucsrb.update(|srb| {
                    srb.set_bit(2, false);
                });
            }
            UsartDataSize::Nine => {
                self.ucsrb.update(|srb| {
                    srb.set_bit(2, true);
                });
            }
        }
        match size {
            UsartDataSize::Five | UsartDataSize::Six => {
                self.ucsrc.update(|src| {
                    src.set_bit(2, false);
                });
            }
            UsartDataSize::Nine | UsartDataSize::Seven | UsartDataSize::Eight => {
                self.ucsrc.update(|src| {
                    src.set_bit(2, true);
                });
            }
        }
        match size {
            UsartDataSize::Five | UsartDataSize::Seven => {
                self.ucsrc.update(|src| {
                    src.set_bit(1, false);
                });
            }
            UsartDataSize::Nine | UsartDataSize::Six | UsartDataSize::Eight => {
                self.ucsrc.update(|src| {
                    src.set_bit(1, true);
                });
            }
        }
    }
```
</details>

Parity is optional i.e. can be odd, even or no parity bit. Check section 22.10.4 in atmael manual.

``` rust
 fn set_parity(&mut self, parity: UsartParity) {
        match parity {
            UsartParity::No => {
                self.ucsrc.update(|src| {
                    src.set_bit(4, false);
                    src.set_bit(5, false);
                });
            }
            UsartParity::Even => {
                self.ucsrc.update(|src| {
                    src.set_bit(4, false);
                    src.set_bit(5, true);
                });
            }
            UsartParity::Odd => {
                self.ucsrc.update(|src| {
                    src.set_bit(4, true);
                    src.set_bit(5, true);
                });
            }
        }
    }
```
Stop bit can be one bit or two bit.

 ``` rust
  /// Function to set the number of stop bits in the USART.
    fn set_stop(&mut self, stop: UsartStop) {
        match stop {
            UsartStop::One => {
                self.ucsrc.update(|src| {
                    src.set_bit(3, false);
                });
            }
            UsartStop::Two => {
                self.ucsrc.update(|src| {
                    src.set_bit(3, true);
                });
            }
        }
    }
```
 Set the frame format for USART. A serial frame is defined to be one character of data bits with synchronization bits (start and stop bits), and optionally a parity bit for error checking. The USART accepts all 30 combinations of the following as valid frame formats.
   -  1 start bit
   -  5, 6, 7, 8, or 9 data bits
   -  no, even or odd parity bit
   -  1 or 2 stop bits
 
 ``` rust 
    fn set_frame(&mut self, stop: UsartStop, size: UsartDataSize, parity: UsartParity) {
        self.set_size(size);
        self.set_parity(parity);
        self.set_stop(stop);
    }
```
Following is the cumulative function for initializing a particular USART and it will take all the necessary details about the mode in which the USART pin is to be used.

```rust
    pub fn initialize(
        &mut self,
        mode: UsartModes,
        baud: i64,
        stop: UsartStop,
        size: UsartDataSize,
        parity: UsartParity,
    ) {
        // Check that recieve and transmit buffers are completely cleared
        // and no transmission or recieve of data is already in process.
        let mut i: i32 = 10;
        while self.check_ongoing() == false {
            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!()
            }
        }

        self.disable(); //  Disable Global interrupts.
        let num: UsartNum = self.get_num();

        self.set_power(num); //  Set Power reduction register.

        self.mode_select(mode); //  Set the USART at the given mode.

        //  Set the clock for USART according to user input.
        match mode {
            UsartModes::Slavesync => {}
            _ => {
                self.set_clock(baud, mode);
            }
        }

        //  Set the frame format according to input.
        self.set_frame(stop, size, parity);

        self.enable(); //  Enable Global interrupts.
    }
```


### Data Transmission

To Transmit data the you need to enable the USART Transmitter, which is enabled by
setting the Transmit Enable(TXEN) bit to 1 in the UCSRnB Register. When the Transmitter
is enabled the TxDn functions as the Transmitter's serial output. Initialization
should be done before transmission.

``` rust
    pub unsafe fn transmit_enable(&mut self) {
        (*self.usart).ucsrb.update(|srb| {
            srb.set_bit(3, true);
        });
    }

```

For sending Frames with **5-8 Data bits** Transmission is initiated by loading the
data to the transmit buffer,which can be done by writing to the UDRn I/O location.
Buffered Data will be moved to Shift Register if the shift Register is in Idle state
or just after the stop bit of an transmitting frame. Upon filling the shift Register
it transmits one frame at the Baud rate, U2X bit.
For sending Frames with **9 Data Bits** we have to store the 9th bit in the TXB8 in
UCSRnB, before the low byte character is written to the UDRn.

Following is the code for transmitting data string byte by byte.
``` rust
    pub fn transmit_data(&mut self, data: u8) {
        let mut ucsra = unsafe { (*self.usart).ucsra.read() };
        let mut udre = ucsra.get_bit(5);

        let mut i: i32 = 100;
        while udre == false {
            ucsra = unsafe { (*self.usart).ucsra.read() };
            udre = ucsra.get_bit(5);

            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!();
            }
        }

        unsafe { (*self.usart).udr.write(data) };
    }

    pub fn write_string(&mut self, data: &'static str) {
        let mut vec: FixedSliceVec<u8> = FixedSliceVec::new(&mut []);

        for c in data.chars() {
            vec.push(c as u8);
        }

        for i in 0..(vec.len()) {
            self.transmit_data(vec[i]);
        }
    }

```

Usart Transmitter has two Flags, **USART Data Register Empty**(UDREn) and 
**Tranfer Complete** (TXCn), to indicate its state. The **UDREn** Flag indicates the
state of Transfer Buffer, it is set when the Transfer Buffer is empty and is cleared
when the transfer buffer has data that has not yet been transffered to shift register.
The **TXCn** is Flag bit is set 1 when entire frame in the transmit shift register
has been shifted out and no new data is present in the transmit buffer it can
automatically br cleared if a interrupt is set up or by writing 1 to its bit.

Parity generator calculates the parity for a serial data Frame and if parity bit(UPMn1 bit) is
set 1 transmitter control logic automatically inserts parity in serial frame.

To disable the Transmitter, shift register and transmit buffer must not contain any data to be transmitted which is
checked by the TXCn and UDREn bit in UCSRnA register respectively.Once disabled, it will no longer override TxDn
pin.

``` rust 

    pub fn transmit_disable(&mut self) {
        let ucsra = unsafe { (*self.usart).ucsra.read() };
        let mut uscra6 = ucsra.get_bit(6);
        let mut uscra5 = ucsra.get_bit(5);
        let mut i: i32 = 100;

        while uscra6 == false || uscra5 == false {
            uscra6 = ucsra.get_bit(6);
            uscra5 = ucsra.get_bit(5);
            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!()
            }
        }

        unsafe {
            (*self.usart).ucsrb.update(|srb| {
                srb.set_bit(3, false);
            });
        }
    }
```

### Data Receiving

USART Receiver function similar to Transmitter except for some features like error
detection. To enable Data Receiver, write 1 to Receive Enable (RXENn) bit i.e BIT 4 in the
UCSRnB Register. Then the RxDn pin functions as Receiver's serial input.

``` rust
/// This function enables the reciever function of microcontroller, whithout enabling it no communication is possible.
    pub fn recieve_enable(&mut self) {
        self.ucsrb.update(|ucsrb| {
            ucsrb.set_bit(4, true);
        });
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
   pub fn recieve_data(&mut self) -> Option<u32> {
        let ucsrc = self.ucsrc.read();
        let ucsrb = self.ucsrb.read();

        let mut i: i32 = 10;
        while self.available() == false {
            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!()
            }
        }

        //  Case when there is 9 bits mode.
        if ucsrc.get_bits(1..3) == 0b11 && ucsrb.get_bit(2) == true {
            let ucsra = self.ucsra.read();
            let mut udr: u32 = self.udr.read() as u32;
            if ucsra.get_bits(2..5) != 0b000 {
                None
            } else {
                let rxb8: u32 = ucsrb.get_bits(1..2) as u32;
                udr.set_bits(8..9, rxb8);
                Some(udr)
            }
        }
        //  Case when there is a case of 5 to 8 bits.
        else {
            let ucsra = self.ucsra.read();
            let udr: u32 = self.udr.read() as u32;
            if ucsra.get_bits(2..5) != 0b000 {
                None
            } else {
                Some(udr)
            }
        }
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
 pub fn error_check(&mut self) -> bool {
        let ucsra = self.ucsra.read();
        if ucsra.get_bits(3..5) != 0b00 {
            true
        } else {
            false
        }
    }

    /// This function can be used to check parity error.
    /// It returns true if error occurs else false.
    pub fn parity_check(&mut self) -> bool {
        let ucsra = self.ucsra.read();
        if ucsra.get_bit(2) == true {
            true
        } else {
            false
        }
    }

    /// This function disables the reciever function of microcontroller.
    pub fn recieve_disable(&mut self) {
        self.ucsrb.update(|ucsrb| {
            ucsrb.set_bit(4, false);
        });
    }

    /// This function clears the unread data in the receive buffer by flushing it
    pub fn flush_recieve(&mut self) {
        let mut _udr = self.udr.read();
        let mut ucsra = self.ucsra.read();
        let mut i: i32 = 100;
        while ucsra.get_bit(7) == true {
            ucsra = self.ucsra.read();
            _udr = self.udr.read();
            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!()
            }
        }

        self.ucsra.update(|ucsra| {
            ucsra.set_bit(7, false);
        });
    }
```
</details>

Unlike Transmitter, Disabling Receiver is immediate. Data from ongoing Reception is
lost forever, and disabled receiver will no longer override the RxD pin.
In case ,if an frame error or parity error occurs, this function returns -1.

<details>
  <summary>Click to expand code</summary>

``` rust
   pub fn read(&mut self) -> Option<u32> {
        let ucsrc = self.ucsrc.read();
        let ucsrb = self.ucsrb.read();

        let mut i: i32 = 10;
        while self.available() == false {
            if i != 0 {
                delay_ms(1000);
                i = i - 1;
            } else {
                unreachable!()
            }
        }

        if ucsrc.get_bits(1..3) == 0b11 && ucsrb.get_bit(2) == true {
            let ucsra = self.ucsra.read();
            let ucsrb = self.ucsrb.read();
            let mut udr: u32 = self.udr.read() as u32;
            if ucsra.get_bits(2..5) != 0b000 {
                None
            } else {
                let rxb8: u32 = ucsrb.get_bits(1..2) as u32;
                udr.set_bits(8..9, rxb8);
                Some(udr)
            }
        } else {
            let ucsra = self.ucsra.read();
            let udr: u32 = self.udr.read() as u32;
            if ucsra.get_bits(2..5) != 0b000 {
                None
            } else {
                Some(udr)
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


# Introduction 