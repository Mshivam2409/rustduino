---
id: atmega328p
slug: /atmega328p
title: AtMega328P
---

*Our First Chip*

----

The high-performance Microchip picoPower® 8-bit AVR® RISC-based microcontroller **AtMega328p** ([datasheet](http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf))
combines 32 KB ISP Flash memory with read-while-write capabilities, 1024B
EEPROM, 2 KB SRAM, 23 general purpose I/O lines, 32 general purpose working
registers, three flexible timer/counters with compare modes, internal and
external interrupts, serial programmable USART, a byte-oriented Two-Wire serial
interface, SPI serial port, a 6-channel 10-bit A/D converter (8-channels in TQFP
and QFN/MLF packages), programmable watchdog timer with internal oscillator, and
five software selectable power saving modes.

**Arduino Uno** is a microcontroller board based on the **ATmega328P** (datasheet). It
has 14 digital input/output pins (of which 6 can be used as PWM outputs), 6
analog inputs, a 16 MHz ceramic resonator, a USB connection, a power jack, an
ICSP header and a reset button.

**The Arduino Nano** is a small, complete, and breadboard-friendly board based on
the ATmega328P (Arduino Nano 3.x). It has more or less the same functionality of
the Arduino Uno, but in a different package

## Compiling and Linking

```bash
$ cargo +nightly build -Z build-std=core --release --target avr-atmega328p.json
$ cargo +nightly build --release
```

Then, to upload it to a device, assuming that you have avrdude installed, run:

```bash
$ avrdude -v -patmega328p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-atmega328p/release/examples/serial.elf:e
```
