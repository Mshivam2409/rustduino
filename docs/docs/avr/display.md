---
id: display
slug: /display
title: 7 Segment Display
---

# 7 Segment Display
Seven Segment display is how we see the numbers in modern day electronics. Digits are displayed with the help of 7 bars out of which 4 are vertical and 3 are horizaontal. Decimal point can also be displayed with this display by turning it on/off. 7 Segment display is used as it is easy to operate upon. We can display digits from 0 to 9, decimal point and alphabets from A to F too.

# Code implementation
## Setup
```rust
pub fn setup(datapin: u8, clockpin: u8, latchpin: u8, decpt: bool, common_anode: bool, value: u8)
```
#### Usage
This function is the only one we need to call. We need to tell the pin numbers of datapin, clockpin,latchpin, true/false whether decimal is on/off, true/false if common anode is used/not used and the value to be displayed respectively in the arguments.
Ex- 
```rust
setup(4,7,8,true,true,7) //7 is display on the 7-segment display. 
```
## Update display function
```rust
pub fn myfn_update_display(/*arguments omitted*/)
```
#### Usage
This is a function used internally for the passing out value to right pins.

## Out function
```rust
pub fn out(/*arguments omitted*/)
```
#### Usage
This is a function used internally for coordinating the different functions for output.

## Converting numbers to bits
```rust
pub fn myfn_num_to_bits(somenumber: u8) -> u8
```
#### Usage
This is used to convert interger to bit system which will then be interpreted one-by-one for on/off different segments of 7-segments display.

## Making pin
```rust 
fn make_pin(pin: u8) -> Pin
```
#### Usage
Returns the Pin in response to the pin number passed.