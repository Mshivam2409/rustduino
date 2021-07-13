use bit_field::BitField;
use core::ptr::{read_volatile, write_volatile};
use core::{u32, u8};
// use volatile::Volatile;

// use crate::atmega328p::hal::port;
use crate::delay::{delay_ms};


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
// static STATUS_MASK: u8 = 0xF8;


// TWSR values (not bits)
// (taken from avr-libc twi.h)
// Master
static START:u8 =					0x08;
static REP_START:u8 =				0x10;
// Master Transmitter;
static MT_SLA_ACK:u8 =				0x18;
static MT_SLA_NACK:u8 =				0x20;
static MT_DATA_ACK:u8 =				0x28;
static MT_DATA_NACK:u8 =			0x30;
static MT_ARB_LOST:u8 =				0x38;
// Master Receiver;
static MR_ARB_LOST:u8 =				0x38;
static MR_SLA_ACK:u8 =				0x40;
static MR_SLA_NACK:u8 =				0x48;
static MR_DATA_ACK:u8 =				0x50;
static MR_DATA_NACK:u8 =			0x58;
// Slave Transmitter;
static ST_SLA_ACK:u8 =				0xA8;
static ST_ARB_LOST_SLA_ACK:u8 =		0xB0;
static ST_DATA_ACK:u8 =				0xB8;
static ST_DATA_NACK:u8 =			0xC0;
static ST_LAST_DATA:u8 =			0xC8;
// Slave Receiver;
static SR_SLA_ACK:u8 =				0x60;
static SR_ARB_LOST_SLA_ACK:u8 =		0x68;
static SR_GCALL_ACK:u8 =			0x70;
static SR_ARB_LOST_GCALL_ACK:u8 =	0x78;
static SR_DATA_ACK:u8 =				0x80;
static SR_DATA_NACK:u8 =			0x88;
static SR_GCALL_DATA_ACK:u8 =		0x90;
static SR_GCALL_DATA_NACK:u8 =		0x98;
static SR_STOP:u8 =					0xA0;
// Misc
static NO_INFO:u8 =					0xF8;
static BUS_ERROR:u8 =				0x00;

// defines and constants;
static TWCR_CMD_MASK: u8 =          0x0F;
static TWSR_STATUS_MASK: u8 =       0xF8;

// return values;
static I2C_OK:u8 = 0x00;
static I2C_ERROR_NODEV: u8 = 0x01;

// pub fn sbi(var: u8, mask: u8) {
//     ((var) |= (uint8)(1 << mask));
// }   
// pub fn cbi(var: u8, mask: u8) {
//     ((var) &= (uint8_t)!(1 << mask));
// }   

pub fn write_sda() {
    // let mut portc = Port::new(port::PortName::C);  
    unsafe {
        let portc = &mut *(0x27 as *mut u8); 
        let mut ddrc = read_volatile(portc);
        ddrc.set_bit(3, true); //SDA must be output when writing
    }
} 

pub fn read_sda() {
    unsafe {
        let portc = &mut *(0x27 as *mut u8);
        let mut ddrc = read_volatile(portc);
        ddrc.set_bit(3, false); //SDA must be input when reading - don't forget the resistor on SDA!!
    }
}  


impl Twi {
    pub fn new() -> &'static mut Self {
        unsafe { &mut *(0xB8 as *mut Self) }
    }

    pub fn init(&mut self) {
        let mut i:u32 = 0;
        write_sda();
        unsafe {
            write_volatile(&mut self.twcr, 0xA4); // TWINT TWSTA and TWA set to 1
        }
        while !self.twcr.get_bit(TWINT) || i <= 100 {  // waiting for TWINT to be set
            i += 1;
        }

        if self.twsr & TWSR_STATUS_MASK != START  { //if status id ok return else panic
            if i >= 100 {
                panic!("Timeout");
            } else {
                panic!("Error");
            } 
        }
    }

    pub fn set_address(&mut self, addr: u8) {
        let mut i:u32 = 0;
        unsafe {
            write_volatile(&mut self.twdr, addr); // loading SLA_W to TWDR
            self.twdr.set_bit(TWINT, true); 
            self.twdr.set_bit(TWEN, true);
        }
        while !self.twcr.get_bit(TWINT) || i <= 100 { //waiting for TWINT to be set
            i += 1;
        } 
        
        if self.twsr & TWSR_STATUS_MASK != MT_SLA_ACK { // if ACK recieved.. ok.. else panic
            if i >= 100 {
                panic!("Timeout");
            } else {
                panic!("Error");
            }
        }
    }


    pub fn send_bytes(&mut self, data:u8) {
        let mut i = 0;
        delay_ms(1);
        write_sda();
        self.twdr = data;
        unsafe {
            write_volatile(&mut self.twcr, 0x84); // TWCR = (1<<TWINT)|(1<<TWEN);
        }

        while !self.twcr.get_bit(TWINT) || i <= 100 {  // waiting for TWINT to be set
            i += 1;
        }

        if self.twsr & TWSR_STATUS_MASK != MT_DATA_ACK  { //if status id ok return else panic
            if i >= 100 {
                panic!("Timeout");
            } else {
                panic!("Error");
            } 
        }
    }


    pub fn read_from(&mut self,data:u8){
        unsafe {
            write_volatile(&mut self.twdr,data);
            self.twdr.set_bit(TWINT, true);
            self.twdr.set_bit(TWEN, true);
        }
        while !self.twcr.get_bit(TWINT) {}
    }

    
    pub fn stop(&mut self){
        unsafe {
            write_volatile(&mut self.twcr, 0xB0);
        } 
    }


}
