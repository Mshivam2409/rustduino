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

use crate::{com, delay::delay_ms};
use bit_field::BitField;
use fixed_slice_vec::FixedSliceVec;

const MPU6050_ADDRESS: u8 = 0x68; // 0x69 when AD0 pin to Vcc

///* This register configures the external Frame Synchronization (FSYNC) pin sampling and the Digital Low Pass Filter (DLPF) setting for both the gyroscopes and accelerometers.
///* Used in functions :`set_dhpf_mode()` , `set_dlpf_mode()`
const MPU6050_REG_CONFIG: u8 = 0x1A;

///* This register is used to trigger gyroscope self-test and configure the gyroscopes’ full scale range.
///* Used in functions : `set_scale()` , `get_scale()`
const MPU6050_REG_GYRO_CONFIG: u8 = 0x1B;

///* This register is used to trigger accelerometer self-test and to configure the accelerometers’ full scale range.
/// Used in functions : `set_range()` , `get_range()`
const MPU6050_REG_ACCEL_CONFIG: u8 = 0x1C;
const MPU6050_REG_FF_THRESHOLD: u8 = 0x1D;
const MPU6050_REG_FF_DURATION: u8 = 0x1E;
const MPU6050_REG_MOT_THRESHOLD: u8 = 0x1F;
const MPU6050_REG_MOT_DURATION: u8 = 0x20;
const MPU6050_REG_ZMOT_THRESHOLD: u8 = 0x21;
const MPU6050_REG_ZMOT_DURATION: u8 = 0x22;

///* This register configures the behavior of the interrupt signals at the INT pins. This register is also used to enable the FSYNC Pin to be used as an interrupt to the host application processor, as well as to enable Bypass Mode on the I2C Master. This bit also enables the clock output
///* Used in functions :`set_i2c_byepass_enabled()`, `get_i2c_bypass_enabled()`
const MPU6050_REG_INT_PIN_CFG: u8 = 0x37;

///* This register enables interrupt generation by interrupt sources.
///* Used in functions :`set_int_motion_enable()` , `get_int_motion_enable()`, `set_int_free_fall_enabled()`, `get_int_free_fall_enabled()`, `set_int_zero_motion_enabled()`, `get_int_zero_motion_enabled()`.
const MPU6050_REG_INT_ENABLE: u8 = 0x38; // INT Enable

///* This register shows the interrupt status of each interrupt generation source.
///* Used in function : `get_int_status()`.
const MPU6050_REG_INT_STATUS: u8 = 0x3A;

///* These registers store the most recent accelerometer measurements
const MPU6050_REG_ACCEL_XOUT_H: u8 = 0x3B; // Accel XOUT High
/// These registers store the most recent gyroscope measurements.
const MPU6050_REG_GYRO_XOUT_H: u8 = 0x43; //Registers for output of X,Y & Z axis.
const MPU6050_REG_MOT_DETECT_CTRL: u8 = 0x69;
const MPU6050_REG_USER_CTRL: u8 = 0x6A; // User Control
const MPU6050_REG_PWR_MGMT_1: u8 = 0x6B; // Power Management 1

pub enum MPUClockSourceT {
    MPU6050ClockInternal8MHZ,
    MPU6050ClockPllGyrox,
    MPU6050ClockPllGyroy,
    MPU6050ClockPllGyroz,
    MPU6050ClockExternal32MHZ,
    MPU6050ClockExternal19MHZ,
    MPU6050ClockKeepReset,
}

pub enum MPUdpsT {
    MPU6050Scale2000DPS,
    MPU6050Scale1000DPS,
    MPU6050Scale500DPS,
    MPU6050Scale250DPS,
}

pub enum MPURangeT {
    MPU6050Range2G,
    MPU6050Range4G,
    MPU6050Range8G,
    MPU6050Range16G,
}

pub enum MPUOnDelayT {
    MPU6050Delay3MS,
    MPU6050Delay2MS,
    MPU6050Delay1MS,
    MPU6050NoDelay,
}

pub enum MPUdhpfT {
    MPU6050dhpfReset,
    MPU6050dhpf5HZ,
    MPU6050dhpf2_5HZ,
    MPU6050dhpf1_25HZ,
    MPU6050dhpf0_63HZ,
    MPU6050dhpfHold,
}

pub enum MPUdlpfT {
    MPU6050dlpf6,
    MPU6050dlpf5,
    MPU6050dlpf4,
    MPU6050dlpf3,
    MPU6050dlpf2,
    MPU6050dlpf1,
    MPU6050dlpf0,
}

pub struct MPU6050 {
    _address: u8,
}

