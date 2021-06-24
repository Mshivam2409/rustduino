use core;
use core::arch::arm::__nop;

// Section 10.13 of the manual
// Also references from Section 10.12 and 10.7

#[repr(C, packed)]
pub struct Prescalar { 
    CLKPR:u8,
    // memory addresses not clear from the Manual as of now
    // padding is not correct surely
    pad_1:[char;4],
    OSCCAL:u8
}

mod interrupts;

impl Prescalar {
    pub unsafe fn new() -> &'static mut Prescalar {
        // Creates a reference to the mutable structure to control the hardware of SIM.
        &mut *(0x61 as *mut Prescalar)
    }

    pub fn enable_clock(&mut self,freq:u32) {
        unsafe {
            // Creates a new instance for the Prescalar structure.
            Prescalar *ptr = new();
            // Control of the clock Gating in ATMEGA2560P

            // First global interrupts are disabled
            let itr = interrupt::Status::new();
            itr.disable();

            // System Clock Pre-scalar register are controlled for clock gating
            
            // Write the Clock Prescaler Change Enable (CLKPCE) bit to one and all other bits in CLKPR to zero.
            // Within four cycles, write the desired value to CLKPS bits while writing a zero to CLKPCE.
            // Interrupts must be disabled when changing prescaler setting to make sure the write procedure is not interrupted.
            let mut clkpr = core::ptr::read_volatile(&mut self.CLKPR);
            core::ptr::write_volatile(&mut self.CLKPR,0x80);
            // Just for stability wait for a clock cycle
            __nop();
            // The clock division factor is set to desired value
            // Only certain values are allowed for the user
            if freq == 1 { core::ptr::write_volatile(&mut self.CLKPR,0x00); }
            else if freq == 2 { core::ptr::write_volatile(&mut self.CLKPR,0x01); }
            else if freq == 4 { core::ptr::write_volatile(&mut self.CLKPR,0x02); }
            else if freq == 8 { core::ptr::write_volatile(&mut self.CLKPR,0x03); }
            else if freq == 16 { core::ptr::write_volatile(&mut self.CLKPR,0x04); }
            else if freq == 32 { core::ptr::write_volatile(&mut self.CLKPR,0x05); }
            else if freq == 64 { core::ptr::write_volatile(&mut self.CLKPR,0x06); }
            else if freq == 128 { core::ptr::write_volatile(&mut self.CLKPR,0x07); }
            else if freq == 256 { core::ptr::write_volatile(&mut self.CLKPR,0x08); }
            else { unreachable!(); }

            // Read the power usage and sleep modes etc...
            // Try to figure out how to control each and every clock of the chip
            // Also you need to understand the concept....
            // Here there is nothing like enabling a clock gating.. we have to set the appropriate registers
            
            // Enable global interrupts
            itr.enable();
        }
    }    
}