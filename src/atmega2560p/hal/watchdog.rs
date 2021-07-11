/*RustDuino : A generic HAL implementation for Arduino Boards in Rust
Copyright (C) 2021  Nikhil Gupta,

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>*/

use core;

use crate::atmega2560p::hal::interrupts;

///use interrupts to enable/disable global interrupts
///section 28.6 from datasheet for atmega2560p
///prior to disabling watchdog, all interrupts must be disabled
///when WDE and WDIE bits of WDTCSR register  sets to 0, watchdog disables
///WDRF bit of MCUSR register can overwrite WDE ,so,WDRF must be cleared before

/// Contains various registers to control the functioning of registers Watchdog.
/// MCUSR : Contains 5 writable bits which are used for various watchdog settings as below -
/// Bit 0   – PORF : Power-on Reset Flag
/// Bit 1   – EXTRF: External Reset Flag
/// Bit 2   – BORF : Brown-out Reset Flag
/// Bit 3   – WDRF : Watchdog Reset Flag
/// Bit 4   – JTRF : JTAG Reset Flag
/// Bit 5:7 - Res  : Reserved
///
/// WDTCSR : Contains 8 writable bits which are used for various watchdog settings as below -
/// Bit 5, 2:0 - WDP3:0 : Watchdog Timer Prescaler 3, 2, 1 and 0
/// Bit 3      - WDE    : Watchdog System Reset Enable
/// Bit 4      - WDCE   : Watchdog Change Enable
/// Bit 6      - WDIE   : Watchdog Interrupt Enable
/// Bit 7      - WDIF   : Watchdog Interrupt Flag
pub struct WatchDog {
    mcusr: u8,
    _pad: [u8; 11],
    wdtcsr: u8,
}

impl WatchDog {
    ///creates new struct of watchdog
    ///return its mut static refrence
    pub unsafe fn new() -> &'static mut WatchDog {
        &mut *(0x54 as *mut WatchDog)
    }

    ///disable global interrupts
    ///clears WDRF in MCUSR
    ///reset watchdog to stop its functioning at end of timer
    ///enable watchdog again
    pub fn disable(&mut self) {
        unsafe {
            interrupts::GlobalInterrupts::disable(&mut interrupts::GlobalInterrupts::new());

            let mut mcusr = core::ptr::read_volatile(&self.mcusr);
            mcusr &= !(1 << 3);
            core::ptr::write_volatile(&mut self.mcusr, mcusr);

            let mut wdtcsr = core::ptr::read_volatile(&self.wdtcsr);
            wdtcsr |= (1 << 4) | (1 << 3);
            //sets WDCE for changing WDE
            core::ptr::write_volatile(&mut self.wdtcsr, wdtcsr);
            //sets every bit to 0 including WDE and WDIE
            core::ptr::write_volatile(&mut self.wdtcsr, 0x00);
            interrupts::GlobalInterrupts::enable(&mut interrupts::GlobalInterrupts::new());
        }
    }
}
