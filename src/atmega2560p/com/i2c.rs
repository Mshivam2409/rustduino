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

//!* This source code contains the functions to control the I2C communication protocol for ATMEGA2560P AVR Microcontroller.
//!  The elements of I2C implementation are first enabling the I2C Two Wire Interface(TWI) to work on
//!  the data bus and then using bits manipulation to exchange data and communicate
//!  with the attached peripheral devices.
//!* This has been implemented according to the chip ATMEGA2560P here.

use crate::delay::delay_ms;
use bit_field::BitField;
use core::ptr::read_volatile;
use fixed_slice_vec::FixedSliceVec;
use volatile::Volatile;

/// It will be used to control the I2C Twi
/// implementations with the data registers assigned to
/// it according to the data sheet.
#[repr(C, packed)]
pub struct Twi {
    _twbr: Volatile<u8>,
    twcr: Volatile<u8>,
    twsr: Volatile<u8>,
    twdr: Volatile<u8>,
    _twar: Volatile<u8>,
    _twamr: Volatile<u8>,
}

// TWCR register's bits definitions
const TWINT: u8 = 0;
const TWEA: u8 = 1;
const TWSTA: u8 = 2;
const TWSTO: u8 = 3;
const _TWWC: u8 = 4;
const TWEN: u8 = 5;
const _TWIE: u8 = 7;

static TWI_FREQUENCY: u32 = 100000;

///* This function reads the device clock freequency setup and provide
///  the details in form of boolean numbers and a 8 bit unsigned integer to
///  check the settings of the I2C carefully.
///* If the clock freequency in very low then the function panics as the I2C protocol cannot be activated
///  properly at such low freequencies.
///  # Returns
///  * `a tuple` - Consisting of the following 3 Items -
///     * `a u8` - Which is a 2's exponent till 64 which defines the bandwidth rate for TWI I2C initialization.
///     * `a boolean` - Which denotes the TWPS bit 1 settings.
///     * `a boolean` - Which denotes the TWPS bit 2 settings.
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
    }
}

// TWCR register's bits definitions
const TWPS1: u8 = 6;
const TWPS0: u8 = 7;

// TWSR status codes
// Master
// (taken from avr-libc twi.h)
const START: u8 = 0x08;
const REP_START: u8 = 0x10;

// Master Transmitter
const MT_SLA_ACK: u8 = 0x18;
const _MT_SLA_NACK: u8 = 0x20;
const MT_DATA_ACK: u8 = 0x28;
const _MT_DATA_NACK: u8 = 0x30;
const _MT_ARB_LOST: u8 = 0x38;

// Master Receiver
const _MR_ARB_LOST: u8 = 0x38;
const MR_SLA_ACK: u8 = 0x40;
const _MR_SLA_NACK: u8 = 0x48;
const MR_DATA_ACK: u8 = 0x50;
const MR_DATA_NACK: u8 = 0x58;

// Slave Transmitter
const _ST_SLA_ACK: u8 = 0xA8;
const _ST_ARB_LOST_SLA_ACK: u8 = 0xB0;
const _ST_DATA_ACK: u8 = 0xB8;
const _ST_DATA_NACK: u8 = 0xC0;
const _ST_LAST_DATA: u8 = 0xC8;

// Slave Receiver
const _SR_SLA_ACK: u8 = 0x60;
const _SR_ARB_LOST_SLA_ACK: u8 = 0x68;
const _SR_GCALL_ACK: u8 = 0x70;
const _SR_ARB_LOST_GCALL_ACK: u8 = 0x78;
const _SR_DATA_ACK: u8 = 0x80;
const _SR_DATA_NACK: u8 = 0x88;
const _SR_GCALL_DATA_ACK: u8 = 0x90;
const _SR_GCALL_DATA_NACK: u8 = 0x98;
const _SR_STOP: u8 = 0xA0;

// Miscellaneous
const _NO_INFO: u8 = 0xF8;
const _BUS_ERROR: u8 = 0x00;

// Defines and constants
const _TWCR_CMD_MASK: u8 = 0x0F;
const TWSR_STATUS_MASK: u8 = 0xF8;

// return values
const _I2C_OK: u8 = 0x00;
const _I2C_ERROR_NODEV: u8 = 0x01;
const I2C_TIMEOUT: u32 = 100;

