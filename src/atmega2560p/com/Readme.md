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
Stop bit. To understand UART in more Detail click [here](https://www.analog.com/en/analog-dialogue/articles/uart-a-hardware-communication-protocol.html#) to access online resources.

## Functioning of USART in atmega 2560p

### Initialization of USART

In atmega2560p we have four USART's which are USART0, USART1, USART2, and USART3. The
USART has to be initialized before any communication can take place. To initialize
we have to set Baud Rate, Set the Frame Format and enable Transmitter or Receiver
depending on the usage. For interrupt driven USART operation, the Global Interrupt
Flag should be cleared (and interrupts globally disabled) when doing the
initialization. 

In case, You want to re-initialize the USART make sure that no transmission is on 
going during the peroid the registers are changed for USART, which can be done using
the TXCn(for the transmission) and the RXC (to check if the recieve buffer is
empty). 

**Note** that the TXCn Flag must be cleared before each transmission (before UDRn
is written) if it is used for this purpose. 

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

Usart Transmitter has two Flags, **USART Data Register Empty(UDREn) and Tranfer
Complete (TXCn), to indicate its state. The **UDREn** Flag indicates the state of
Transfer Buffer, it is set when the Transfer Buffer is empty and is cleared
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
detection. To enable Data Receiver, write 1 to Receive Enable (RXENn) bit in the
UCSRnB Register. Then the RxDn pin functions as Receiver's serial input.

**Note**  Initialization should be done before any reception can take place.

For Frames with **5-8 Data bits**, the Data reception begins when a **VALID** start
bit is detected. Each bit following start bit will be sampled at baus rate and
shifted to Receive Shift Register until first stop bit is detected, the second stop
bit is Ignored. Once the first stop bit is recieved then the data of Shift Register 
is send to Recieve Buffer which Can be Read from UDRn. For Frames **9 Data bits**,
the 9th bit is Read from the RXB8n bit in UCSRnB before reading the low bits from the
UDRn.

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

Unlike Transmitter, Disabling Receiver is immediate. Data from ongoing Reception is
lost forever. 











## Code Overview

