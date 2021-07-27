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

use crate::com::i2c;
use crate::delay::delay_ms;
use fixed_slice_vec::FixedSliceVec;

pub struct AHT10<'a> {
    address: u8,
    i2c: i2c::Twi,
    vec: FixedSliceVec<'a, u8>,
}

/// Constant values for AHT10 temperature and humity sensor.
const AHT10_INIT_CMD: u8 = 0xE1; //initialization command for AHT10/AHT15
const AHT10_START_MEASURMENT_CMD: u8 = 0xAC; //start measurment command
const AHT10_SOFT_RESET_CMD: u8 = 0xBA; //soft reset command
const AHT10_INIT_CAL_ENABLE: u8 = 0x08; //load factory calibration coeff
const AHT10_INIT_BUSY: u8 = 0x08; //Status bit for busy

impl<'a> AHT10<'a> {
    /// * 20ms delay to wake up.
    /// * I2C address is set.
    /// * Usage: rustduino::sensors::aht10::new()
    pub fn new(&mut self) -> &'static mut Self {
        delay_ms(20);

        self.soft_reset();

        if !self.initialise() {
            unreachable!();
        }
        unsafe { &mut *(0x38 as *mut Self) }
    }

    ///Return pointer
    pub fn get() -> &'static mut Self {
        unsafe { &mut *(0x38 as *mut Self) }
    }

    /// * Initiates the transmission by self initiating the sensor.
    /// * Returns true if done otherwise false.
    /// * Usage:let aht10=rustduino::sensors::AHT10::new()
    /// * aht10.initialise()
    pub fn initialise(&mut self) -> bool {
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

    /// * Restart sensor, without power off.
    /// *  It takes ~20ms.
    /// *  All registers restores to default.
    /// * Usage:let aht10=rustduino::sensors::AHT10::new()
    /// * aht10.soft_reset()
    pub fn soft_reset(&mut self) {
        self.vec.clear();
        self.vec.push(AHT10_SOFT_RESET_CMD);

        if !self.i2c.write_to_slave(self.address, &self.vec) {
            unreachable!();
        }
        delay_ms(20);
    }

    /// * Reads data from slave.
    /// * Usage:let aht10=rustduino::sensors::AHT10::new()
    /// * aht10.read_to_buffer()
    pub fn read_to_buffer(&mut self) {
        if !self
            .i2c
            .read_from_slave(self.address, self.vec.len(), &mut self.vec)
        {
            unreachable!();
        }
    }

    /// * Triggers the AHT10 to read temperature/humidity.
    /// * Usage:let aht10=rustduino::sensors::AHT10::new()
    /// * aht10.trigger_slave()
    pub fn trigger_slave(&mut self) {
        self.vec.clear();
        self.vec.push(AHT10_START_MEASURMENT_CMD);
        self.vec.push(0x33);
        self.vec.push(0x00);

        if !self.i2c.write_to_slave(self.address, &self.vec) {
            unreachable!();
        }
    }

    /// * Delay of 5ms when status bit is 0 and sensor is busy.
    /// * Usage:let aht10=rustduino::sensors::AHT10::new()
    /// * aht10.wait_for_idle()
    pub fn wait_for_idle(&mut self) {
        while (self.status() == 0 && AHT10_INIT_BUSY == 0) == true {
            delay_ms(5);
        }
    }

    /// * Performs measurement using the functions mentioned.
    /// * Usage:let aht10=rustduino::sensors::AHT10::new()
    /// * aht10.perform_measurement()
    pub fn perform_measurement(&mut self) {
        self.trigger_slave();
        self.wait_for_idle();
        self.read_to_buffer();
    }

    /// * Reads status bit returned by the slave.
    /// * Usage:let aht10=rustduino::sensors::AHT10::new()
    /// * aht10.status()
    pub fn status(&mut self) -> u8 {
        self.read_to_buffer();
        return self.vec[0];
    }

    /// * Reads 20 bit raw humidity data and returns relative humidity .
    /// * Usage:let aht10=rustduino::sensors::AHT10::new()
    /// * aht10.relative_humidity()
    pub fn relative_humidity(&mut self) -> f32 {
        self.perform_measurement();
        let mut humid: f32 = (((self.vec[1] as u32) << 12)
            | ((self.vec[2] as u32) << 4)
            | ((self.vec[3] as u32) >> 4)) as f32;
        humid = (humid * 100.0) / 0x100000 as f32;
        return humid;
    }

    /// * Reads 20 bit raw temperature data and returns temperature .
    /// * Usage:let aht10=rustduino::sensors::AHT10::new()
    /// * aht10.temperature()
    pub fn temperature(&mut self) -> f32 {
        self.perform_measurement();
        let mut temp: f32 = ((((self.vec[3] as u32) & 0xF) << 16)
            | (self.vec[4] as u32) << 8
            | (self.vec[5]) as u32) as f32;
        temp = ((temp as f32 * 200.0) / 0x100000 as f32) - 50.0;
        return temp;
    }
}
