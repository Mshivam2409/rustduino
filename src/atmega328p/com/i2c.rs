// RustDuino : A generic HAL implementation for Arduino Boards in Rust
// Copyright (C) 2021  Akshit Verma, Indian Institute of Technology Kanpur

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>



use bit_field::BitField;
use crate::delay::delay_ms;
use fixed_slice_vec::FixedSliceVec;
use volatile::Volatile;

pub struct Twi {
    twbr: Volatile<u8>,
    twsr: Volatile<u8>,
    twar: Volatile<u8>,
    twdr: Volatile<u8>,
    twcr: Volatile<u8>,
    twamr: Volatile<u8>,
}

// for twcr
const TWINT: u8 = 0;
const TWEA: u8 = 1;
const TWSTA: u8 = 2;
const TWSTO: u8 = 3;
const TWWC: u8 = 4;
const TWEN: u8 = 5;
const TWIE: u8 = 7;

// for twsr
const TWPS1: u8 = 6;
const TWPS0: u8 = 7;

static TWI_FREQUENCY: u32 = 100000;

pub fn prescaler() -> (u8, bool, bool) {
    if (crate::config::CPU_FREQUENCY_HZ / TWI_FREQUENCY - 16) / (2 * 1) >= 10
        && (crate::config::CPU_FREQUENCY_HZ / TWI_FREQUENCY - 16) / (2 * 1) <= 0xFF
    {
        return (1, false, false);
    } else if (crate::config::CPU_FREQUENCY_HZ / TWI_FREQUENCY - 16) / (2 * 4) >= 10
        && (crate::config::CPU_FREQUENCY_HZ / TWI_FREQUENCY - 16) / (2 * 4) <= 0xFF
    {
        return (4, true, false);
    } else if (crate::config::CPU_FREQUENCY_HZ / TWI_FREQUENCY - 16) / (2 * 16) >= 10
        && (crate::config::CPU_FREQUENCY_HZ / TWI_FREQUENCY - 16) / (2 * 16) <= 0xFF
    {
        return (16, false, true);
    } else if (crate::config::CPU_FREQUENCY_HZ / TWI_FREQUENCY - 16) / (2 * 64) >= 10
        && (crate::config::CPU_FREQUENCY_HZ / TWI_FREQUENCY - 16) / (2 * 64) <= 0xFF
    {
        return (64, true, true);
    } else {
        panic!("TWI_FREQUENCY too low!");
        // return (-1, -1, -1);
    }
}
// doss not work this way.. call value in the required function.
// static PRESCALER:u8 = prescaler().0;
// static TWPS0_VAL:bool = prescaler().1;
// static TWPS1_VAL:bool = prescaler().2;

// TWSR values (not bits)
// (taken from avr-libc twi.h)
// Master
const START: u8 = 0x08;
const REP_START: u8 = 0x10;
// Master Transmitter;
const MT_SLA_ACK: u8 = 0x18;
const MT_SLA_NACK: u8 = 0x20;
const MT_DATA_ACK: u8 = 0x28;
const MT_DATA_NACK: u8 = 0x30;
const MT_ARB_LOST: u8 = 0x38;
// Master Receiver;
const MR_ARB_LOST: u8 = 0x38;
const MR_SLA_ACK: u8 = 0x40;
const MR_SLA_NACK: u8 = 0x48;
const MR_DATA_ACK: u8 = 0x50;
const MR_DATA_NACK: u8 = 0x58;
// Slave Transmitter;
const ST_SLA_ACK: u8 = 0xA8;
const ST_ARB_LOST_SLA_ACK: u8 = 0xB0;
const ST_DATA_ACK: u8 = 0xB8;
const ST_DATA_NACK: u8 = 0xC0;
const ST_LAST_DATA: u8 = 0xC8;
// Slave Receiver;
const SR_SLA_ACK: u8 = 0x60;
const SR_ARB_LOST_SLA_ACK: u8 = 0x68;
const SR_GCALL_ACK: u8 = 0x70;
const SR_ARB_LOST_GCALL_ACK: u8 = 0x78;
const SR_DATA_ACK: u8 = 0x80;
const SR_DATA_NACK: u8 = 0x88;
const SR_GCALL_DATA_ACK: u8 = 0x90;
const SR_GCALL_DATA_NACK: u8 = 0x98;
const SR_STOP: u8 = 0xA0;
// Misc
const NO_INFO: u8 = 0xF8;
const BUS_ERROR: u8 = 0x00;

// defines and constants;
const TWCR_CMD_MASK: u8 = 0x0F;
const TWSR_STATUS_MASK: u8 = 0xF8;

// return values;
const I2C_OK: u8 = 0x00;
const I2C_ERROR_NODEV: u8 = 0x01;
const I2C_TIMEOUT: u32 = 100;

pub fn write_sda() {
    unsafe {
        Volatile::new(*(0x27 as *mut u8)).update(|ddrd| {
            ddrd.set_bit(1, true);
        });
    }
}

pub fn read_sda() {
    unsafe {
        Volatile::new(*(0x27 as *mut u8)).update(|ddrd| {
            ddrd.set_bit(1, false);
        });
    }
}

