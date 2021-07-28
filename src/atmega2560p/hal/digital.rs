//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Satender Kumar Yadav,Indian Institute of Technology Kanpur
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Affero General Public License as published
//     by the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Affero General Public License for more details.
//
//     You should have received a copy of the GNU Affero General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>

//! This source code creates a array for controlling all digital pins at one place in form
//! Pins array which would be used so that we get meaningful functions to work upon and
//! also the implementation of rustduino library is easier for the user.
//! For more details see section 16,17,25 and 26 of ATMEGA2560P datasheet.

// Include the required source codes.
use crate::atmega2560p::hal::pin::*;
use core::ptr::{read_volatile, write_volatile};

impl DigitalPin {
    /// Toggles the appropriate bit in PINxn register so that the mode of the pin
    /// is changed from high to low or vice versa.
    pub fn toggle(&mut self) {
        unsafe { write_volatile(&mut (*self.pin.port).pin, 0x1 << self.pin.pin) }
    }

    /// Set the pin to high output value.
    pub fn high(&mut self) {
        // Checks if pin number is valid.
        if self.pin.pin >= 8 {
            return;
        }
        let mut p = unsafe { read_volatile(&mut (*self.pin.port).port) }; // Reading the value of PORTxn.
        p = p & (1 << self.pin.pin);
        let ddr_value = unsafe { read_volatile(&mut (*self.pin.port).ddr) }; // Read the DDRxn register.
        if p == 0 && ddr_value == (0x1 << self.pin.pin) {
            // Toggling the value of PORTxn, if it isn't set to high.
            self.toggle();
        }
    }

    /// Sets the pin to low output value.
    pub fn low(&mut self) {
        // Check if pin number is valid.
        if self.pin.pin >= 8 {
            return;
        }
        let mut p = unsafe { read_volatile(&mut (*self.pin.port).port) }; //Reading the value of PORTxn.
        p = p & (1 << self.pin.pin);
        let ddr_value = unsafe { read_volatile(&mut (*self.pin.port).ddr) }; // Read the DDRxn register.
        if p != 0 && ddr_value == (0x1 << self.pin.pin) {
            //Toggling the value of PORTxn, if it isn't set to low.
            self.toggle();
        }
    }
}
