// RustDuino : A generic HAL implementation for Arduino Boards in Rust
// Copyright (C) 2021  Akshit Verma, Indian Institute of Technology Kanpur
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>

// Standard crates to be used
use bit_field::BitField;
use fixed_slice_vec::FixedSliceVec;
use volatile::Volatile;

// Source code crates required
use crate::delay::delay_ms;

///  Contains registers fow TWI.
///
/// * `TWBR`: *TWI Bit Rate Register*. TWBR selects the division factor for the
/// bit rate generator. The bit rate generator is a frequency divider which generates the
/// SCL clock frequency in the master modes. See [Section 21.5.2 “Bit Rate Generator Unit”]
/// (https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf)
/// on page 180 for calculating bit rates.
///
///  * `TWSR`: *TWI Status Register*. The first 5 bits of TWSR reflect the status
/// of the TWI logic and te 2-wire Serial bus. The last 2 bits decide the bit
/// rate prescaler.
///
///  * `TWAR`: *TWI (Slave) Address Register*. The TWAR should be loaded with
/// the 7-bit slave address (in the seven most significant bits of TWAR) to
/// which the TWI will respond when programmed as a slave transmitter or
/// receiver, and not needed in the master modes.
///
///  * `TWDR`: *TWI Data Register*. n transmit mode, TWDR contains the next
/// byte to be transmitted. In receive mode, the TWDR contains the last byte
/// received. It is writable while the TWI is not in the process of shifting
/// a byte. This occurs when the TWI interrupt flag (TWINT) is set by hardware.
/// Note that the data register cannot be initialized by the user before the
/// first interrupt occurs. The data in TWDR remains stable as long as TWINT
/// is set. While data is shifted out, data on the bus is simultaneously shifted
/// in. TWDR always contains the last byte present on the bus, except after a
/// wake up from a sleep mode by the TWI interrupt. In this case, the contents
/// of TWDR is undefined.
///
/// * `TWSR`: *TWI Status Register*. The TWCR is used to control the operation
/// of the TWI. It is used to enable the TWI, to initiate a master access by
/// applying a START condition to the bus, to generate a receiver acknowledge,
/// to generate a stop condition, and to control halting of the bus while the
/// data to be written to the bus are written to the TWDR. It also indicates
/// a write collision if data is attempted written to TWDR while the register is inaccessible
///
/// * `TWAMR`: *TWI Address Mask*. The TWAMR can be loaded with a 7-bit slave
/// address mask. Each of the bits in TWAMR can mask (disable) the corresponding
/// address bits in the TWI address register (TWAR). If the mask bit is set to
/// one then the address match logic ignores the compare between the incoming
/// address bit and the corresponding bit in TWAR.
///
///  Section 21.9 of ATmega328P datasheet.
pub struct Twi {
    _twbr: Volatile<u8>,
    twsr: Volatile<u8>,
    _twar: Volatile<u8>,
    twdr: Volatile<u8>,
    twcr: Volatile<u8>,
    _twamr: Volatile<u8>,
}

// for twcr
const TWINT: u8 = 0;
const TWEN: u8 = 5;

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
        unreachable!();
        // return (-1, -1, -1);
    }
}

// TWSR values (not bits)
// (taken from avr-libc twi.h)
// Master
const START: u8 = 0x08;
const REP_START: u8 = 0x10;
// Master Transmitter;
const MT_SLA_ACK: u8 = 0x18;
const MT_DATA_ACK: u8 = 0x28;
// Master Receiver;
const MR_SLA_ACK: u8 = 0x40;
const MR_DATA_ACK: u8 = 0x50;
const MR_DATA_NACK: u8 = 0x58;
// defines and constants;
const TWSR_STATUS_MASK: u8 = 0xF8;

// return values;
const I2C_TIMEOUT: u32 = 100;

/// Sets DDRC to write direction.
pub fn write_sda() {
    unsafe {
        Volatile::new(*(0x27 as *mut u8)).update(|ddrd| {
            ddrd.set_bit(1, true);
        });
    }
}
/// Sets DDRC to write direction.
pub fn read_sda() {
    unsafe {
        Volatile::new(*(0x27 as *mut u8)).update(|ddrd| {
            ddrd.set_bit(1, false);
        });
    }
}

