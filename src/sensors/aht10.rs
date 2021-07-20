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

use crate::atmega328p::com::i2c;
use crate::delay::delay_ms;
use fixed_slice_vec::FixedSliceVec;

pub enum Tempsensor {
    Aht10sensor,
}

pub struct AHT10<'a> {
    address: u8,
    i2c: i2c::Twi,
    vec: FixedSliceVec<'a, u8>,
}


const AHT10_INIT_CMD: u8 = 0xE1; //initialization command for AHT10/AHT15
const AHT10_START_MEASURMENT_CMD: u8 = 0xAC; //start measurment command
const AHT10_SOFT_RESET_CMD: u8 = 0xBA; //soft reset command

const AHT10_INIT_CAL_ENABLE: u8 = 0x08; //load factory calibration coeff
const AHT10_INIT_BUSY: u8 = 0x08;

impl<'a> AHT10<'a> {
    pub fn new(&mut self) -> &'static mut Self {
        delay_ms(20); //20ms delay to wake up

        self.soft_reset();

        if !self.initialise() {
            unreachable!("Could not intialise!");
        }
        unsafe { &mut *(0x38 as *mut Self) }
    }

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

    pub fn soft_reset(&mut self) {
        self.vec.clear();
        self.vec.push(AHT10_SOFT_RESET_CMD);

        if !self.i2c.write_to_slave(self.address, &self.vec) {
            unreachable!();
        }
        delay_ms(20);
    }

    pub fn read_to_buffer(&mut self) {
        if !self
            .i2c
            .read_from_slave(self.address, self.vec.len(), &mut self.vec)
        {
            unreachable!();
        }
    }

    pub fn trigger_slave(&mut self) {
        self.vec.clear();
        self.vec.push(AHT10_START_MEASURMENT_CMD);
        self.vec.push(0x33);
        self.vec.push(0x00);

        if !self.i2c.write_to_slave(self.address, &self.vec) {
            unreachable!();
        }
    }

    pub fn wait_for_idle(&mut self) {
        while (self.status() == 0 && AHT10_INIT_BUSY == 0) == true {
            delay_ms(5);
        }
    }

    pub fn perform_measurement(&mut self) {
        self.trigger_slave();
        self.wait_for_idle();
        self.read_to_buffer();
    }
    pub fn status(&mut self) -> u8 {
        self.read_to_buffer();
        return self.vec[0];
    }

    pub fn relative_humidity(&mut self) -> f64 {
        self.perform_measurement();
        let mut humid: f64 = (((self.vec[1] as u32) << 12)
            | ((self.vec[2] as u32) << 4)
            | ((self.vec[3] as u32) >> 4)) as f64;
        humid = (humid * 100.0) / 0x100000 as f64;
        return humid;
    }

    pub fn temperature(&mut self) -> f64 {
        self.perform_measurement();
        let mut temp: f64 = ((((self.vec[3] as u32) & 0xF) << 16)
            | (self.vec[4] as u32) << 8
            | (self.vec[5]) as u32) as f64;
        temp = ((temp as f64 * 200.0) / 0x100000 as f64) - 50.0;
        return temp;
    }
}