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

    pub fn enable_twi(&mut self) {
        unsafe {
            // A instance to the PRR0 register is created and the corresponding bit is controlled
            // to disable various clocks in the system.
            Power *ptr = new();
            let mut twi = core::ptr::read_volatile(&mut self.PRR0);
            twi = twi | 0x80;
            core::ptr::write_volatile(&mut self.PRR0, twi);
        }
    }

    pub fn enable_timer2(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut timer2 = core::ptr::read_volatile(&mut self.PRR0);
            timer2 = timer2 | 0x40;
            core::ptr::write_volatile(&mut self.PRR0, timer2);
        }
    }

    pub fn enable_timer0(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut timer0 = core::ptr::read_volatile(&mut self.PRR0);
            timer0 = timer0 | 0x20;
            core::ptr::write_volatile(&mut self.PRR0, timer0);
        }
    }

    pub fn enable_timer1(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut timer1 = core::ptr::read_volatile(&mut self.PRR0);
            timer1 = timer1 | 0x08;
            core::ptr::write_volatile(&mut self.PRR0, timer1);
        }
    }

    pub fn enable_spi(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut spi = core::ptr::read_volatile(&mut self.PRR0);
            spi = spi | 0x04;
            core::ptr::write_volatile(&mut self.PRR0, spi);
        }
    }

    pub fn enable_usart0(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut usart0 = core::ptr::read_volatile(&mut self.PRR0);
            usart0 = usart0 | 0x02;
            core::ptr::write_volatile(&mut self.PRR0, usart0);
        }
    }

    pub fn enable_adc(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut adc = core::ptr::read_volatile(&mut self.PRR0);
            adc = adc | 0x01;
            core::ptr::write_volatile(&mut self.PRR0, adc);
        }
    }

    pub fn enable_timer5(&mut self) {
        unsafe {
            // A instance to the PRR1 register is created and the corresponding bit is controlled
            // to disable various clocks in the system.
            Power *ptr = new();
            let mut twi = core::ptr::read_volatile(&mut self.PRR1);
            timer5 = timer5 | 0x20;
            core::ptr::write_volatile(&mut self.PRR1, twi);
        }
    }

    pub fn enable_timer4(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut timer4 = core::ptr::read_volatile(&mut self.PRR1);
            timer4 = timer4 | 0x10;
            core::ptr::write_volatile(&mut self.PRR1, timer4);
        }
    }

    pub fn enable_timer3(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut timer3 = core::ptr::read_volatile(&mut self.PRR1);
            timer3 = timer3 | 0x08;
            core::ptr::write_volatile(&mut self.PRR1, timer3);
        }
    }

    pub fn enable_usart3(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut usart3 = core::ptr::read_volatile(&mut self.PRR1);
            usart3 = usart3 | 0x04;
            core::ptr::write_volatile(&mut self.PRR1, usart3);
        }
    }

    pub fn enable_usart2(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut usart2 = core::ptr::read_volatile(&mut self.PRR1);
            usart2 = usart2 | 0x02;
            core::ptr::write_volatile(&mut self.PRR1, usart2);
        }
    }

    pub fn enable_usart1(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut usart1 = core::ptr::read_volatile(&mut self.PRR1);
            usart1 = usart1 | 0x01;
            core::ptr::write_volatile(&mut self.PRR1, usart1);
        }
    }


    pub fn disable_twi(&mut self) {
        unsafe {
            // A instance to the PRR0 register is created and the corresponding bit is controlled
            // to disable various clocks in the system.
            Power *ptr = new();
            let mut twi = core::ptr::read_volatile(&mut self.PRR0);
            twi = twi & 0x7F;
            core::ptr::write_volatile(&mut self.PRR0, twi);
        }
    }

    pub fn disable_timer2(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut timer2 = core::ptr::read_volatile(&mut self.PRR0);
            timer2 = timer2 & 0xBF;
            core::ptr::write_volatile(&mut self.PRR0, timer2);
        }
    }

    pub fn disable_timer0(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut timer0 = core::ptr::read_volatile(&mut self.PRR0);
            timer0 = timer0 & 0xDF;
            core::ptr::write_volatile(&mut self.PRR0, timer0);
        }
    }

    pub fn disable_timer1(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut timer1 = core::ptr::read_volatile(&mut self.PRR0);
            timer1 = timer1 & 0xF7;
            core::ptr::write_volatile(&mut self.PRR0, timer1);
        }
    }

    pub fn disable_spi(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut spi = core::ptr::read_volatile(&mut self.PRR0);
            spi = spi & 0xFB;
            core::ptr::write_volatile(&mut self.PRR0, spi);
        }
    }

    pub fn disable_usart0(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut usart0 = core::ptr::read_volatile(&mut self.PRR0);
            usart0 = usart0 & 0xFD;
            core::ptr::write_volatile(&mut self.PRR0, usart0);
        }
    }

    pub fn disable_adc(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut adc = core::ptr::read_volatile(&mut self.PRR0);
            adc = adc & 0xFE;
            core::ptr::write_volatile(&mut self.PRR0, adc);
        }
    }

    pub fn disable_timer5(&mut self) {
        unsafe {
            // A instance to the PRR1 register is created and the corresponding bit is controlled
            // to disable various clocks in the system.
            Power *ptr = new();
            let mut twi = core::ptr::read_volatile(&mut self.PRR1);
            timer5 = timer5 & 0xDF;
            core::ptr::write_volatile(&mut self.PRR1, twi);
        }
    }

    pub fn disable_timer4(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut timer4 = core::ptr::read_volatile(&mut self.PRR1);
            timer4 = timer4 & 0xEF;
            core::ptr::write_volatile(&mut self.PRR1, timer4);
        }
    }

    pub fn disable_timer3(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut timer3 = core::ptr::read_volatile(&mut self.PRR1);
            timer3 = timer3 & 0xF7;
            core::ptr::write_volatile(&mut self.PRR1, timer3);
        }
    }

    pub fn disable_usart3(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut usart3 = core::ptr::read_volatile(&mut self.PRR1);
            usart3 = usart3 & 0xFB;
            core::ptr::write_volatile(&mut self.PRR1, usart3);
        }
    }

    pub fn disable_usart2(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut usart2 = core::ptr::read_volatile(&mut self.PRR1);
            usart2 = usart2 & 0xFD;
            core::ptr::write_volatile(&mut self.PRR1, usart2);
        }
    }

    pub fn disable_usart1(&mut self) {
        unsafe {
            Power *ptr = new();
            let mut usart1 = core::ptr::read_volatile(&mut self.PRR1);
            usart1 = usart1 & 0xFE;
            core::ptr::write_volatile(&mut self.PRR1, usart1);
        }
    }


    // These are the functions directly to be controlled by the user using calls in the main function
    // We will provide seperate functions for enabling "power-down" and "power-up" features

    // This is the function for disabling the clock system of your choice
    pub fn disable_clockgates(&mut self,mode: Options) {
        // Creates a new instance to the power structure
        Power *power = new();
        match mode {
            Options::TWI => power.enable_twi(),
            Options::TIMER2 => power.enable_timer2(),
            Options::TIMER0 => power.enable_timer0(),
            Options::TIMER1 => power.enable_timer1(),
            Options::SPI => power.enable_spi(),
            Options::USART0 => power.enable_usart0(),
            Options::ADC => power.enable_adc(),
            Options::TIMER5 => power.enable_timer5(),
            Options::TIMER4 => power.enable_timer4(),
            Options::TIMER3 => power.enable_timer3(),
            Options::USART3 => power.enable_usart3(),
            Options::USART2 => power.enable_usart2(),
            Options::USART1 => power.enable_usart1(),
            _ => unreachable!();
        }
    }

    // This is the function for enabling back the clock system of your choice
    // User has to make sure that these functions are called after disabling functions
    // otherwise they have no effect.
    pub fn enable_clockgates(&mut self,mode: Options) {
        // Creates a new instance to the power structure
        Power *power = new();
        match mode {
            Options::TWI => power.disable_twi(),
            Options::TIMER2 => power.disable_timer2(),
            Options::TIMER0 => power.disable_timer0(),
            Options::TIMER1 => power.disable_timer1(),
            Options::SPI => power.disable_spi(),
            Options::USART0 => power.disable_usart0(),
            Options::ADC => power.disable_adc(),
            Options::TIMER5 => power.disable_timer5(),
            Options::TIMER4 => power.disable_timer4(),
            Options::TIMER3 => power.disable_timer3(),
            Options::USART3 => power.disable_usart3(),
            Options::USART2 => power.disable_usart2(),
            Options::USART1 => power.disable_usart1(),
            _ => unreachable!();
        }
    }
    
}