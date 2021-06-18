---
id: data-protocols
slug: /data-protocols
title: Data Protocols
---

The proper descriptions of digital message formats as well as rules are known communication protocols. The implementation of these protocols can be done within hardware as well as software. So communications protocols are available around thousand types which are used all over in analog & digital communications.

- **Protocol: ** A set of rules and regulations is called a protocol.
- **Communication: ** Exchange of information from one system to another system with a medium is called communication.
- **Communication Protocol: ** A set of rules and regulations that allow two electronic devices to connect to exchange the data with one and another.

## Types of Communication Protocols

There are two types of communication protocols which are classified below:

- Inter System Protocol
- Intra System Protocol

### Inter System Protocol

The inter-system protocol using to communicate the two different devices. Like communication between computer to microcontroller kit. The communication is done through an inter bus system.

The different categories of intersystem protocol mainly include the following.

- UART Protocol
- USART Protocol
- USB Protocol

#### UART Protocol

**UART**, for **Universal Asynchronous Receiver Transmitter**, is one of the most used serial protocols. It's almost as old as I am, and very simple. Most controllers have a hardware UART on board. It uses a single data line for transmitting and one for receiving data. Most often 8-bit data is transferred, as follows: 1 start bit (low level), 8 data bits and 1 stop bit (high level). The low level start bit and high level stop bit mean that there's always a high to low transition to start the communication. That's what describes UART. No voltage level, so you can have it at 3.3 V or 5 V, whichever your microcontroller uses. Note that the microcontrollers which want to communicate via UART have to agree on the transmission speed, the bit-rate, as they only have the start bits falling edge to synchronize. That's called asynchronous communication.

The universal asynchronous receiver-transmitter (UART) takes bytes of data and transmits the individual bits in a sequential fashion. At the destination, a second UART re-assembles the bits into complete bytes. Each UART contains a shift register , which is the fundamental method of conversion between serial and parallel forms. Serial transmission of digital information (bits) through a single wire or other medium is less costly than parallel transmission through multiple wires. The UART usually does not directly generate or receive the external signals used between different items of equipment. Separate interface devices are used to convert the logic level signals of the UART to and from the external signalling levels, which may be standardised voltage levels, current levels, or other signals.  

The timing dependency is one of the big drawbacks of UART, and the solution is USART, for Universal Synchronous/Asynchronous Receiver Transmitter. This can do UART, but also a synchronous protocol. In synchronous there's not only data, but also a clock transmitted. With each bit a clock pulse tells the receiver it should latch that bit.

 A **UART** usually contains the following components:

- a clock generator, usually a multiple of the bit rate to allow sampling in the middle of a bit period
- input and output shift registers
- transmit/receive control
- read/write control logic
- Autobaud measurement
- transmit/receive buffers 
- system data bus buffer 
- First-in, first-out (FIFO) buffer memory 
- Signals needed by a third party DMA controller 
- Integrated bus mastering DMA controller 

For long-distance communication, the 5V UART is not very reliable, that's why it's converted to a higher voltage, typically +12 V for a "0" and -12 V for a "1". The data format remains the same.

#### USART Protocol

USART stands for a "universal synchronous and asynchronous transmitter and receiver". It is a serial communication of a two-wire protocol. The data cable signal lines are labelled as Rx and TX. This protocol is used to transmitting and receiving the data byte by byte along with the clock pulses. It is a full-duplex protocol that means *transmitting and receiving data simultaneously* to different board rates. Different devices communicate with microcontroller to this protocol.

#### USB Protocol

USB stands for universal serial bus. Again it is a serial communication of two-wire protocol. The data cable signal lines are labelled D+ and D-. This protocol is used to communicate with the system peripherals. USB protocol is used to send and receive the data serially to the host and peripheral devices. USB communication requires driver software that is based on the functionality of the system. USB devices can transfer data on the bus without any request on the host computer. Now a day’s most devices are using this technique for communicating with USB protocol. Like a computer to communicate with an ARM controller using USB. 

### Intra System Protocol

The Intra system protocol is used to communicate the two devices within the circuit board. While using these intra system protocols, without going to intrasystem protocols we will expand the peripherals of the microcontroller. The circuit complexity and power consumption will be increased by using the intrasystem protocol. Using intra system protocols circuit complexity and power consumption, the cost is decreased and it is very secure to accessing the data.

The different categories of intrasystem protocol mainly include the following.

- I2C Protocol
- SPI Protocol
- CAN Protocol

#### I2C Protocol

**I2C** stands for **Inter-Integrated Circuit.** It is a bus interface connection protocol incorporated into devices for serial communication. It was originally designed by Philips Semiconductor in 1982. Recently, it is a widely used protocol for short-distance communication. It is also known as Two Wired Interface(TWI).

**Working of I2C Communication Protocol :**

It uses only 2 bi-directional open-drain lines for data communication called SDA and SCL. Both these lines are pulled high.

**Serial Data (SDA) –** Transfer of data takes place through this pin.

**Serial Clock (SCL) –** It carries the clock signal. 

I2C operates in 2 modes –

- Master mode
- Slave mode

Each data bit transferred on the SDA line is synchronized by a high to the low pulse of each clock on the SCL line.

According to **I2C protocols**, the data line can not change when the clock line is high, it can change only when the clock line is low. The 2 lines are open drain, hence a pull-up resistor is required so that the lines are high since the devices on the I2C bus are active low. The data is transmitted in the form of packets which comprise 9 bits. 

The sequence of these bits are –

1. **Start Condition** - 1 bit
2. **Slave Address** - 8 bit
3. **Acknowledge** - 1 bit

**Advantages :**

- Can be configured in multi-master mode.
- Complexity is reduced because it uses only 2 bi-directional lines (unlike SPI Communication).
- Cost-efficient.
- It uses the ACK / NACK feature due to which it has improved error handling capabilities.

#### SPI Protocol

SPI stands for the serial peripheral interface. It is one of the serial communication protocol developed by Motorola. Sometimes SPI protocol is also called a 4-wire protocol. It requires four wires MOSI, MISO, SS, and SCLK.SPI protocol used to communicate the master and slave devices. The master first configures the clock using a frequency.

The master then selects the particular slave device for communication by pulling the chip select button. That particular device is selected and starts the communication between the master and that particular slave. The master selects only one slave at a time. It is a full-duplex communication protocol. Not limited to 8-bit words in the case of bit transferring.

#### CAN Protocol

CAN stands for the controller area network. It is a serial communication protocol. It requires two wires CAN High (H+) and CAN low (H-). It was developed by the Robert bosh company in 1985 for in-vehicle networks. It is based on a message-oriented transmission protocol.

