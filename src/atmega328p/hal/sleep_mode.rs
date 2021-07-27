// RustDuino : A generic HAL implementation for Arduino Boards in Rust
// Copyright (C) 2021  Akshit Verma, Indian Institute of Technology Kanpur

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>

/// Power management for ATmega328p chip using sleep modes.
use volatile::Volatile;

/// Contains sleep modes.
///
/// Section 9.1 of ATmega328p datasheet.

/// ~ Idle: This  mode makes the MCU enter idle mode, stopping the CPU but
/// allowing the SPI, USART, analog comparator, ADC, 2-wire serial
/// interface, Timer/Counters, watchdog, and the interrupt system
/// to continue operating. This sleep mode basically halts clkCPU
/// and clkFLASH, while allowing the other clocks to run.

/// ~ ADCNR: ADC Noise Reducion mode makes the MCU enter ADC noise reduction
/// mode, stopping the CPU but allowing the ADC, the external interrupts,
/// the 2-wire serial interface address watch, Timer/Counter2, and the
/// watchdog to continue operating (if enabled). This sleep mode
/// basically halts clkI/O, clkCPU, and clkFLASH, while allowing the
/// other clocks to run.

/// ~ PowerDown: Power Down mode makes the MCU enter power-down mode. In this mode, the
/// external oscillator is stopped, while the external interrupts, the
/// 2-wire serial interface address watch, and the watchdog continue
/// operating (if enabled). Only an external reset, a watchdog system
/// reset, a watchdog interrupt, a brown-out reset, a 2-wire serial
/// interface address match, an external level interrupt on INT0 or INT1,
/// or a pin change interrupt can wake up the MCU. This sleep mode basically
/// halts all generated clocks, allowing operation of asynchronous modules only.

/// ~ PowerSave: Power Save mode is identical to Power Down, with one exception:
///
/// If Timer/Counter2 is enabled, it will keep running during sleep. The
/// device can wake up from either timer overflow or output  compare event
/// from Timer/Counter2 if the corresponding Timer/Counter2 interrupt enable
/// bits are set in TIMSK2, and the global interrupt enable bit in SREG is set.

/// ~ Standby: It is identical to Power Down, with one exception:
///
/// If Timer/Counter2 is enabled, it will keep running during sleep. The device
/// can wake up from either timer overflow or output compare event from
/// Timer/Counter2 if the corresponding Timer/Counter2 interrupt enable bits
/// are set in TIMSK2, and the global interrupt enable bit in SREG is set.

/// ~ ExtStandby: Extendend Standby mode is identical to Power Save with the exception that
/// the oscillator is kept running. From extended standby mode, the device
/// wakes up in six clock cycles.

/// Disable: Disables the sleep mode.
pub enum SleepMode {
    Idle,
    ADCNR,
    PowerDown,
    PowerSave,
    Standby,
    ExtStandby,
    Disable,
}

/// Contains registers controlling power management.
///
/// Section 9.11 of ATmega328p Datasheet
pub struct Sleep {
    /// The sleep mode control register contains control bits for power management.
    pub smcr: Volatile<u8>,
}

impl Sleep {
    /// Returns mutable reference to `Sleep` struct to control power management.
    pub fn new() -> &'static mut Self {
        unsafe { &mut *(0x53 as *mut Self) }
    }

    /// Enable `MCU` to enter sleep mode.
    ///
    /// Writes logic one to `SE` bit to make `MCU` enter sleep mode when a `SLEEP`
    /// instruction is executed.
    pub fn idle(&mut self) {
        self.smcr.write(0x1);
    }

    pub fn adcnr(&mut self) {
        self.smcr.write(0x3);
    }

    pub fn power_down(&mut self) {
        self.smcr.write(0x5);
    }

    pub fn power_save(&mut self) {
        self.smcr.write(0x7);
    }

    pub fn standby(&mut self) {
        self.smcr.write(0xD);
    }

    pub fn ext_standby(&mut self) {
        self.smcr.write(0xF);
    }

    pub fn disable(&mut self) {
        self.smcr.write(0x0);
    }
}
/// Enables the Chosen power mode.
pub fn enable_mode(mode: SleepMode) {
    match mode {
        SleepMode::Idle => Sleep::idle(&mut Sleep::new()),
        SleepMode::ADCNR => Sleep::adcnr(&mut Sleep::new()),
        SleepMode::PowerDown => Sleep::power_down(&mut Sleep::new()),
        SleepMode::PowerSave => Sleep::power_save(&mut Sleep::new()),
        SleepMode::Standby => Sleep::standby(&mut Sleep::new()),
        SleepMode::ExtStandby => Sleep::ext_standby(&mut Sleep::new()),
        SleepMode::Disable => Sleep::disable(&mut Sleep::new()),
    }
}
