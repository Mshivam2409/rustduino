use bit_field::BitField;
use core::ptr::{read_volatile};
// use core::{array, u32, u8};
use volatile::Volatile;

use crate::delay::{delay_ms};


pub struct Twi {
    twbr: Volatile<u8>,
    twsr: Volatile<u8>,
    twar: Volatile<u8>,
    twdr: Volatile<u8>,
    twcr: Volatile<u8>,
    twamr: Volatile<u8>,
}

// for twcr
static TWINT: u8 = 0;
static TWEA: u8 = 1;
static TWSTA: u8 = 2;
static TWSTO: u8 = 3;
static TWWC: u8 = 4;
static TWEN: u8 = 5;
static TWIE: u8 = 7;

// for twsr
static TWPS1:u8 = 6;
static TWPS0:u8 = 7;

pub fn prescaler() -> (u8, bool, bool) { 
    if (crate::config::CPU_FREQUENCY_HZ/TWI_FREQUENCY - 16) / (2 * 1) >= 10 
        && (crate::config::CPU_FREQUENCY_HZ/TWI_FREQUENCY - 16) / (2 * 1) <= 0xFF {
            return (1, false, false);
        }
    
    else if (crate::config::CPU_FREQUENCY_HZ/TWI_FREQUENCY - 16) / (2 * 4) >= 10 
        && (crate::config::CPU_FREQUENCY_HZ/TWI_FREQUENCY - 16) / (2 * 4) <= 0xFF {
            return (4, true, false);
        }
        
    else if (crate::config::CPU_FREQUENCY_HZ/TWI_FREQUENCY - 16) / (2 * 16) >= 10 
        && (crate::config::CPU_FREQUENCY_HZ/TWI_FREQUENCY - 16) / (2 * 16) <= 0xFF{
            return (16, false, true);
        }
        
    else if (crate::config::CPU_FREQUENCY_HZ/TWI_FREQUENCY - 16) / (2 * 64) >= 10 
        && (crate::config::CPU_FREQUENCY_HZ/TWI_FREQUENCY - 16) / (2 * 64) <= 0xFF{
            return (64, true, true);
        }
    
    else {
        panic!("TWI_FREQUENCY too low!");
        // return (-1, -1, -1);
    }
}

static PRESCALER:u8 = prescaler().0;
static TWPS0_VAL:bool = prescaler().1;
static TWPS1_VAL:bool = prescaler().2;
static TWI_FREQUENCY:u32 =  100000;

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
static I2C_TIMEOUT:u32 = 100;

// pub fn sbi(var: u8, mask: u8) {
//     ((var) |= (uint8)(1 << mask));
// }   
// pub fn cbi(var: u8, mask: u8) {
//     ((var) &= (uint8_t)!(1 << mask));
// }   

pub fn write_sda() {  
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

    pub fn wait_to_complete(&mut self, operation:u8) -> u8{
        let mut i:u32 = 0;
        while !self.twcr.read().get_bit(TWINT) || i <= I2C_TIMEOUT {  // waiting for TWINT to be set
            i += 1;
        }

        if self.twsr.read() & TWSR_STATUS_MASK != operation { //if status id ok return else panic
            if i >= I2C_TIMEOUT {
                return 1 // for timeout
            } else {
                return 2 // for other errors
            } 
        } else {
            return 0 // if everything is fine.
        }
    }

    pub fn init (&mut self) {
        unsafe {
            self.twsr.update(|sr| {
                sr.set_bit(TWPS1, TWPS1_VAL);
                sr.set_bit(TWPS0, TWPS0_VAL);
            });
            self.twcr.update(|cr| {
                cr.set_bit(TWEN, true);
            })
        }
    }

    pub fn start(&mut self) -> u8{
        write_sda();
        unsafe {
            self.twcr.write(0xA4); // TWINT TWSTA and TWA set to 1
        }
        
    //     match self.wait_to_complete(START) {
    //         0 => return,
    //         1 => panic!("Timeout"),
    //         2 => panic!("Error"),
    //     }
        return self.wait_to_complete(START)
    }

    pub fn rep_start(&mut self) -> u8 {
        unsafe {
            self.twcr.write(0xA4); // TWINT TWSTA and TWA set to 1
        }
        
        // match self.wait_to_complete(REP_START) {
        //     0 => return,
        //     1 => panic!("Timeout"),
        //     2 => panic!("Error"),
        // }
        return self.wait_to_complete(REP_START)
    }

    pub fn stop(&mut self){
        unsafe {
            self.twcr.write(0xB0);
        } 
    }


    pub fn set_address(&mut self, addr: u8) -> u8 {
        unsafe {
            self.twdr.write(addr); // loading SLA_W to TWDR
            self.twcr.update(|cr| {
                cr.set_bit(TWINT, true);
                cr.set_bit(TWEN, true);
            });
        }
        
        // match self.wait_to_complete(MT_SLA_ACK) {
        //     0 => return,
        //     1 => panic!("Timeout"),
        //     2 => panic!("Error"),
        // }
        return self.wait_to_complete(MT_SLA_ACK);
    }

    // pub fn address_read (&mut self) { //not sure iski zaroorat hai isilie likha nhi hai....

    // }


    pub fn write(&mut self, data:u8) -> u8 {
        delay_ms(1);
        // write_sda(); // doubt if neended
        unsafe {
            self.twdr.write(data);
            self.twcr.write(0x84); // TWCR = (1<<TWINT)|(1<<TWEN);
        }

        // match self.wait_to_complete(MT_DATA_ACK) {
        //     0 => return,
        //     1 => panic!("Timeout"),
        //     2 => panic!("Error"),
        // }
        return self.wait_to_complete(MT_DATA_ACK);
    }

    // how to pass variable size array in rust..??

    // pub fn write_burst (&mut self, &mut data:&mut u8, size:u8)  {
    //     let mut x = size;
    //     while x > 0 {
    //         if  self.write(data) !=0 {
    //             break;
    //         } 
    //         data+=1;
    //         x-=1;
    //     }
    // }
}

