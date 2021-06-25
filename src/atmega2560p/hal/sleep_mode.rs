use core;
use core::arch::arm::__nop;

// Section 11.10.1 of the manual
// Also references from Section 11.4

// Various modes are
    // IDLE => Idle sleep mode          
    // ADC => ADC Noise Reduction
    // PD => Power-down    
    // PS => Power-save
    // SBY => Standby      
    // ESBY => Extended Standby

#[derive(Clone,Copy)]
pub enum Options {
    IDLE,   ADC,
    PD,     PS,
    SBY,    ESBY
}


#[repr(C, packed)]
pub struct Sleep { 
    SMCR:u8,
}

// mod interrupts;

impl Sleep {
    pub unsafe fn new() -> &'static *mut Sleep {
        // Creates a new reference to the Sleep structure 
        &mut *(0x53 as *mut Sleep)
    }

    pub fn enable(&mut self) {
        unsafe {
            // Create a instance of sleep to work upon
            Sleep *ptr = new();
            
            // The SE bit must be written to logic one to make the MCU enter the sleep mode when the SLEEP instruction is executed.
            // To avoid the MCU entering the sleep mode unless it is the programmer’s purpose, it is recommended to
            // write the Sleep Enable (SE) bit to one just before the execution of the SLEEP instruction and to clear it immediately
            // after waking up.

            // Set the last bit of SMCR register as 1 for enabling the sleep mode.
            let mut smcr = core::ptr::read_volatile(&mut self.SMCR);
            smcr = smcr | 0x01;
            core::ptr::read_volatile(&mut self.SMCR, smcr);
        }
    }

    pub fn disable(&mut self) {
        unsafe {
            // Create a instance of sleep to work upon
            Sleep *ptr = new();

            // Set the last bit of SMCR register as 0 for disabling the sleep mode.
            let mut smcr = core::ptr::read_volatile(&mut self.SMCR);
            smcr = smcr & 0xFE;
            core::ptr::read_volatile(&mut self.SMCR, smcr);
        }
    }

    pub fn select_mode(&mut self,mode:Options) {
        unsafe {
            // Create a instance of sleep to work upon
            Sleep *ptr = new();
            // Enable the Sleep mode
            ptr.enable();

            // Set the bits of SMCR register according to the sleep mode required.
            // The sleep mode to be set will be given as the standard name in the manual
            // For more info see the implementation in main.rs

            let mut smcr = core::ptr::read_volatile(&mut self.SMCR);
            smcr = 0x0F;
            
            match mode {
                Options::IDLE => smcr = smcr & 0xF1,
                Options::ADC  => smcr = smcr & 0xF3,
                Options::PD   => smcr = smcr & 0xF5,
                Options::PS   => smcr = smcr & 0xF7,
                Options::SBY  => smcr = smcr & 0xFD,
                Options::ESBY => smcr = smcr & 0xFF,
                _ => unreachable!()
            }

            core::ptr::read_volatile(&mut self.SMCR, smcr);
        }
    }
}