impl Twi {
    /// Returns a pointer to TWBR.
    /// Usage: rustduino::atmega328p::com::i2c::new()
    pub fn new() -> &'static mut Self {
        unsafe { &mut *(0xB8 as *mut Self) }
    }

    /// Waits for the process to be complete.
    /// Times out if TWINT is not set in 100 seconds.
    /// Returns if process was successful.
    /// Usage: rustduino::atmega328p::com::i2c::wait_to_complete(operation:u8)
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

    /// Iniates the TWI bus.
    /// Usage: rustduino::atmega328p::com::i2c::init()
    pub fn init(&mut self) {
        self.twsr.update(|sr| {
            sr.set_bit(TWPS0, prescaler().1);
            sr.set_bit(TWPS1, prescaler().2);
        });
        self.twcr.update(|cr| {
            cr.set_bit(TWEN, true);
        })
    }

    /// Sends a Start Signal
    /// Usage: rustduino::atmega328p::com::i2c::start()
    pub fn start(&mut self) -> bool {
        write_sda();
        self.twcr.write(0xA4); // TWINT TWSTA and TWA set to 1

        return self.wait_to_complete(START);
    }

    /// Sends a Repeat Start Signal
    /// Usage: rustduino::atmega328p::com::i2c::rep_start()
    pub fn rep_start(&mut self) -> bool {
        self.twcr.write(0xA4); // TWINT TWSTA and TWA set to 1

        return self.wait_to_complete(REP_START);
    }

    /// Stops the TWI bus.
    /// Usage: rustduino::atmega328p::com::i2c::stop()
    pub fn stop(&mut self) {
        self.twcr.write(0xB0);
    }

    /// Sets address of Slave.
    /// Used in Master Slave Mode.
    /// Returns true if process is successful
    /// Usage: rustduino::atmega328p::com::i2c::set_addr(address:u8)
    pub fn set_address(&mut self, addr: u8) -> bool {
        self.twdr.write(addr << 1 & !0x01); // loading SLA_W to TWDR
        self.twcr.update(|cr| {
            cr.set_bit(TWINT, true);
            cr.set_bit(TWEN, true);
        });

        return self.wait_to_complete(MT_SLA_ACK);
    }

    /// Checks if slave is acknowledged.
    /// Returns true if process is successful
    /// Usage: rustduino::atmega328p::com::i2c::address_read(address:u8)
    pub fn address_read(&mut self, address: u8) -> bool {
        self.twdr.write(address << 1 | 0x01);
        self.twcr.update(|cr| {
            cr.set_bit(TWINT, true);
            cr.set_bit(TWEN, true);
        });

        return self.wait_to_complete(MR_SLA_ACK);
    }

    /// Writes one byte of data to the Slave.
    /// Need to set address first.
    /// Returns true if process is successful
    /// Usage: rustduino::atmega328p::com::i2c::write(data:u8)
    pub fn write(&mut self, data: u8) -> bool {
        delay_ms(1);
        // write_sda(); // doubt if neended
        self.twdr.write(data);
        self.twcr.write(0x84); // TWCR = (1<<TWINT)|(1<<TWEN);

        return self.wait_to_complete(MT_DATA_ACK);
    }

    /// Writes consecutive bytes of data to the Slave.
    /// Need to set address first.
    /// Returns number of bytes written
    /// Usage: rustduino::atmega328p::com::i2c::write_burst(data:&FixedSliceVec<u8>)
    pub fn write_burst(&mut self, data: &FixedSliceVec<u8>) -> usize {
        let mut x: usize = 0;
        while x < data.len() {
            delay_ms(1);
            if !self.write(data[x]) {
                break;
            }
            x += 1;
        }
        return x + 1;
    }

    /// Appends the value in TWCR to the given vector.
    /// Need to set address first.
    /// Returns true if process is completed.
    /// Usage: rustduino::atmega328p::com::i2c::read_ack(data:&FixedSliceVec<u8>)
    pub fn read_ack(&mut self, data: &mut FixedSliceVec<u8>) -> bool {
        self.twcr.write(0xC4); //TWCR = (1 << TWINT) | (1 << TWEA) | (1 << TWEN)
        data.push(self.twdr.read());

        return self.wait_to_complete(MR_DATA_ACK);
    }

    /// Appends the value in TWCR to the given vector.
    /// Need to set address first.
    /// Returns true if process is completed.
    /// Usage: rustduino::atmega328p::com::i2c::read_ack_burst(data:&FixedSliceVec<u8>)
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
        self.twcr.write(0x84); //TWCR = (1 << TWINT) | (1 << TWEN)

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

    /// * Writes consecutive Data bytes to slave
    /// * Returns true if process is completed and aborts if any of the steps, i.e
    /// start, setting address or writing fails.
    /// * Sends a stop signal if either of the steps fail or writing is successful.
    /// * Usage: rustduino::atmega328p::com::i2c::write_to_slave(address:u8, data:&FixedSliceVec<u8>)
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

    /// * Reads consecutive Data bytes from slave
    /// * Requires number of bytes ro be read
    /// * Returns true if process is completed and aborts if any of the steps, i.e
    /// start, reading address, reading ACK or reading NACK fails.
    /// * Sends a stop signal if either of the steps fail or reading is successful.
    /// * Usage: rustduino::atmega328p::com::i2c::write_to_slave(address:u8, length:u8,  data:&FixedSliceVec<u8>)
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
