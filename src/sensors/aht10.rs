// RustDuino : A generic HAL implementation for Arduino Boards in Rust
// Copyright (C) 2021  Sanmati Pande, Indian Institute of Technology Kanpur

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

//! This code implements the I2C protocol to control the AHT10
//! sensor which could be used to read the temperature and
//! humidity and stored in a sliced vector which could be given
//! as an output.

use crate::com::i2c;
use crate::delay::delay_ms;
use fixed_slice_vec::FixedSliceVec;

/// Used to control the AHT10 Arduino sensor
/// # Elements
/// * `address` - a u8, used to store the address to control the functioning AHT10 sensor.
/// * `i2c` - a `twi` struct object, which would be used to interface the functioning of sensor with communication protocol I2C.
/// * `vec` - a vector with u8 objects, It would be used to store the data read through the sensors.
#[repr(C, packed)]
pub struct AHT10<'a> {
    address: u8,
    i2c: i2c::Twi,
    vec: FixedSliceVec<'a, u8>,
}

// Constant values for AHT10 temperature and humity sensor.
const AHT10_INIT_CMD: u8 = 0xE1; //initialization command for AHT10/AHT15
const AHT10_START_MEASURMENT_CMD: u8 = 0xAC; //start measurment command
const AHT10_SOFT_RESET_CMD: u8 = 0xBA; //soft reset command
const AHT10_INIT_CAL_ENABLE: u8 = 0x08; //load factory calibration coeff
const AHT10_INIT_BUSY: u8 = 0x08; //Status bit for busy

impl<'a> AHT10<'a> {
    /// Creates a new memory mapped IO at the correct location including a 20ms reset delay for wake-up.
    /// # Returns
    /// * `a reference to AHT10 structure` - Which would be used to control the sensor.
    pub fn new(&mut self) -> &'static mut Self {
        delay_ms(20);

        self.soft_reset();

        unsafe {
            if !self.initialise() {
                unreachable!();
            }
        }
        unsafe { &mut *(0x38 as *mut Self) }
    }

    /// Returns reference to the structure without any reset delay.
    /// # Returns
    /// * `a reference to AHT10 structure` - Which would be used to control the sensor.
    pub fn get() -> &'static mut Self {
        unsafe { &mut *(0x38 as *mut Self) }
    }

    /// Initiates the transmission by self initiating the sensor.
    /// # Returns
    /// * `a boolean` - Which is true if transmission initiated otherwise false.
    pub unsafe fn initialise(&mut self) -> bool {
        self.vec.clear();
        self.vec.push(AHT10_INIT_CMD);
        self.vec.push(0x33);
        self.vec.push(0x00);

        if !self.i2c.write_to_slave(self.address, &self.vec) {
            unreachable!();
        }
        self.wait_for_idle();
        if !(self.status() == 0 && AHT10_INIT_CAL_ENABLE == 0) {
            return false;
        }
        return true;
    }

    /// Restart sensor, without power off in around ~20ms with all registers restored to default.
    pub fn soft_reset(&mut self) {
        unsafe {
            self.vec.clear();
            self.vec.push(AHT10_SOFT_RESET_CMD);

            if !self.i2c.write_to_slave(self.address, &self.vec) {
                unreachable!();
            }
            delay_ms(20);
        }
    }

    /// Reads data from slave mode using the I2C protocol.
    pub unsafe fn read_to_buffer(&mut self) {
        if !self
            .i2c
            .read_from_slave(self.address, self.vec.len(), &mut self.vec)
        {
            unreachable!();
        }
    }

    /// Triggers the AHT10 to read temperature/humidity.
    pub unsafe fn trigger_slave(&mut self) {
        self.vec.clear();
        self.vec.push(AHT10_START_MEASURMENT_CMD);
        self.vec.push(0x33);
        self.vec.push(0x00);

        if !self.i2c.write_to_slave(self.address, &self.vec) {
            unreachable!();
        }
    }

    /// Adds a delay of 5ms when the sensor is already busy with some processing.
    pub unsafe fn wait_for_idle(&mut self) {
        while (self.status() == 0 && AHT10_INIT_BUSY == 0) == true {
            delay_ms(5);
        }
    }

    /// Performs measurement of temperature using the functions `trigger_slave()` and `read_to_buffer()`.
    pub unsafe fn perform_measurement(&mut self) {
        self.trigger_slave();
        self.wait_for_idle();
        self.read_to_buffer();
    }

    /// Reads value returned by the slave.
    /// # Returns
    /// * `a u8` - The read value.
    pub unsafe fn status(&mut self) -> u8 {
        self.read_to_buffer();
        return self.vec[0];
    }

    /// Reads 20 bit raw humidity data.
    /// # Returns
    /// * `a f64` - The relative humidity in percentage.
    pub fn relative_humidity(&mut self) -> f64 {
        unsafe {
            self.perform_measurement();
            let mut humid: f64 = (((self.vec[1] as u32) << 12)
                | ((self.vec[2] as u32) << 4)
                | ((self.vec[3] as u32) >> 4)) as f64;
            humid = (humid * 100.0) / 0x100000 as f64;
            return humid;
        }
    }

    /// Reads 20 bit raw temperature data.
    /// # Returns
    /// * `a f64` - The temperature in degree celsius.
    pub fn temperature(&mut self) -> f64 {
        unsafe {
            self.perform_measurement();
            let mut temp: f64 = ((((self.vec[3] as u32) & 0xF) << 16)
                | (self.vec[4] as u32) << 8
                | (self.vec[5]) as u32) as f64;
            temp = ((temp as f64 * 200.0) / 0x100000 as f64) - 50.0;
            return temp;
        }
    }
}
