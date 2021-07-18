use crate::delay::delay_ms;
use bit_field::BitField;
use fixed_slice_vec::FixedSliceVec;
use volatile::Volatile;
use crate::atmega2560p::com::i2c::*;

const MPU6050_ADDRESS: u8 = 0x68; // 0x69 when AD0 pin to Vcc
const MPU6050_REG_ACCEL_XOFFS_H: u8 = 0x06;
const MPU6050_REG_ACCEL_XOFFS_L: u8 = 0x07;
const MPU6050_REG_ACCEL_YOFFS_H: u8 = 0x08;
const MPU6050_REG_ACCEL_YOFFS_L: u8 = 0x09;
const MPU6050_REG_ACCEL_ZOFFS_H: u8 = 0x0A;
const MPU6050_REG_ACCEL_ZOFFS_L: u8 = 0x0B;
const MPU6050_REG_ACCEL_SMPLRT_DIV: u8 = 0x0C;
const MPU6050_REG_GYRO_XOFFS_H: u8 = 0x13;
const MPU6050_REG_GYRO_XOFFS_L: u8 = 0x14;
const MPU6050_REG_GYRO_YOFFS_H: u8 = 0x15;
const MPU6050_REG_GYRO_YOFFS_L: u8 = 0x16;
const MPU6050_REG_GYRO_ZOFFS_H: u8 = 0x17;
const MPU6050_REG_GYRO_ZOFFS_L: u8 = 0x18;
const MPU6050_REG_CONFIG: u8 = 0x1A;
const MPU6050_REG_GYRO_CONFIG: u8 = 0x1B; // Gyroscope configuration
const MPU6050_REG_ACCEL_CONFIG: u8 = 0x1C; // accelerometer configuration
const MPU6050_REG_FF_THRESHOLD: u8 = 0x1D;
const MPU6050_REG_FF_DURATION: u8 = 0x1E;
const MPU6050_REG_MOT_THRESHOLD: u8 = 0x1F;
const MPU6050_REG_MOT_DURATION: u8 = 0x20;
const MPU6050_REG_ZMOT_THRESHOLD: u8 = 0x21;
const MPU6050_REG_ZMOT_DURATION: u8 = 0x22;
const MPU6050_REG_FIFO_EN: u8 = 0x23;
const MPU6050_REG_I2C_MST_CTRL: u8 = 0x24;
const MPU6050_REG_I2C_SLV0_ADDR: u8 = 0x25;
const MPU6050_REG_I2C_SLV0_REG: u8 = 0x26;
const MPU6050_REG_I2C_SLV0_CTRL: u8 = 0x27;
const MPU6050_REG_I2C_SLV1_ADDR: u8 = 0x28;
const MPU6050_REG_I2C_SLV1_REG: u8 = 0x29;
const MPU6050_REG_I2C_SLV1_CTRL: u8 = 0x2A;
const MPU6050_REG_I2C_SLV2_ADDR: u8 = 0x2B;
const MPU6050_REG_I2C_SLV2_REG: u8 = 0x2C;
const MPU6050_REG_I2C_SLV2_CTRL: u8 = 0x2D;
const MPU6050_REG_I2C_SLV3_ADDR: u8 = 0x2E;
const MPU6050_REG_I2C_SLV3_REG: u8 = 0x2F;
const MPU6050_REG_I2C_SLV3_CTRL: u8 = 0x30;
const MPU6050_REG_I2C_SLV4_ADDR: u8 = 0x31;
const MPU6050_REG_I2C_SLV4_REG: u8 = 0x32;
const MPU6050_REG_I2C_SLV4_DO: u8 = 0x33;
const MPU6050_REG_I2C_SLV4_CTRL: u8 = 0x34;
const MPU6050_REG_I2C_SLV4_DI: u8 = 0x35;
const MPU6050_REG_I2C_MST_STATUS: u8 = 0x36;
const MPU6050_REG_INT_PIN_CFG: u8 = 0x37; // INT Pin. Bypass Enable Configuration
const MPU6050_REG_INT_ENABLE: u8 = 0x38; // INT Enable
const MPU6050_REG_INT_STATUS: u8 = 0x3A; // INT Status
const MPU6050_REG_ACCEL_XOUT_H: u8 = 0x3B; // Accel XOUT High
const MPU6050_REG_ACCEL_XOUT_L: u8 = 0x3C; // Accel XOUT Low
const MPU6050_REG_ACCEL_YOUT_H: u8 = 0x3D; // Accel YOUT High
const MPU6050_REG_ACCEL_YOUT_L: u8 = 0x3E;
const MPU6050_REG_ACCEL_ZOUT_H: u8 = 0x3F;
const MPU6050_REG_ACCEL_ZOUT_L: u8 = 0x40;
const MPU6050_REG_TEMP_OUT_H: u8 = 0x41;
const MPU6050_REG_TEMP_OUT_L: u8 = 0x42;
const MPU6050_REG_GYRO_XOUT_H: u8 = 0x43;
const MPU6050_REG_GYRO_XOUT_L: u8 = 0x44;
const MPU6050_REG_GYRO_YOUT_H: u8 = 0x45;
const MPU6050_REG_GYRO_YOUT_L: u8 = 0x46;
const MPU6050_REG_GYRO_ZOUT_H: u8 = 0x47;
const MPU6050_REG_GYRO_ZOUT_L: u8 = 0x48;
const MPU6050_REG_EXT_SENS_DATA_00: u8 = 0x49;
const MPU6050_REG_EXT_SENS_DATA_01: u8 = 0x4A;
const MPU6050_REG_EXT_SENS_DATA_02: u8 = 0x4B;
const MPU6050_REG_EXT_SENS_DATA_03: u8 = 0x4C;
const MPU6050_REG_EXT_SENS_DATA_04: u8 = 0x4D;
const MPU6050_REG_EXT_SENS_DATA_05: u8 = 0x4E;
const MPU6050_REG_EXT_SENS_DATA_06: u8 = 0x4F;
const MPU6050_REG_EXT_SENS_DATA_07: u8 = 0x50;
const MPU6050_REG_EXT_SENS_DATA_08: u8 = 0x51;
const MPU6050_REG_EXT_SENS_DATA_09: u8 = 0x52;
const MPU6050_REG_EXT_SENS_DATA_10: u8 = 0x53;
const MPU6050_REG_EXT_SENS_DATA_11: u8 = 0x54;
const MPU6050_REG_EXT_SENS_DATA_12: u8 = 0x55;
const MPU6050_REG_EXT_SENS_DATA_13: u8 = 0x56;
const MPU6050_REG_EXT_SENS_DATA_14: u8 = 0x57;
const MPU6050_REG_EXT_SENS_DATA_15: u8 = 0x58;
const MPU6050_REG_EXT_SENS_DATA_16: u8 = 0x59;
const MPU6050_REG_EXT_SENS_DATA_17: u8 = 0x5A;
const MPU6050_REG_EXT_SENS_DATA_18: u8 = 0x5B;
const MPU6050_REG_EXT_SENS_DATA_19: u8 = 0x5C;
const MPU6050_REG_EXT_SENS_DATA_20: u8 = 0x5D;
const MPU6050_REG_EXT_SENS_DATA_21: u8 = 0x5E;
const MPU6050_REG_EXT_SENS_DATA_22: u8 = 0x5F;
const MPU6050_REG_EXT_SENS_DATA_23: u8 = 0x60;
const MPU6050_REG_MOT_DETECT_STATUS: u8 = 0x61;
const MPU6050_REG_I2C_SLV0_DO: u8 = 0x63;
const MPU6050_REG_I2C_SLV1_DO: u8 = 0x64;
const MPU6050_REG_I2C_SLV2_DO: u8 = 0x65;
const MPU6050_REG_I2C_SLV3_DO: u8 = 0x66;
const MPU6050_REG_I2C_MST_DELAY_CTRL: u8 = 0x67;
const MPU6050_REG_SIGNAL_PATH_RESET: u8 = 0x68;
const MPU6050_REG_MOT_DETECT_CTRL: u8 = 0x69;
const MPU6050_REG_USER_CTRL: u8 = 0x6A; // User Control
const MPU6050_REG_PWR_MGMT_1: u8 = 0x6B; // Power Management 1
const MPU6050_REG_PWR_MGMT_2: u8 = 0x6C;
const MPU6050_REG_FIFO_COUNTH: u8 = 0x72;
const MPU6050_REG_FIFO_COUNTL: u8 = 0x73;
const MPU6050_REG_FIFO_R_W: u8 = 0x74;
const MPU6050_REG_WHO_AM_I: u8 = 0x75; // Who Am I

