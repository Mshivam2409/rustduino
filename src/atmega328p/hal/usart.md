# Introduction

**USART**, or universal synchronous and asynchronous receiver-transmitter, is one of the most used
device-to-device communication protocols.Embedded systems, microcontrollers, and
computers mostly use **USART** as a form of device-to-device hardware communication
protocol.
By definition, **USART** is a hardware communication protocol that uses synchronous and asynchronous
serial communication with configurable speed and Data Format.

The **USART** (Universal Synchronous and Asynchronous Serial Receiver and Transmitter) hardware in the ATmega328P microcontroller uses registers *UDRn*, *UCSRnA*, *UCSRnB*, *UCSRnC*, *UBRRnL*, and *UBRRnH* to configure the hardware and to transmit and receive data. 
Given below are the register descriptions. The variable ‘n’ = 0 for USART0 ‘n’ = 1 for USART1 etc. The ATmega328p has only USART0.


