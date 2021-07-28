// RustDuino : A generic HAL implementation for Arduino Boards in Rust
// Copyright (C) 2021  Akshit Verma, Indian Institute of Technology Kanpur
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>

//! Control on Watchdog timer in ATMEGA328P
//! Watchdog timer 10.9 of the manual.

use crate::atmega328p::hal::interrupts;
use core::ptr::{read_volatile, write_volatile};

/// MCUSR (MCU Status Register)
/// The MCU status register provides information on which reset source caused an MCU reset.
///
/// WDTCSR (Watchdog Timer Control Register).
/// Used to control the action of timer on timeout.
///       Mode Action                                        on Time-out
///         Stopped                                             None
///     Interrupt mode                                        Interrupt
///     System reset mode                                       Reset
/// Interrupt and system reset mode         Interrupt, then go to system reset mode
#[repr(C, packed)]
pub struct WatchDog {
    mcusr: u8,
    _pad: [u8; 10],
    wdtcsr: u8,
}

impl WatchDog {
    /// Creates new struct of Watchdog.
    /// # Returns
    /// * `a reference to Watchdog structure` - for further implementations.
    pub unsafe fn new() -> &'static mut WatchDog {
        &mut *(0x55 as *mut WatchDog)
    }

    /// Resets watchdog timer.
    pub fn reset_watchdog(&mut self) {
        unsafe {
            let mut ctrl_mcusr = read_volatile(&self.mcusr);
            ctrl_mcusr &= 0x7;
            write_volatile(&mut self.mcusr, ctrl_mcusr);
        }
    }

    /// Disables watchdog
    pub fn disable(&mut self) {
        unsafe {
            interrupts::Interrupt::disable(&mut interrupts::Interrupt::new());
            WatchDog::reset_watchdog(&mut WatchDog::new());
            let mut ctrl_wdtcsr = read_volatile(&self.wdtcsr);
            ctrl_wdtcsr |= 0x18;
            write_volatile(&mut self.wdtcsr, ctrl_wdtcsr);
            write_volatile(&mut self.wdtcsr, 0x00);
            interrupts::Interrupt::enable(&mut interrupts::Interrupt::new());
        }
    }
}
