---
id: atmega2560p
slug: /atmega2560p
title: AtMega2560P
---

*our second chip*
----

ATmega2560
([datasheet](https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf))is
a microcontroller featuring the 8-bit AVR® processor based on the AVR enhanced RISC architecture -
running with up to 256 KB of In-System Programmable
Flash with Read-While-Write capabilities, 8 KB SRAM and 4 KB of
EEPROM ,54/86 general purpose I/O lines, 32 general purpose working registers, Real Time Counter (RTC), 
six flexible Timer/Counters with compare modes and PWM, four USARTs, a byte oriented 2-wire Serial Interface,
a 16-channel, 10-bit ADC with optional differential input stage with programmable gain, programmable Watchdog Timer with Internal Oscillator,
an SPI serial port, IEEE® std.. The series uses the latest Core Independent
Peripherals with low-power features, including an Event System, intelligent
analog, and advanced peripherals.

**Arduino Mega** is a tiny powerful board that is based on the
**ATMega2560 AVR** processor. The Arduino Nano Every is almost similar to the
Arduino Nano board with the addition of a more powerful processor like
Atmega2560. This board comes with more program memory compared to Arduino Uno
and RAM is 200% bigger, helping us create a lot of variables.

## Compiling and Linking

```bash
$ cargo +nightly build -Z build-std=core --release --target avr-atmega2560.json
$ cargo +nightly build --release
```

Then, to upload it to a device, assuming that you have avrdude installed, run:

```bash
$ avrdude -v -patmega2560p -carduino -P/dev/ttyACM0 -b115200 -D -Uflash:w:target/avr-atmega2560p/release/examples/serial.elf:e
```
