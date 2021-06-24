/// Power management for ATmega328p chip using sleep modes.

/// Contains sleep modes.
///
/// Section 9.1 of ATmega328p datasheet.
pub enum SleepMode {
    /// Idle mode makes the MCU enter idle mode, stopping the CPU but
    /// allowing the SPI, USART, analog comparator, ADC, 2-wire serial
    /// interface, Timer/Counters, watchdog, and the interrupt system
    /// to continue operating. This sleep mode basically halts clkCPU
    /// and clkFLASH, while allowing the other clocks to run.
    Idle,

    /// ADC Noise Reducion mode makes the MCU enter ADC noise reduction
    /// mode, stopping the CPU but allowing the ADC, the external interrupts,
    /// the 2-wire serial interface address watch, Timer/Counter2, and the
    /// watchdog to continue operating (if enabled). This sleep mode
    /// basically halts clkI/O, clkCPU, and clkFLASH, while allowing the
    /// other clocks to run.
    ADCNR,

    /// Power Down mode makes the MCU enter power-down mode. In this mode, the
    /// external oscillator is stopped, while the external interrupts, the
    /// 2-wire serial interface address watch, and the watchdog continue
    /// operating (if enabled). Only an external reset, a watchdog system
    /// reset, a watchdog interrupt, a brown-out reset, a 2-wire serial
    /// interface address match, an external level interrupt on INT0 or INT1,
    /// or a pin change interrupt can wake up the MCU. This sleep mode basically
    /// halts all generated clocks, allowing operation of asynchronous modules only.
    PowerDown,

    /// Power Save mode is identical to Power Down, with one exception:
    ///
    /// If Timer/Counter2 is enabled, it will keep running during sleep. The
    /// device can wake up from either timer overflow or output  compare event
    /// from Timer/Counter2 if the corresponding Timer/Counter2 interrupt enable
    /// bits are set in TIMSK2, and the global interrupt enable bit in SREG is set.
    PowerSave,

    /// This mode is identical to Power Down, with one exception:
    ///
    /// If Timer/Counter2 is enabled, it will keep running during sleep. The device
    // can wake up from either timer overflow or output compare event from
    /// Timer/Counter2 if the corresponding Timer/Counter2 interrupt enable bits
    /// are set in TIMSK2, and the global interrupt enable bit in SREG is set.
    Standby,

    /// Extendend Standby mode is identical to Power Save with the exception that
    /// the oscillator is kept running. From extended standby mode, the device
    /// wakes up in six clock cycles.
    ExtStandby,
}

/// Contains registers controlling power management.
///
/// Section 9.11 of ATmega328p Datasheet
pub struct Sleep {
    /// The sleep mode control register contains control bits for power management.
    smcr: u8,
    _pad1: u8,
    mcucr: u8,
    _pad2: [u8; 14],
    prr: u8,
}

impl Sleep {
    /// Returns mutable reference to `Sleep` struct to control power management.
    pub fn get() -> &'static mut Self {
        unsafe { &mut *(0x53 as *mut Self) }
    }

    /// Enable `MCU` to enter sleep mode.
    ///
    /// Writes logic one to `SE` bit to make `MCU` enter sleep mode when a `SLEEP`
    /// instruction is executed.
    pub fn idle(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut self.smcr, 0x1);
        }
    }

    pub fn adcnr(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut self.smcr, 0x3);
        }
    }

    pub fn power_down(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut self.smcr, 0x5);
        }
    }

    pub fn power_save(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut self.smcr, 0x7);
        }
    }

    pub fn standby(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut self.smcr, 0xD);
        }
    }

    pub fn ext_standby(&mut self) {
        unsafe {
            core::ptr::write_volatile(&mut self.smcr, 0xF);
        }
    }
}

pub fn enable_mode(mode: SleepMode) {
    match mode {
        SleepMode::Idle => Sleep::idle(&mut Sleep::get()),
        SleepMode::ADCNR => Sleep::adcnr(&mut Sleep::get()),
        SleepMode::PowerDown => Sleep::power_down(&mut Sleep::get()),
        SleepMode::PowerSave => Sleep::power_save(&mut Sleep::get()),
        SleepMode::Standby => Sleep::standby(&mut Sleep::get()),
        SleepMode::ExtStandby => Sleep::ext_standby(&mut Sleep::get()),
    }
}
//disable the sleep mode by changing 0th bit to 0
pub fn disable_mode(&mut self){
    unsafe{
        core::ptr::write_volatile(&mut self.smcr, 0x0);
    }
}