impl Twi {
    pub fn new() -> &'static mut Self {
        unsafe { &mut *(0xB8 as *mut Self) }
    }

    pub fn wait_to_complete(&mut self, operation: u8) -> bool {
        let mut i: u32 = 0;
        while !self.twcr.read().get_bit(TWINT) || i <= I2C_TIMEOUT {
            // waiting for TWINT to be set
            unsafe {
                llvm_asm!("nop");
            }
            i += 1;
        }

        if self.twsr.read() & TWSR_STATUS_MASK != operation || i >= I2C_TIMEOUT {
            //if status id ok return else panic
            return false; // for timeout... ignoring other error case...
        } else {
            return true; // if everything is fine.
        }
    }

    pub fn init(&mut self) {
        unsafe {
            self.twsr.update(|sr| {
                sr.set_bit(TWPS0, prescaler().1);
                sr.set_bit(TWPS1, prescaler().2);
            });
            self.twcr.update(|cr| {
                cr.set_bit(TWEN, true);
            })
        }
    }

    pub fn start(&mut self) -> bool {
        write_sda();
        unsafe {
            self.twcr.write(0xA4); // TWINT TWSTA and TWA set to 1
        }

        return self.wait_to_complete(START);
    }

    pub fn rep_start(&mut self) -> bool {
        unsafe {
            self.twcr.write(0xA4); // TWINT TWSTA and TWA set to 1
        }

        return self.wait_to_complete(REP_START);
    }

    pub fn stop(&mut self) {
        unsafe {
            self.twcr.write(0xB0);
        }
    }

    pub fn set_address(&mut self, addr: u8) -> bool {
        unsafe {
            self.twdr.write(addr << 1 & !0x01); // loading SLA_W to TWDR
            self.twcr.update(|cr| {
                cr.set_bit(TWINT, true);
                cr.set_bit(TWEN, true);
            });
        }

        return self.wait_to_complete(MT_SLA_ACK);
    }

    pub fn address_read(&mut self, address: u8) -> bool {
        unsafe {
            self.twdr.write(address << 1 | 0x01);
            self.twcr.update(|cr| {
                cr.set_bit(TWINT, true);
                cr.set_bit(TWEN, true);
            });

            return self.wait_to_complete(MR_SLA_ACK);
        }
    }

    pub fn write(&mut self, data: u8) -> bool {
        delay_ms(1);
        // write_sda(); // doubt if neended
        unsafe {
            self.twdr.write(data);
            self.twcr.write(0x84); // TWCR = (1<<TWINT)|(1<<TWEN);
        }

        return self.wait_to_complete(MT_DATA_ACK);
    }

    // returns number of elements not written
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

    pub fn read_ack(&mut self, data: &mut FixedSliceVec<u8>) -> bool {
        unsafe {
            self.twcr.write(0xC4); //TWCR = (1 << TWINT) | (1 << TWEA) | (1 << TWEN)
        }
        data.push(self.twdr.read());
        return self.wait_to_complete(MR_DATA_ACK);
    }

    pub fn read_ack_burst(&mut self, data: &mut FixedSliceVec<u8>, length: usize) -> usize {
        let mut x: usize = 0;
        // let mut vec:FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        while x < length {
            if !self.read_ack(data) {
                break;
            }
            x += 1;
        }
        return x + 1;
    }

    pub fn read_nack(&mut self, data: &mut FixedSliceVec<u8>) -> bool {
        unsafe {
            self.twcr.write(0x84); //TWCR = (1 << TWINT) | (1 << TWEN)
        }
        data.push(self.twdr.read());
        return self.wait_to_complete(MR_DATA_NACK);
    }

    pub fn read_nack_burst(&mut self, data: &mut FixedSliceVec<u8>, length: usize) -> usize {
        let mut x: usize = 0;
        // let mut vec:FixedSliceVec<u8> = FixedSliceVec::new(&mut []);

        while x < length {
            if !self.read_nack(data) {
                break;
            }
            x += 1;
        }
        return x + 1;
    }

    pub fn write_to_slave(&mut self, address: u8, data: &FixedSliceVec<u8>) -> bool {
        delay_ms(1);
        write_sda();

        if !self.start() {
            return false;
        }
        if !self.set_address(address) {
            self.stop();
            return false;
        }
        if self.write_burst(data) != data.len() {
            self.stop();
            return false;
        }

        self.stop();

        return true;
    }

    pub fn read_from_slave(
        &mut self,
        address: u8,
        length: usize,
        data: &mut FixedSliceVec<u8>,
    ) -> bool {
        delay_ms(1);
        read_sda();

        // let mut vec:FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        if !self.start() {
            return false;
        }
        if !self.address_read(address) {
            self.stop();
            return false;
        }
        if length > 1 && self.read_ack_burst(data, length - 1) != length - 1 {
            self.stop();
            return false;
        }
        if length > 0 && self.read_nack(data) {
            self.stop();
            return false;
        }

        self.stop();

        return true;
    }
}
