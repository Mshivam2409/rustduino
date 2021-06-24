pub enum Peripherals{
TWI,
Timer2,
Timer0,
Timer1,
SPI,  //serial peripheral interface
USART0,
ADC,
}
///registers controlling power management
pub struct Power{
prr:u8,
}
///returns mutable reference to Power struct
impl Power{
pub fn get()->&'static mut Self{
        unsafe { &mut*(0x64 as *mut Self)}
}
pub fn twi(&mut self) {
       unsafe {
           let mut ctrl_twi=core::ptr::read_volatile(&mut self.prr);
           ctrl_twi |=0x80;
           core::ptr::write_volatile(&mut self.prr, ctrl_twi);
       }
}
pub fn timer2(&mut self) {
       unsafe {
            let mut ctrl_timer2=core::ptr::read_volatile(&mut self.prr);
            ctrl_timer2 |=0x40;
            core::ptr::write_volatile(&mut self.prr, ctrl_timer2);
       }
}
pub fn timer0(&mut self) {
       unsafe {
           let mut ctrl_timer0=core::ptr::read_volatile(&mut self.prr);
           ctrl_timer0 |=0x20;
           core::ptr::write_volatile(&mut self.prr, ctrl_timer0);
           
       }
}
pub fn timer1(&mut self) {
       unsafe {

           let mut ctrl_timer1=core::ptr::read_volatile(&mut self.prr);
           ctrl_timer1 |=0x8;
           core::ptr::write_volatile(&mut self.prr, ctrl_timer1);
       }
}
pub fn spi(&mut self) {
       unsafe {

           let mut ctrl_spi=core::ptr::read_volatile(&mut self.prr);
           ctrl_spi |=0x4;
           core::ptr::write_volatile(&mut self.prr, ctrl_spi);
       }
}
pub fn usart0(&mut self) {
       unsafe {

           let mut ctrl_usart0=core::ptr::read_volatile(&mut self.prr);
           ctrl_usart0 |=0x2;
           core::ptr::write_volatile(&mut self.prr, ctrl_usart0);
       }
}
pub fn adc(&mut self) {
       unsafe {
           let mut ctrl_adc=core::ptr::read_volatile(&mut self.prr);
           ctrl_adc |=0x1;
           core::ptr::write_volatile(&mut self.prr, ctrl_adc);
       }
}
pub fn disable_clock(mode: Peripherials) {
    match mode {
        Peripherials::TWI => Power::twi(&mut Power::get()),
        Peripherials::Timer2 => Power::timer2(&mut Power::get()),
        Peripherials::Timer0 => Power::timer0(&mut Power::get()),
        Peripherials::Timer1 => Power::timer1(&mut Power::get()),
        Peripherials::SPI=> Power::spi(&mut Power::get()),
        Peripherials::USART0 => Power::usart0(&mut Power::get()),
        Peripherials::ADC => Power::adc(&mut Power::get())
    }
}


         
