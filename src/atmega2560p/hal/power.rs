use core;
use core::arch::arm::__nop;

// Section 11.10.2 and 11.10.3 of the manual
// Also references from Section 11.8

// The options correspond to real world as shown - 
       //  TWI =>  Power Reduction TWI     
       //  TIMER2 =>  Power Reduction Timer/Counter2
       //  TIMER0 =>  Power Reduction Timer/Counter0 
       //  TIMER1 =>  Power Reduction Timer/Counter1
       //  SPI => Power Reduction Serial Peripheral Interface
       //  USART0 =>  Power Reduction USART0 
       //  ADC => Power Reduction ADC
       //  TIMER5 =>  Power Reduction Timer/Counter5 
       //  TIMER4 =>  Power Reduction Timer/Counter4
       //  TIMER3 =>  Power Reduction Timer/Counter3 
       //  USART3 => Power Reduction USART3
       //  USART2 => Power Reduction USART2  
       //  USART1 => Power Reduction USART1
// Refer to section 11.10.2 and 11.10.3 at page 55 and 56 of 
// https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf


#[derive(Clone,Copy)]
pub enum Options {
    TWI,      TIMER2,
    TIMER0,   TIMER1,
    SPI, //serial peripheral interface
    USART0,   ADC,
    TIMER5,   TIMER4,
    TIMER3,   USART3,
    USART2,   USART1
}


#[repr(C, packed)]
pub struct Power { 
    PRR0:u8,
    PRR1:u8
}

// mod interrupts;

