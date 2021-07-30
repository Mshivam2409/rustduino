---
id: servo
slug: /servo
title: Servo Motor
---

*Maybe it can work like a mini fan !*

---

## Description
**Servomotors** are small devices containing embedded mechanics and electronics. There are widely used in modelism, robotics and other applications. Their name comes from the fact that they control their position (or velocity) on their own.
Basically, a servomotor is composed with a small dc motor, a gearbox and embedded electronics that can be easily commanded using Pulse Width Modulation (PWM) from a microcontroller.

**Wiring**
Servomotor is powered through the black/brown cable (GND) and the red cable (+5V) and recieve the PWM signal on the yellow/white cable (pin9). Depending on the number of or power of servomotors you want to use, servomotor can be powered by the Arduino board but there is usually an external power source. The board is powered by the computer via the USB cable.


## Servo Control
Servo motors are controlled by sending a PWM (pulse-width modulation) signal to the signal line of the servo. The width of the pulses determines the position of the output shaft. When you send the servo a signal with a pulse width of 1.5 milliseconds (ms), the servo will move to the neutral position (90 degrees). The min (0 degrees) and max (180 degrees) position typically correspond to a pulse width of 1 ms and 2 ms respectively. Note this can vary slightly between different types and brands of servo motors (e.g. 0.5 and 2.5 ms). Many servos only rotate through about 170 degrees (or even only 90) but the middle position is almost always at 1.5 ms.


## Struct Definition

```rust
pub struct Servo { /* fields omitted */ }
```

### Impl `new` of `Servo`

New structure declaration for servo motor.
```rust
pub unsafe fn new() -> &'static mut Servo { /* code follows */ }
```

#### Usage

```rust
use rustduino::sensor::servo::Servo;
let servo = Servo::new();
```

### Impl `attach` of `Servo`

Function to attach the servo motor to a particular I/O pin of the micro controller.
```rust
pub fn attach(&mut self,pinno : usize) { /* fields omitted */ }
```

#### Usage

```rust
use rustduino::sensor::servo::Servo;
let mut servo = Servo::new();
servo.attach(13);
```


### Impl `write` of `Servo`

It is used to set the Servo Motor according to the value given by the user.
The value given will decide the RPM of motor.
```rust
pub fn write(&mut self,value : u8) { /* fields omitted */ }
```

#### Usage

```rust
use rustduino::sensor::servo::Servo;
let mut servo = Servo::new();
servo.write(10);
```