//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Prateek Kumar Pandey, Indian Institute of Technology Kanpur
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU Affero General Public License as published
//     by the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU Affero General Public License for more details.
//
//     You should have received a copy of the GNU Affero General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>



/// Crates which would be used in the implementation.
use bit_field::BitField;
use core::ptr::{read_volatile};
// use core::{array, u32, u8};
use volatile::Volatile;
use crate::delay::delay_ms;
use fixed_slice_vec::FixedSliceVec;
// use core::mem::MaybeUninit;

/// ## TWI registers definitions
pub struct Twi {
    twbr: Volatile<u8>,
    twcr: Volatile<u8>,
    twsr: Volatile<u8>,
    twdr: Volatile<u8>,
    twar: Volatile<u8>,
    twamr: Volatile<u8>,
}

/// ## TWCR register's bits definitions
const TWINT: u8 = 0;
const TWEA: u8 = 1;
const TWSTA: u8 = 2;
const TWSTO: u8 = 3;
const TWWC: u8 = 4;
const TWEN: u8 = 5;
const TWIE: u8 = 7;

/// ## TWCR register's bits definitions
const TWPS1: u8 = 6;
const TWPS0: u8 = 7;

// TWSR status codes
// Master
const START: u8 = 0x08;
const REP_START: u8 = 0x10;

// Master Transmitter
const MT_SLA_ACK: u8 = 0x18;
const MT_SLA_NACK: u8 = 0x20;
const MT_DATA_ACK: u8 = 0x28;
const MT_DATA_NACK: u8 = 0x30;
const MT_ARB_LOST: u8 = 0x38;

// Master Receiver
const MR_ARB_LOST: u8 = 0x38;
const MR_SLA_ACK: u8 = 0x40;
const MR_SLA_NACK: u8 = 0x48;
const MR_DATA_ACK: u8 = 0x50;
const MR_DATA_NACK: u8 = 0x58;

// Slave Transmitter
const ST_SLA_ACK: u8 = 0xA8;
const ST_ARB_LOST_SLA_ACK: u8 = 0xB0;
const ST_DATA_ACK: u8 = 0xB8;
const ST_DATA_NACK: u8 = 0xC0;
const ST_LAST_DATA: u8 = 0xC8;

// Slave Receiver
const SR_SLA_ACK:u8 =0x60;
const SR_ARB_LOST_SLA_ACK: u8 = 0x68;
const SR_GCALL_ACK: u8 = 0x70;
const SR_ARB_LOST_GCALL_ACK: u8 = 0x78;
const SR_DATA_ACK: u8 = 0x80;
const SR_DATA_NACK: u8 = 0x88;
const SR_GCALL_DATA_ACK: u8 = 0x90;
const SR_GCALL_DATA_NACK: u8 = 0x98;
const SR_STOP: u8 = 0xA0;

// Miscellaneous
const NO_INFO: u8 = 0xF8;
const BUS_ERROR: u8 = 0x00;

// Defines and constants
const TWCR_CMD_MASK: u8 = 0x0F;
const TWSR_STATUS_MASK: u8 = 0xF8;

//return values
const I2C_OK: u8 = 0x00;
const I2C_ERROR_NODEV: u8 = 0x01;

pub fn write_sda(){
    unsafe{
    let port_d=&mut*(0x2A as *mut u8);
    let mut ddrd= read_volatile(port_d);
    ddrd.set_bit(1,true);
    }
}

pub fn read_sda(){
    unsafe{
        let port_d=&mut*(0x2A as *mut u8);
        let mut ddrd=read_volatile(port_d);
        ddrd.set_bit(1,false);
    }
}