/// Sets DDRC to write direction.
pub fn write_sda() {
    unsafe {
        let port_d = &mut *(0x2A as *mut u8);
        let mut ddrd = read_volatile(port_d);
        ddrd.set_bit(1, true);
    }
}

/// Sets DDRC to read direction.
pub fn read_sda() {
    unsafe {
        let port_d = &mut *(0x2A as *mut u8);
        let mut ddrd = read_volatile(port_d);
        ddrd.set_bit(1, false);
    }
}

impl Twi {
    /// Creates a pointer to TWI structure objects.
    /// # Returns
    /// * `a reference to Twi struct object` - Which would be used to control the implementation.
    pub fn new() -> &'static mut Self {
        unsafe { &mut *(0xB8 as *mut Self) }
    }

    /// Waits for the TWI bus to be ready.
    /// # Returns
    /// * `a boolean` - Which is true if the TWI is ready, false otherwise.
    pub fn wait_to_complete(&mut self, start: u8) -> bool {
        let mut i: u32 = 0;
        //Waiting for TWINT flag set.
        //This indicates that start condition has been transmitted.
        while !self.twcr.read().get_bit(TWINT) || i <= I2C_TIMEOUT {
            unsafe {
                llvm_asm!("nop");
            }
            i += 1;
        }
        // if TWSR_STATUS_MASK is different from start, error.
        if self.twsr.read() & TWSR_STATUS_MASK != start || i >= I2C_TIMEOUT {
            return false;
        } else {
            return true;
        }
    }

    /// Initiates the TWI Bus.
    pub fn init(&mut self) {
        self.twsr.update(|sr| {
            sr.set_bit(TWPS0, prescaler().1);
            sr.set_bit(TWPS1, prescaler().2);
        });
        self.twcr.update(|cr| {
            cr.set_bit(TWEN, true);
        })
    }

    /// Sends a Start Signal for TWI.
    /// # Returns
    /// * `a boolean` - Which is true if process is successful, false otherwise.
    pub fn start(&mut self) -> bool {
        write_sda();
        self.twcr.update(|x| {
            // TWCR: Enable TWI module
            x.set_bit(TWSTA, true);
            x.set_bit(TWINT, true);
            x.set_bit(TWEN, true);
        });
        return self.wait_to_complete(START);
    }

    /// Stops the TWI Bus.
    pub fn stop(&mut self) {
        self.twcr.update(|x| {
            // TWCR: Disable TWI module
            x.set_bit(TWSTO, true);
            x.set_bit(TWINT, true);
            x.set_bit(TWEN, true);
        });
    }

    /// Sends the Repeated Start Signal.
    /// # Returns
    /// * `a boolean` - Which is true if process is successful, false otherwise.
    pub fn rep_start(&mut self) -> bool {
        self.twcr.update(|x| {
            // TWCR: Enable TWI module
            x.set_bit(TWSTA, true);
            x.set_bit(TWINT, true);
            x.set_bit(TWEN, true);
        });
        return self.wait_to_complete(REP_START);
    }

    /// Loads the address of the slave device on SDA.
    /// # Arguments
    /// * `address` - It is passed into the function and  is a seven bit integer used for location of implementation.
    /// # Returns
    /// * `a boolean` - Which is true if the checking process is sucessful otherwise false.
    pub fn address_write(&mut self, address: u8) -> bool {
        self.twdr.write(address << 1);
        self.twcr.update(|x| {
            // TWCR: Enables TWI to pass address
            x.set_bit(TWINT, true);
            x.set_bit(TWEN, true);
        });
        return self.wait_to_complete(MT_SLA_ACK);
    }

    /// Loads the address of the slave device from SDA for other implementations.
    /// # Arguments
    /// * `address` - It is passed into the function and  is a seven bit integer used for location of implementation.
    /// # Returns
    /// * `a boolean` - Which is true if the checking process is sucessful otherwise false.
    pub fn address_read(&mut self, address: u8) -> bool {
        self.twdr.write(address << 1 | 0x01);
        self.twcr.update(|x| {
            x.set_bit(TWINT, true);
            x.set_bit(TWEN, true);
        });
        return self.wait_to_complete(MR_SLA_ACK);
    }

    /// Appends the value in TWCR to the given vector.
    /// # Arguments
    /// * `data` - a sliced vector consisting of u8, which will be filled with the data read.
    /// # Returns
    /// * `a boolean` - Which is true if process is completed otherwise false.
    pub fn read_ack(&mut self, data: &mut FixedSliceVec<u8>) -> bool {
        self.twcr.update(|x| {
            x.set_bit(TWINT, true);
            x.set_bit(TWEA, true);
            x.set_bit(TWEN, true);
        });
        data.push(self.twdr.read());
        return self.wait_to_complete(MR_DATA_ACK);
    }

    /// Appends the value in TWCR to the given vector.
    /// # Arguments
    /// * `data` - a sliced vector consisting of u8, which is filled with the data read.
    /// * `length` - a usize integer, which is the theoretically set value of length of the sliced vector `data`.
    /// # Returns
    /// * `a usize integer` - Which gives the critical length of the data bus above which no data exists.
    pub fn read_ack_burst(&mut self, data: &mut FixedSliceVec<u8>, length: usize) -> usize {
        let mut x: usize = 0;
        while x < length {
            if !self.read_ack(data) {
                break;
            }
            x += 1;
        }
        return x + 1;
    }

    /// Writes one byte of data to the Slave.
    /// # Arguments
    /// * `data` - a u8, the integer which is to be written.
    /// # Returns
    /// * `a boolean` - Whcih is true if process is successful otherwise false.
    pub fn write(&mut self, data: u8) -> bool {
        delay_ms(1);
        self.twdr.write(data);
        self.twcr.update(|x| {
            // TWCR: Enables TWI module to pass data to slave.
            x.set_bit(TWINT, true);
            x.set_bit(TWEN, true);
        });
        return self.wait_to_complete(MT_DATA_ACK);
    }

    /// Writes continuous bytes on the TWCR.
    /// # Arguments
    /// * `data` - a sliced vector consisting of u8, which is filled with the data read.
    /// # Returns
    /// * `a usize integer` - Which gives the critical length of the data bus above which no data could be written.
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

    /// Reads the nack value data in TWCR to the given vector.
    /// # Arguments
    /// * `data` - a sliced vector consisting of u8, which will be filled with the data read.
    /// # Returns
    /// * `a boolean` - Which is true if process is completed otherwise false.
    pub fn read_nack(&mut self, data: &mut FixedSliceVec<u8>) -> bool {
        self.twcr.update(|x| {
            x.set_bit(TWINT, true);
            x.set_bit(TWEN, true);
        });
        data.push(self.twdr.read());
        return self.wait_to_complete(MR_DATA_NACK);
    }

    /// Reads the nack value data in TWCR to the given vector.
    /// # Arguments
    /// * `data` - a sliced vector consisting of u8, which is filled with the data read.
    /// * `length` - a usize integer, which is the theoretically set value of length of the sliced vector `data`.
    /// # Returns
    /// * `a usize integer` - Which gives the critical length of the data bus above which no data exists.
    pub fn read_nack_burst(&mut self, data: &mut FixedSliceVec<u8>, length: usize) -> usize {
        let mut x: usize = 0;

        while x < length {
            if !self.read_nack(data) {
                break;
            }
            x += 1;
        }
        return x + 1;
    }

    /// Reads consecutive Data bytes from slave.
    /// Sends a stop signal if either of the steps fail or reading is successful.
    /// # Arguments
    /// * `address` - a u8, consisting the target address of the read implementation.
    /// * `length` - a usize integer, showing the number of bytes to read.
    /// * `data` - a sliced vector consisting of u8, where the data will be stored after reading.
    /// # Returns
    /// * `a boolean` - Which is true if process is completed otherwise false and aborts the process if any of the steps, i.e
    /// start, reading address, reading ACK or reading NACK fails.
    pub fn read_from_slave(
        &mut self,
        address: u8,
        length: usize,
        data: &mut FixedSliceVec<u8>,
    ) -> bool {
        delay_ms(1);
        read_sda();

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

    /// Writes consecutive Data bytes to slave.
    /// Also sends a stop signal if either of the steps fail or writing is successful.
    /// # Returns
    /// * `a boolean` - Which is true if process is completed and aborts if any of the steps, i.e start, setting address or writing fails.
    pub fn write_to_slave(&mut self, address: u8, data: &FixedSliceVec<u8>) -> bool {
        delay_ms(1);
        if !self.start() {
            return false;
        }
        if !self.address_write(address) {
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