impl Power {
    pub unsafe fn new() -> &'static *mut Power {
        // Creates a new reference to the Sleep structure 
        &mut *(0x64 as *mut Power)
    }

    pub fn enable(&mut self,mode:Options) {
        unsafe {
            // Here we would create a new element of the structure Power
            // and it would be used to control various clock gating features of the
            // chip ATMEGA2560P

            Power *ptr = new();
            let mut prr = 0;
            match mode {
                {  Options::TWI    || Options::TIMER2 ||
                   Options::TIMER0 || Options::TIMER1 ||
                   Options::SPI    || Options::USART0 || 
                   Options::ADC                              }     => prr = core::ptr::read_volatile(&mut self.PRR0),

                {  Options::TIMER5    || Options::TIMER4 ||
                   Options::TIMER3    || Options::USART3 ||
                   Options::USART2    || Options::USART1     }     => prr = core::ptr::read_volatile(&mut self.PRR1); 

            }

            match mode {
                Options::TWI =>    prr = prr | 0x80,
                Options::TIMER2 => prr = prr | 0x40,
                Options::TIMER0 => prr = prr | 0x20,
                Options::TIMER1 => prr = prr | 0x08,
                Options::SPI =>    prr = prr | 0x04,
                Options::USART0 => prr = prr | 0x02,
                Options::ADC =>    prr = prr | 0x01,
                Options::TIMER5 => prr = prr | 0x20,
                Options::TIMER4 => prr = prr | 0x10,
                Options::TIMER3 => prr = prr | 0x08,
                Options::USART3 => prr = prr | 0x04,
                Options::USART2 => prr = prr | 0x02,
                Options::USART1 => prr = prr | 0x01
            }

            match mode {
                {  Options::TWI    || Options::TIMER2 ||
                   Options::TIMER0 || Options::TIMER1 ||
                   Options::SPI    || Options::USART0 || 
                   Options::ADC                              }     => prr = core::ptr::write_volatile(&mut self.PRR0,prr),

                {  Options::TIMER5    || Options::TIMER4 ||
                   Options::TIMER3    || Options::USART3 ||
                   Options::USART2    || Options::USART1     }     => prr = core::ptr::write_volatile(&mut self.PRR1,prr); 

            }
        }
    }

    pub fn disable(&mut self,mode:Options) {
        unsafe {
            // Here we would create a new element of the structure Power
            // and it would be used to control various clock gating features of the
            // chip ATMEGA2560P
            
            Power *ptr = new();
            let mut prr = 0;
            match mode {
                {  Options::TWI    || Options::TIMER2 ||
                   Options::TIMER0 || Options::TIMER1 ||
                   Options::SPI    || Options::USART0 || 
                   Options::ADC                              }     => prr = core::ptr::read_volatile(&mut self.PRR0),

                {  Options::TIMER5    || Options::TIMER4 ||
                   Options::TIMER3    || Options::USART3 ||
                   Options::USART2    || Options::USART1     }     => prr = core::ptr::read_volatile(&mut self.PRR1); 

            }

            match mode {
                Options::TWI =>    prr = prr & 0x7F,
                Options::TIMER2 => prr = prr & 0xBF,
                Options::TIMER0 => prr = prr & 0xDF,
                Options::TIMER1 => prr = prr & 0xF7,
                Options::SPI =>    prr = prr & 0xFB,
                Options::USART0 => prr = prr & 0xFD,
                Options::ADC =>    prr = prr & 0xFE,
                Options::TIMER5 => prr = prr & 0xDF,
                Options::TIMER4 => prr = prr & 0xEF,
                Options::TIMER3 => prr = prr & 0xF7,
                Options::USART3 => prr = prr & 0xFB,
                Options::USART2 => prr = prr & 0xFD,
                Options::USART1 => prr = prr & 0xFE
            }

            match mode {
                {  Options::TWI    || Options::TIMER2 ||
                   Options::TIMER0 || Options::TIMER1 ||
                   Options::SPI    || Options::USART0 || 
                   Options::ADC                              }     => prr = core::ptr::write_volatile(&mut self.PRR0,prr),

                {  Options::TIMER5    || Options::TIMER4 ||
                   Options::TIMER3    || Options::USART3 ||
                   Options::USART2    || Options::USART1     }     => prr = core::ptr::write_volatile(&mut self.PRR1,prr); 

            }
        }
    }

    // These are the functions directly to be controlled by the user using calls in the main function
    // We will provide seperate functions for enabling "power-down" and "power-up" features

    // This is the function for disabling the clock system of your choice
    pub fn disable_clocks(&mut self,mode: Options) {
        // Creates a new instance to the power structure
        Power *power = new();
        match mode {
            Options::TWI => power.disable(mode),
            Options::TIMER2 => power.disable(mode),
            Options::TIMER0 => power.disable(mode),
            Options::TIMER1 => power.disable(mode),
            Options::SPI => power.disable(mode),
            Options::USART0 => power.disable(mode),
            Options::ADC => power.disable(mode),
            Options::TIMER5 => power.disable(mode),
            Options::TIMER4 => power.disable(mode),
            Options::TIMER3 => power.disable(mode),
            Options::USART3 => power.disable(mode),
            Options::USART2 => power.disable(mode),
            Options::USART1 => power.disable(mode),
            _ => unreachable!()
        }
    }

    // This is the function for enabling back the clock system of your choice
    // User has to make sure that these functions are called after disabling functions
    // otherwise they have no effect.
    pub fn enable_clocks(&mut self,mode: Options) {
        // Creates a new instance to the power structure
        Power *power = new();
        match mode {
            Options::TWI =>    power.enable(mode),
            Options::TIMER2 => power.enable(mode),
            Options::TIMER0 => power.enable(mode),
            Options::TIMER1 => power.enable(mode),
            Options::SPI =>    power.enable(mode),
            Options::USART0 => power.enable(mode),
            Options::ADC =>    power.enable(mode),
            Options::TIMER5 => power.enable(mode),
            Options::TIMER4 => power.enable(mode),
            Options::TIMER3 => power.enable(mode),
            Options::USART3 => power.enable(mode),
            Options::USART2 => power.enable(mode),
            Options::USART1 => power.enable(mode),
            _ => unreachable!()
        }
    }
    
}