impl MPU6050 {
    ///Returns a mutable refernce to the struct to be used in the implementations
    pub fn new() -> &'static mut Self {
        unsafe { &mut *(0x75 as *mut Self) }
    }

    fn readregister(&mut self, reg: u8) -> u8 {
        let mut vec1: FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        vec1.push(reg);
        let i2c = com::i2c::Twi::new();
        i2c.read_from_slave(MPU6050_ADDRESS, 1, &mut vec1);
        return vec1[1];
    }

    fn writeregister(&mut self, reg: u8, value: u8) {
        let mut vec2: FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        vec2.push(reg);
        vec2.push(value);
        let i2c = com::i2c::Twi::new();
        i2c.write_to_slave(MPU6050_ADDRESS, &vec2);
    }

    fn writeregister_bit(&mut self, reg: u8, pos: u8, state: bool) {
        let mut value: u8;
        value = self.readregister(reg);
        if state {
            value |= 1 << pos;
        } else {
            value &= !(1 << pos);
        }
        self.writeregister(reg, value);
    }

    pub fn set_dlpf_mode(&mut self, dlpf: MPUdlpfT) {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_CONFIG);
        value &= 0b11111000;
        value |= match dlpf {
            MPUdlpfT::MPU6050dlpf6 => 0b110,
            MPUdlpfT::MPU6050dlpf5 => 0b101,
            MPUdlpfT::MPU6050dlpf4 => 0b100,
            MPUdlpfT::MPU6050dlpf3 => 0b011,
            MPUdlpfT::MPU6050dlpf2 => 0b010,
            MPUdlpfT::MPU6050dlpf1 => 0b001,
            MPUdlpfT::MPU6050dlpf0 => 0b000,
        };
        self.writeregister(MPU6050_REG_CONFIG, value);
    }

    pub fn set_dhpf_mode(&mut self, dhpf: MPUdhpfT) {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_CONFIG);
        value &= 0b11111100;
        value |= match dhpf {
            MPUdhpfT::MPU6050dhpfReset => 0b000,
            MPUdhpfT::MPU6050dhpf5HZ => 0b001,
            MPUdhpfT::MPU6050dhpf2_5HZ => 0b010,
            MPUdhpfT::MPU6050dhpf1_25HZ => 0b011,
            MPUdhpfT::MPU6050dhpf0_63HZ => 0b100,
            MPUdhpfT::MPU6050dhpfHold => 0b101,
        };
        self.writeregister(MPU6050_REG_CONFIG, value);
    }

    pub fn set_scale(&mut self, scale: MPUdpsT) {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_GYRO_CONFIG);
        value &= 0b11100111;
        value |= (match scale {
            MPUdpsT::MPU6050Scale2000DPS => 3,
            MPUdpsT::MPU6050Scale1000DPS => 2,
            MPUdpsT::MPU6050Scale500DPS => 1,
            MPUdpsT::MPU6050Scale250DPS => 0,
        } << 3);
        self.writeregister(MPU6050_REG_GYRO_CONFIG, value);
    }

    pub fn get_scale(&mut self) -> MPUdpsT {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_GYRO_CONFIG);
        value &= 0b00011000;
        value >>= 3;
        if value == 3 {
            return MPUdpsT::MPU6050Scale2000DPS;
        } else if value == 2 {
            return MPUdpsT::MPU6050Scale1000DPS;
        } else if value == 1 {
            return MPUdpsT::MPU6050Scale500DPS;
        } else {
            return MPUdpsT::MPU6050Scale250DPS;
        }
    }

    pub fn set_range(&mut self, range: MPURangeT) {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_ACCEL_CONFIG);
        value &= 0b11100111;
        value |= (match range {
            MPURangeT::MPU6050Range2G => 0,
            MPURangeT::MPU6050Range4G => 1,
            MPURangeT::MPU6050Range8G => 2,
            MPURangeT::MPU6050Range16G => 3,
        } << 3);
        self.writeregister(MPU6050_REG_ACCEL_CONFIG, value);
    }

    pub fn get_range(&mut self) -> MPURangeT {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_ACCEL_CONFIG);
        value &= 0b00011000;
        value >>= 3;
        if value == 3 {
            return MPURangeT::MPU6050Range16G;
        } else if value == 2 {
            return MPURangeT::MPU6050Range8G;
        } else if value == 1 {
            return MPURangeT::MPU6050Range4G;
        } else {
            return MPURangeT::MPU6050Range2G;
        }
    }

    pub fn set_clock_source(&mut self, source: MPUClockSourceT) {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_PWR_MGMT_1);
        value &= 0b11111000;
        value |= match source {
            MPUClockSourceT::MPU6050ClockInternal8MHZ => 0,
            MPUClockSourceT::MPU6050ClockPllGyrox => 1,
            MPUClockSourceT::MPU6050ClockPllGyroy => 2,
            MPUClockSourceT::MPU6050ClockPllGyroz => 3,
            MPUClockSourceT::MPU6050ClockExternal32MHZ => 4,
            MPUClockSourceT::MPU6050ClockExternal19MHZ => 5,
            MPUClockSourceT::MPU6050ClockKeepReset => 7,
        };
        self.writeregister(MPU6050_REG_PWR_MGMT_1, value);
    }

    pub fn get_clock_source(&mut self) -> MPUClockSourceT {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_PWR_MGMT_1);
        value &= 0b00000111;
        if value == 0 {
            return MPUClockSourceT::MPU6050ClockInternal8MHZ;
        } else if value == 1 {
            return MPUClockSourceT::MPU6050ClockPllGyrox;
        } else if value == 2 {
            return MPUClockSourceT::MPU6050ClockPllGyroy;
        } else if value == 3 {
            return MPUClockSourceT::MPU6050ClockPllGyroz;
        } else if value == 4 {
            return MPUClockSourceT::MPU6050ClockExternal32MHZ;
        } else if value == 5 {
            return MPUClockSourceT::MPU6050ClockExternal19MHZ;
        } else {
            return MPUClockSourceT::MPU6050ClockKeepReset;
        }
    }

    pub fn set_int_free_fall_enabled(&mut self, state: bool) {
        self.writeregister_bit(MPU6050_REG_INT_ENABLE, 7, state);
    }

    pub fn get_int_free_fall_enabled(&mut self) -> bool {
        let value = self.readregister(MPU6050_REG_INT_ENABLE);
        return value.get_bit(6);
    }

    pub fn set_accel_power_on_delay(&mut self, delay: MPUOnDelayT) {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_MOT_DETECT_CTRL);
        value &= 0b11001111;
        value |= match delay {
            MPUOnDelayT::MPU6050Delay3MS => 3,
            MPUOnDelayT::MPU6050Delay2MS => 2,
            MPUOnDelayT::MPU6050Delay1MS => 1,
            MPUOnDelayT::MPU6050NoDelay => 0,
        };
        self.writeregister(MPU6050_REG_MOT_DETECT_CTRL, value);
    }

    pub fn get_accel_power_on_delay(&mut self) -> MPUOnDelayT {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_MOT_DETECT_CTRL);
        value &= 0b00110000;
        if value == 3 {
            return MPUOnDelayT::MPU6050Delay3MS;
        } else if value == 2 {
            return MPUOnDelayT::MPU6050Delay2MS;
        } else if value == 1 {
            return MPUOnDelayT::MPU6050Delay1MS;
        } else {
            return MPUOnDelayT::MPU6050NoDelay;
        }
    }

    pub fn set_motion_detection_threshold(&mut self, threshold: u8) {
        self.writeregister(MPU6050_REG_MOT_THRESHOLD, threshold);
    }

    pub fn get_motion_detection_threshold(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_MOT_THRESHOLD);
    }

    pub fn set_motion_detection_duration(&mut self, duration: u8) {
        self.writeregister(MPU6050_REG_MOT_DURATION, duration);
    }

    pub fn get_motion_detection_duration(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_MOT_DURATION);
    }

    pub fn set_zero_motion_detection_threshold(&mut self, threshold: u8) {
        self.writeregister(MPU6050_REG_ZMOT_THRESHOLD, threshold);
    }

    pub fn get_zero_motion_detection_threshold(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_ZMOT_THRESHOLD);
    }

    pub fn set_zero_motion_detection_duration(&mut self, duration: u8) {
        self.writeregister(MPU6050_REG_ZMOT_DURATION, duration);
    }

    pub fn get_zero_motion_detection_duration(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_ZMOT_DURATION);
    }

    pub fn set_free_fall_detection_threshold(&mut self, threshold: u8) {
        self.writeregister(MPU6050_REG_FF_THRESHOLD, threshold);
    }

    pub fn get_free_fall_detection_threshold(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_FF_THRESHOLD);
    }

    pub fn set_free_fall_detection_duration(&mut self, duration: u8) {
        self.writeregister(MPU6050_REG_FF_DURATION, duration);
    }

    pub fn get_free_fall_detection_duration(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_FF_DURATION);
    }

    pub fn set_sleep_enabled(&mut self, state: bool) {
        self.writeregister_bit(MPU6050_REG_PWR_MGMT_1, 6, state);
    }

    pub fn get_sleep_enabled(&mut self) -> bool {
        let value = self.readregister(MPU6050_REG_PWR_MGMT_1);
        return value.get_bit(6);
    }

    pub fn get_int_zero_motion_enabled(&mut self) -> bool {
        let value = self.readregister(MPU6050_REG_INT_ENABLE);
        return value.get_bit(5);
    }

    pub fn set_int_zero_motion_enabled(&mut self, state: bool) {
        self.writeregister_bit(MPU6050_REG_INT_ENABLE, 5, state);
    }

    pub fn get_int_motion_enabled(&mut self) -> bool {
        let value = self.readregister(MPU6050_REG_INT_ENABLE);
        return value.get_bit(6);
    }

    pub fn set_int_motion_enabled(&mut self, state: bool) {
        self.writeregister_bit(MPU6050_REG_INT_ENABLE, 6, state);
    }

    pub fn set_i2c_master_mode_enabled(&mut self, state: bool) {
        self.writeregister_bit(MPU6050_REG_USER_CTRL, 5, state);
    }

    pub fn get_i2c_master_mode_enabled(&mut self) -> bool {
        let value = self.readregister(MPU6050_REG_USER_CTRL);
        return value.get_bit(5);
    }

    pub fn set_i2c_byepass_enabled(&mut self, state: bool) {
        self.writeregister_bit(MPU6050_REG_INT_PIN_CFG, 1, state);
    }

    pub fn get_i2c_byepass_enabled(&mut self) -> bool {
        let value = self.readregister(MPU6050_REG_INT_PIN_CFG);
        return value.get_bit(1);
    }

    pub fn get_int_status(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_INT_STATUS);
    }

    ///* Reads the three, two-byte accelerometer values from the sensor.
    ///* Returns the two-byte raw accelerometer values as a 32-bit float.
    ///* The vec accel_output stores the raw values of the accelerometer where `accel_output[0]` is the x-axis, `accel_output[1]` is the y-axis and `accel_output[2]` is the z-axis output respectively. These raw values are then converted to g's per second according to the scale given as input in `begin()` function.
    pub fn read_accel(&mut self) -> FixedSliceVec<f32> {
        let mut v: FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        v.push(MPU6050_REG_ACCEL_XOUT_H);
        let i2c = com::i2c::Twi::new();
        i2c.read_from_slave(MPU6050_ADDRESS, 6, &mut v); //input from slave
        let mut accel_output: FixedSliceVec<f32> = FixedSliceVec::new(&mut []);
        accel_output.push((((v[1] as u16) << 8) | (v[2] as u16)) as f32); //input of X axis
        accel_output.push((((v[3] as u16) << 8) | (v[4] as u16)) as f32); //input of Y axis
        accel_output.push((((v[5] as u16) << 8) | (v[6] as u16)) as f32); //input of Z axis
        return accel_output;
    }

    ///* Reads the three, two-byte gyroscope values from the sensor.
    ///* Returns the two-byte raw gyroscope values as a 32-bit float.
    ///* The vec gyro_output stores the raw values of the gyroscope where `gyro_output[0]` is the x-axis, `gyro_output[1]` is the y-axis and `gyro_output[2]` is the z-axis output respectively. These raw values are then converted to degrees per second according to the scale given as input in `begin()` function.
    pub fn read_gyro(&mut self) -> FixedSliceVec<f32> {
        let mut v: FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        v.push(MPU6050_REG_GYRO_XOUT_H);
        let i2c = com::i2c::Twi::new();
        i2c.read_from_slave(MPU6050_ADDRESS, 6, &mut v); //input from slave
        let mut gyro_output: FixedSliceVec<f32> = FixedSliceVec::new(&mut []);
        gyro_output.push((((v[1] as u16) << 8) | (v[2] as u16)) as f32); //input of X axis
        gyro_output.push((((v[3] as u16) << 8) | (v[4] as u16)) as f32); //input of Y axis
        gyro_output.push((((v[5] as u16) << 8) | (v[6] as u16)) as f32); //input of Z axis
        return gyro_output;
    }

    /// Starts the sensor by setting the device to active mode ,setting the accelerometer range and gyroscope scale.
    pub fn begin(&mut self, scale: MPUdpsT, range: MPURangeT) -> bool {
        delay_ms(5);

        //Set clock source.
        self.set_clock_source(MPUClockSourceT::MPU6050ClockPllGyrox);

        //Set scale and range.
        self.set_range(range);
        self.set_scale(scale);

        //disable sleep mode.
        self.set_sleep_enabled(false);

        return true;
    }
}
