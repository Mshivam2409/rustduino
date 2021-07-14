# Introduction 

UART, or universal asynchronous receiver-transmitter, is one of the most used
device-to-device communication protocols.Embedded systems, microcontrollers, and
computers mostly use UART as a form of device-to-device hardware communication
protocol.
By definition, UART is a hardware communication protocol that uses asynchronous
serial communication with configurable speed and Data Format.

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

To Transmit data the you need to enable the USART Transmitter, which is enabled by setting the Transmit Enable(TXEN) bit in the UCSRnB Register 




## Code Overview







## 