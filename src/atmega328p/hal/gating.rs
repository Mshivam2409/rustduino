////// Power reduction for ATmega328p chip
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
pub struct Power {
    //power reduction register control bits for power management.
    prr: u8,
}
///returns mutable reference to Power struct
impl Power {
    pub fn get() -> &'static mut Self {
        unsafe { &mut *(0x64 as *mut Self) }
    }
    
    pub fn twi(&mut self) {
        unsafe {
            let mut ctrl_twi = core::ptr::read_volatile(&mut self.prr);
            ctrl_twi |= 0x80;
            //setting 7th bit shuts down the TWI by stopping the clock to the module.
            core::ptr::write_volatile(&mut self.prr, ctrl_twi);
        }
    }
    pub fn timer2(&mut self) {
        unsafe {
            let mut ctrl_timer2 = core::ptr::read_volatile(&mut self.prr);
            ctrl_timer2 |= 0x40;
            //setting 6th bit shuts down the Timer/Counter2 module in synchronous mode 
            core::ptr::write_volatile(&mut self.prr, ctrl_timer2);
        }
    }
    pub fn timer0(&mut self) {
        unsafe {
            let mut ctrl_timer0 = core::ptr::read_volatile(&mut self.prr);
            ctrl_timer0 |= 0x20;
            //setting 5th bit shuts down the Timer/Counter0 module.
            core::ptr::write_volatile(&mut self.prr, ctrl_timer0);
        }
    }
    pub fn timer1(&mut self) {
        unsafe {
            let mut ctrl_timer1 = core::ptr::read_volatile(&mut self.prr);
            ctrl_timer1 |= 0x8;
            //setting 3rd bit shuts down the Timer/Counter1 module.
            core::ptr::write_volatile(&mut self.prr, ctrl_timer1);
        }
    }
    pub fn spi(&mut self) {
        unsafe {
            let mut ctrl_spi = core::ptr::read_volatile(&mut self.prr);
            ctrl_spi |= 0x4;
            //setting 2nd bit shuts down the serial peripheral interface by stopping the clock to the module.
            core::ptr::write_volatile(&mut self.prr, ctrl_spi);
        }
    }
    pub fn usart0(&mut self) {
        unsafe {
            let mut ctrl_usart0 = core::ptr::read_volatile(&mut self.prr);
            ctrl_usart0 |= 0x2;
            //setting 1st bit shuts down the USART by stopping the clock to the module
            core::ptr::write_volatile(&mut self.prr, ctrl_usart0);
        }
    }
    pub fn adc(&mut self) {
        unsafe {
            let mut ctrl_adc = core::ptr::read_volatile(&mut self.prr);
            ctrl_adc |= 0x1;
            //setting 0th bit shuts down the ADC
            core::ptr::write_volatile(&mut self.prr, ctrl_adc);
        }
    }
 ///disable_clock function disables the mode as per requirement
    pub fn disable_clock(mode: Peripherals) {
        match mode {
            Peripherals::TWI => Power::twi(&mut Power::get()),
            Peripherals::Timer2 => Power::timer2(&mut Power::get()),
            Peripherals::Timer0 => Power::timer0(&mut Power::get()),
            Peripherals::Timer1 => Power::timer1(&mut Power::get()),
            Peripherals::SPI => Power::spi(&mut Power::get()),
            Peripherals::USART0 => Power::usart0(&mut Power::get()),
            Peripherals::ADC => Power::adc(&mut Power::get()),
        }
    }
}