impl Twi {
    pub fn new() -> &'static mut Self {
        unsafe { &mut *(0xB8 as *mut Self) }
    }
    /// 
    pub fn wait_to_complete(&mut self,start:u8) ->bool {
        let mut i:u32=0;
        //Waiting for TWINT flag set.
        //This indicates that start condition has been transmitted.
        while !self.twcr.read().get_bit(TWINT){
            unsafe{
                llvm_asm!("nop");
            }
            i+=1;
        }
        // if TWSR_STATUS_MASK is different from start, error.
        if self.twsr.read() & TWSR_STATUS_MASK!=start{
            return false;
        }
        else{
            return true;
        }
    }

    pub fn init(&mut self) {
        unsafe {
            self.twbr.update(|x| {
                x.set_bit(TWEN, true);
            });
        }
    }

    pub fn start(&mut self) -> bool {
        unsafe {
            self.twcr.update(|x| {
                // TWCR: Enable TWI module
                x.set_bit(TWSTA, true);
                x.set_bit(TWINT, true);
                x.set_bit(TWEN, true);
            });
        }
        return self.wait_to_complete(START);
    }

    pub fn stop(&mut self) {
        unsafe {
            self.twcr.update(|x| {
                // TWCR: Disable TWI module
                x.set_bit(TWSTO, true);
                x.set_bit(TWINT, true);
                x.set_bit(TWEN, true);
            });
        }
    }

    pub fn rep_start(&mut self) -> bool {
        unsafe {
            self.twcr.update(|x| {
                // TWCR: Enable TWI module
                x.set_bit(TWSTA, true);
                x.set_bit(TWINT, true);
                x.set_bit(TWEN, true);
            });
        }
        return self.wait_to_complete(REP_START);
    }

    /// * Loads the address of the slave device on SDA.
    /// * The `address` argument passed into the function is a seven bit integer.
    pub fn address_write(&mut self, address: u8) -> bool {
        unsafe {
            self.twdr.write(address << 1);
            self.twcr.update(|x| {
                // TWCR: Enables TWI to pass address 
                x.set_bit(TWINT, true);
                x.set_bit(TWEN, true);
            });
        }
        return self.wait_to_complete(MT_SLA_ACK);
    }

    /// * Loads the address of the slave device on SDA.
    /// * The `address` argument passed into the function is a seven bit integer.
    pub fn address_read(&mut self,address:u8)->bool{
        self.twdr.write(address<<1 | 0x01);
        self.twcr.update(|x|{
               x.set_bit(TWINT,true);
               x.set_bit(TWEN,true);
        });
        return self.wait_to_complete(MR_SLA_ACK);
    }

    pub fn read_ack(&mut self,data: &mut FixedSliceVec<u8>)-> bool{
        self.twcr.update(|x|{
            x.set_bit(TWINT,true);
            x.set_bit(TWEA,true);
            x.set_bit(TWEN,true);
        });
        data.push(self.twdr.read());
        return self.wait_to_complete(MR_DATA_ACK);
    }

    pub fn read_ack_burst(&mut self, data: &mut FixedSliceVec<u8>,length:usize)->usize{
        let mut x:usize=0;
        while x<length{
            if !self.read_ack(data){
                break;
            }
            x+=1;
        }
        return x+1;
    }

    pub fn write(&mut self, data:u8) -> bool{
        delay_ms(1);
        unsafe {
            self.twdr.write(data);
            self.twcr.update(|x| {
                // TWCR: Enables TWI module to pass data to slave.
                x.set_bit(TWINT, true);
                x.set_bit(TWEN, true);
            });
        }
        return self.wait_to_complete(MT_DATA_ACK);
    }

    pub fn write_burst(&mut self, data: &FixedSliceVec<u8>) -> usize {
        let mut x: usize = 0;
        while x < data.len() {
            if !self.write(data[x]) {
                break;
            }
            x += 1;
        }
        return x + 1;
    }
    pub fn read_from_slave(&mut self, address:u8, length:usize, data:&mut FixedSliceVec<u8>) -> bool {
        delay_ms(1);
        if !self.start(){
            return false;
        }
        if !self.address_read (address){
            return false;
        }

        self.stop();
        return true;
    }
    
    pub fn write_to_slave(&mut self, address: u8, data: &FixedSliceVec<u8>)-> bool {
        delay_ms(1);
        if !self.start(){
            return false;
        }
        if !self.address_write(address){
            return false;
        }

        if self.write_burst(data) != data.len() {
            self.stop();
            return false;
        }
        self.stop();
        return true;
    }
}
