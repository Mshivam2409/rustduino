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
use crate::delay::{delay_ms};
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
const  TWSTA: u8 = 2;
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


impl Twi {
    pub fn new() -> &'static mut Self {
        unsafe { &mut *(0xB8 as *mut Self) }
    }
    /// # Tulika needs to write this
    pub fn wait_to_complete(&mut self) ->bool {
        true
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
        return self.wait_to_complete()
    }

    pub fn stop(&mut self) -> bool {
        unsafe {
            self.twcr.update(|x| {
                // TWCR: Disable TWI module
                x.set_bit(TWSTO, true);
                x.set_bit(TWINT, true);
                x.set_bit(TWEN, true);
            });
        }
        return self.wait_to_complete()
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
        return self.wait_to_complete()
    }

    /// * Loads the address of the slave device on SDA.
    /// * The `addr` argument passed into the function is a seven bit integer.
    pub fn set_address(&mut self, addr: u8) -> bool {
        unsafe {
            self.twdr.write(addr << 1);
            self.twcr.update(|x| {
                // TWCR: Enable TWI module
                x.set_bit(TWINT, true);
                x.set_bit(TWEN, true);
            });
        }
        return self.wait_to_complete()
    }



}
