// RustDuino : A generic HAL implementation for Arduino Boards in Rust
// Copyright (C) 2021  Sanmati Pande, Indian Institute of Technology Kanpur

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

/// Power reduction for ATmega328p chip
/// Each of the Peripherals below refers to a bit in the PRR
/// Setting 7th bit shuts down the TWI(2-wire serial interface) by stopping the clock to the module.
/// Setting 6th bit shuts down the Timer/Counter2 module in synchronous mode.
/// Setting 5th bit shuts down the Timer/Counter0 module.
/// Setting 3rd bit shuts down the Timer/Counter1 module.
/// Setting 2nd bit shuts down the serial peripheral interface by stopping the clock to the module.
/// Setting 1st bit shuts down the USART by stopping the clock to the module.
/// Setting 0th bit shuts down the ADC.
pub enum Peripherals {
    TWI,
    Timer2,
    Timer0,
    Timer1,
    SPI, //serial peripheral interface
    USART0,
    ADC,
}
///registers controlling power management
/// Section 9.11 of ATmega328p Datasheet
///
///Power Reduction Register control bits for power management.
pub struct Power {
  pub prr: u8,
}

impl Power {
    pub fn new() -> &'static mut Self {
        unsafe { &mut *(0x64 as *mut Self) }
    }

    pub fn twi(&mut self) {
        unsafe {
            let mut ctrl_twi = core::ptr::read_volatile(&mut self.prr);
            ctrl_twi |= 0x80;
            core::ptr::write_volatile(&mut self.prr, ctrl_twi);
        }
    }
    pub fn timer2(&mut self) {
        unsafe {
            let mut ctrl_timer2 = core::ptr::read_volatile(&mut self.prr);
            ctrl_timer2 |= 0x40;
            core::ptr::write_volatile(&mut self.prr, ctrl_timer2);
        }
    }
    pub fn timer0(&mut self) {
        unsafe {
            let mut ctrl_timer0 = core::ptr::read_volatile(&mut self.prr);
            ctrl_timer0 |= 0x20;
            core::ptr::write_volatile(&mut self.prr, ctrl_timer0);
        }
    }
    pub fn timer1(&mut self) {
        unsafe {
            let mut ctrl_timer1 = core::ptr::read_volatile(&mut self.prr);
            ctrl_timer1 |= 0x8;
            core::ptr::write_volatile(&mut self.prr, ctrl_timer1);
        }
    }
    pub fn spi(&mut self) {
        unsafe {
            let mut ctrl_spi = core::ptr::read_volatile(&mut self.prr);
            ctrl_spi |= 0x4;
            core::ptr::write_volatile(&mut self.prr, ctrl_spi);
        }
    }
    pub fn usart0(&mut self) {
        unsafe {
            let mut ctrl_usart0 = core::ptr::read_volatile(&mut self.prr);
            ctrl_usart0 |= 0x2;
            core::ptr::write_volatile(&mut self.prr, ctrl_usart0);
        }
    }
    pub fn adc(&mut self) {
        unsafe {
            let mut ctrl_adc = core::ptr::read_volatile(&mut self.prr);
            ctrl_adc |= 0x1;
            core::ptr::write_volatile(&mut self.prr, ctrl_adc);
        }
    }
    ///Disables the clock
    pub fn disable_clock(mode: Peripherals) {
        match mode {
            Peripherals::TWI => Power::twi(&mut Power::new()),
            Peripherals::Timer2 => Power::timer2(&mut Power::new()),
            Peripherals::Timer0 => Power::timer0(&mut Power::new()),
            Peripherals::Timer1 => Power::timer1(&mut Power::new()),
            Peripherals::SPI => Power::spi(&mut Power::new()),
            Peripherals::USART0 => Power::usart0(&mut Power::new()),
            Peripherals::ADC => Power::adc(&mut Power::new()),
        }
    }
}