pub enum mpu_ClockSource_t  {
    MPU_CLOCK_INTERNAL_8MHZ = 0,
    MPU_CLOCK_PLL_GYROX = 1,
    MPU_CLOCK_PLL_GYROY = 2,
    MPU_CLOCK_PLL_GYROZ = 3,
    MPU_CLOCK_EXTERNAL_32MHZ = 4, 
    MPU_CLOCK_EXTERNAL_19MHZ = 5,
    MPU_CLOCK_KEEP_RESET = 7,
}

pub enum mpu_dps_t  {
    MPU6050_SCALE_2000DPS = 3,
    MPU6050_SCALE_1000DPS = 2,
    MPU6050_SCALE_500DPS = 1,
    MPU6050_SCALE_250DPS = 0,
}

pub enum mpu_range_t  {
    MPU6050_RANGE_2_G = 0,
    MPU6050_RANGE_4_G = 1,
    MPU6050_RANGE_8_G = 2,
    MPU6050_RANGE_16_G = 3,
}

pub enum mpu_OnDelay_t  {
    MPU6050_DELAY_3MS = 3,
    MPU6050_DELAY_2MS = 2,
    MPU6050_DELAY_1MS = 1,
    MPU6050_NO_DELAY = 0,
}

pub enum mpu_dhpc_t  {
    MPU6050_DHPF_RESET = 0,
    MPU6050_DHPF_5HZ = 1,
    MPU6050_DHPF_2_5HZ = 2,
    MPU6050_DHPF_1_25HZ = 3,
    MPU6050_DHPF_0_63HZ = 4,
    MPU6050_DHPF_HOLD = 7,
}

