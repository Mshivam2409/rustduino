use bit_field::BitField;
use core::ptr::{read_volatile, write_volatile};
use volatile::Volatile;

pub struct Twi {
    twbr: u8,
    twsr: u8,
    twar: u8,
    twdr: u8,
    twcr: u8,
    twamr: u8,
}

// for twcr
static TWINT: usize = 0;
static TWEA: usize = 1;
static TWSTA: usize = 2;
static TWSTO: usize = 3;
static TWWC: usize = 4;
static TWEN: usize = 5;
static TWIE: usize = 7;
static START: u8 = 0xF8;

impl Twi {
    pub fn new() -> &'static mut Self {
        unsafe { &mut *(0xB8 as *mut Self) }
    }

    pub fn init(&mut self) {
        unsafe {
            write_volatile(&mut self.twcr, 0xA4);
        }
        while !self.twcr.get_bit(TWINT) {}

        if self.twsr & START != START {
            panic!("Error");
        }
    }

    pub fn set_address(&mut self, addr: u8) {
        unsafe {
            write_volatile(&mut self.twdr, addr);
            self.twdr.set_bit(TWINT, true);
            self.twdr.set_bit(TWEN, true);
        }
        while !self.twcr.get_bit(TWINT) {}
        //what is the start condition... finding that
        // also how to know that acknowledgement is recieved.
    }
}
