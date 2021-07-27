//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Nikhil Gupta,Indian Institute of Technology Kanpur
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

//! Control on Watchdog timer in ATMEGA2560P
//! Section 12.5 and 28.6 of manual
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf

use volatile::Volatile;

/// Crates required in the code for reading and writing to registers.
use crate::atmega2560p::hal::interrupts;

/// Use interrupts to enable/disable global interrupts,
/// prior to disabling watchdog, all interrupts must be disabled.
/// A new struct of WatchDog can be created through new() function.
/// Watchdog can be disabled by disable() function.
pub struct WatchDog {
    pub mcusr: Volatile<u8>,
    _pad: [u8; 11],
    pub wdtcsr: Volatile<u8>,
}

impl WatchDog {
    ///Creates new struct of Watchdog.
    ///Return its mut static Refrence.
    pub unsafe fn new() -> &'static mut WatchDog {
        &mut *(0x54 as *mut WatchDog)
    }

    ///This function disables WatchDog.
    ///Reset watchdog to stop its functioning at end of timer
    pub fn disable(&mut self) {
        unsafe {
            // Disable global interrupts.
            interrupts::GlobalInterrupts::disable(&mut interrupts::GlobalInterrupts::new());
        }
        // Clears WDRF in MCUSR.
        let mut mcusr = self.mcusr.read();
        mcusr &= !(1 << 3);
        self.mcusr.write(mcusr);

        let mut wdtcsr = self.wdtcsr.read();
        wdtcsr |= (1 << 4) | (1 << 3);
        //Sets WDCE for changing WDE.
        self.wdtcsr.write(wdtcsr);
        //Sets every bit to 0 including WDE and WDIE.
        self.wdtcsr.write(0x00);
        //Enables globalinterrupts again.
        unsafe {
            interrupts::GlobalInterrupts::enable(&mut interrupts::GlobalInterrupts::new());
        }
    }
}