pub enum mpu_dlpf_t  {
    MPU6050_DLPF_6 = 0b110,
    MPU6050_DLPF_5 = 0b101,
    MPU6050_DLPF_4 = 0b100,
    MPU6050_DLPF_3 = 0b011,
    MPU6050_DLPF_2 = 0b010,
    MPU6050_DLPF_1 = 0b001,
    MPU6050_DLPF_0 = 0b000,
}

pub struct MPU6050 {
    //address: Volatile<u8>,
    i2c:Twi, 
    //atmega2560p::com::i2C::Twi::new(),
    //vec: FixedSliceVec<u8>,
}

impl MPU6050 {

    pub fn new () {
        let Self = MPU6050 {
            i2c: atmega2560p::com::i2C::Twi::new(),
        };
    }
    fn readregister(&mut self,reg: u8) -> u8 {
        let mut vec1 : FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        vec1.push(reg);
        self.i2c.read_from_slave(MPU6050_ADDRESS, 1, vec);
        return vec1[1];
    }
    
    fn writeregister(&mut self, reg: u8, value: u8){
        let mut vec2 : FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        vec2.push(reg);
        vec2.push(value);
        self.i2c.write_to_slave(MPU6050_ADDRESS, vec2)
    }

    fn writeregisterBit(&mut self,reg: u8, pos: u8, state: bool){
        let mut value: u8;
        value = self.readregister(reg);
        if state {
            value |= 1 << pos;
        } else {
            value &= !(1 << pos);
        }
        self.writeregister(reg, value);
    }

    pub fn SetClockSource() {
        
    }

    pub fn SetDLPF(dlpf: mpu_dlpf_t) {
        
    }

    pub fn SetScale() {
        
    }

    pub fn SetRange() {
        
    }

    pub fn SetDHPFMode() {
        
    }

    pub fn SetDLPFMode() {
        
    }

    pub fn SetAccelPowerOnDelay(){
        
    }

    pub fn SetIntFreeFallEnabled(){
        
    }

    pub fn SetMotionDetectionThreshold(){
        
    }

    pub fn SetMotionDetectionDuration(){
        
    }

    pub fn SetZeroMotionDetectionThreshold(){
        
    }

    pub fn SetZeroMotionDetectionDuration(){
        
    }

    pub fn SetFreeFallDetectionThreshold(){
        
    }

    pub fn SetFreeFallDetectionDuration(){
        
    }

    pub fn SetSleepEnabled(bool :state ){
        self.writeregisterBit(MPU6050_REG_PWR_MGMT_1, 6, state);
    }

    pub fn GetSleepEnabled() -> bool{
        value = self.readregister(MPU6050_REG_PWR_MGMT_1);
        return get_bit(value,6);
    }

    pub fn GetIntZeroMotionEnabled() -> bool{
        let mut value = self.readregister(MPU6050_REG_INT_ENABLE);
        return get_bit(value,5);
    }

    pub fn SetIntZeroMotionEnabled(state: bool){
        self.writeregisterBit(MPU6050_REG_INT_ENABLE, 5, state);
    }

    pub fn GetIntMotionEnabled() -> bool{
       let mut value = self.readregister(MPU6050_REG_INT_ENABLE);
        return get_bit(value,6);
    }

    pub fn SetIntMotionEnabled(state: bool){
        self.writeregisterBit(MPU6050_REG_INT_ENABLE, 6, state);
    }

    pub fn SetI2CMasterModeEnabled(state: bool){
        self.writeregisterBit(MPU6050_REG_USER_CTRL, 5, state);
    }

    pub fn GetI2CMasterModeEnabled() -> bool{
        let mut value = self.readregister(MPU6050_REG_USER_CTRL);
        return get_bit(value,5);
    }

    pub fn SetI2CByepassEnabled(){
        
    }

    pub fn Calibrategyro(){
        
    }

    pub fn SetThreshold(){
        
    }

    // pub fn Calibrategyro(){
    //     ;
    // }

    // pub fn Calibrategyro(){
    //     ;
    // }

    // pub fn Calibrategyro(){
    //     ;
    // }

    // pub fn Calibrategyro(){
        
    // }

}