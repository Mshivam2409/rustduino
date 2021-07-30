//      RustDuino : A generic HAL implementation for Arduino Boards in Rust
//      Copyright (C) 2021  Devansh Kumar Jha, Indian Institute of Technology Kanpur
//
//      This program is free software: you can redistribute it and/or modify
//      it under the terms of the GNU Affero General Public License as published
//      by the Free Software Foundation, either version 3 of the License, or
//      (at your option) any later version.
//
//      This program is distributed in the hope that it will be useful,
//      but WITHOUT ANY WARRANTY; without even the implied warranty of
//      MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//      GNU Affero General Public License for more details.

//! Servomotors are small devices containing embedded mechanics and electronics.
//! There are widely used in modelism, robotics and other applications.
//! This source code controls the functioning of servo motor
//! which can measure their position and velocity on their own.

// Source codes required
use crate::hal::pin::Pins;

/// Structure to control the Servo Motor
#[repr(C, packed)]
pub struct Servo {
    servo: Pins,
    pinno: usize,
}

impl Servo {
    /// New structure declaration for servo motor.
    /// # Returns
    /// * `a mutable reference to Servo` - Which will be used for further implementations.
    pub unsafe fn new() -> Servo {
        let num = (Pins::new()).digital[13].pinno as usize;
        Servo {
            servo: Pins::new(),
            pinno: num,
        }
    }

    /// Function to attach the servo motor to a particular I/O pin
    /// of the micro controller.
    /// However care has to be taken that only correct digital pins are used for this.
    /// Only 2-13 and 44-46 digital pins can be used in this function, other pins will lead to crash.
    /// All pin except 4 and 13 are set to give output at 490 hertz.
    /// pin 4 and 13 will give output at 980 hertz.
    /// # Arguments
    /// * `pinno` - a u32, defining the pin number of the digital pin to use. By default it is Digital Pin 13.
    pub fn attach(&mut self, pinno: usize) {
        self.servo.digital[pinno].set_output();
        self.servo.digital[pinno].high();
        self.pinno = pinno;
    }

    /// It is used to set the Servo Motor according to the value given by the user.
    /// The value given will decide the RPM of motor.
    /// # Arguments
    /// * `value` - a u8, defining for the functioning of servo motor.
    pub fn write(&mut self, value: u8) {
        self.servo.digital[self.pinno].write(value);
    }
}
