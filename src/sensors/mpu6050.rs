use crate::delay::delay_ms;
use bit_field::BitField;
use fixed_slice_vec::FixedSliceVec;
use volatile::Volatile;
use crate::atmega2560p::com::i2c;

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

pub enum MPUClockSourceT  {
    MPU6050ClockInternal8MHZ = 0,
    MPU6050ClockPllGyrox = 1,
    MPU6050ClockPllGyroy = 2,
    MPU6050ClockPllGyroz = 3,
    MPU6050ClockExternal32MHZ = 4, 
    MPU6050ClockExternal19MHZ = 5,
    MPU6050ClockKeepReset = 7,
}

pub enum MPUdpsT  {
    MPU6050Scale2000DPS = 3,
    MPU6050Scale1000DPS =2,
    MPU6050Scale500DPS =1,
    MPU6050Scale250DPS =0,
}

pub enum MPURangeT  {
    MPU6050Range2G = 0,
    MPU6050Range4G = 1,
    MPU6050Range8G = 2,
    MPU6050Range16G = 3,
}

pub enum MPUOnDelayT  {
    MPU6050Delay3MS = 3,
    MPU6050Delay2MS = 2,
    MPU6050Delay1MS = 1,
    MPU6050NoDelay = 0,
}

pub enum MPUdhpfT  {
    MPU6050dhpfReset ,
    MPU6050dhpf5HZ ,
    MPU6050dhpf2_5HZ ,
    MPU6050dhpf1_25HZ ,
    MPU605dhpf0_63HZ ,
    MPU6050dhpfHold ,
}

pub enum MPUdlpfT {
    MPU6050dlpf6 ,
    MPU6050dlpf5 ,
    MPU6050dlpf4 ,
    MPU6050dlpf3 ,
    MPU6050dlpf2 ,
    MPU6050dlpf1 ,
    MPU6050dlpf0 ,
}

pub struct MPU6050{
    address: Volatile<u8>,
    i2c: i2c::Twi, 
    //atmega2560p::com::i2C::Twi::new(),
    //vec: FixedSliceVec<'a ,u8>,
}

impl MPU6050 {

    // pub fn new () {
        //     let Self = MPU6050 {
            //         i2c: atmega2560p::com::i2C::Twi::new(),
    //     };
    // }
    fn readregister(&mut self,reg: u8) -> u8 {
        let mut vec1 : FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        vec1.push(reg);
        self.i2c.read_from_slave(MPU6050_ADDRESS, 1, &mut vec1);
        return vec1[1];
    }
    
    fn writeregister(&mut self, reg: u8, value: u8){
        let mut vec2 : FixedSliceVec<u8> = FixedSliceVec::new(&mut []);
        vec2.push(reg);
        vec2.push(value);
        self.i2c.write_to_slave(MPU6050_ADDRESS, &vec2);
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
    
    
    pub fn SetDLPF(&mut self,dlpf: MPUdlpfT) {
        let mut value : u8;
        value = self.readregister(MPU6050_REG_CONFIG);
        value &= 0b11111000;
        value |= match dlpf {
            MPU6050dlpf6 => 0b110,
            MPU6050dlpf5 => 0b101,
            MPU6050dlpf4 => 0b100,
            MPU6050dlpf3 => 0b011,
            MPU6050dlpf2 => 0b010,
            MPU6050dlpf1 => 0b001,
            MPU6050dlpf0 => 0b000,
        }
    }
    
    pub fn SetDHPFMode(&mut self,dhpf: MPUdhpfT) {
        let mut value: u8;
        value = self.readregister(MPU6050_REG_CONFIG);
        value &= 0b11111100;
        value |= match dhpf {
            MPU6050dhpfReset => 0b000,
            MPU6050dhpf5HZ => 0b001,
            MPU6050dhpf2_5HZ => 0b010,
            MPU6050dhpf1_25HZ => 0b011,
            MPU6050dhpf0_63HZ => 0b100,
            MPU6050dhpfHold => 0b111,
        };
        self.writeregister(MPU6050_REG_CONFIG, value);
    }
    
    pub fn SetScale(&mut self , scale: MPUdpsT) {
        
    }
    
    pub fn GetScale(&mut self ) -> MPUdpsT {
        let mut value : u8;
        value = self.readregister(MPU6050_REG_GYRO_CONFIG);
        value &= 0b00011000;
        value >>= 3;
        match value {
            MPU6050Scale2000DPS => MPUdpsT::MPU6050Scale2000DPS,
            MPU6050Scale1000DPS => MPUdpsT::MPU6050Scale1000DPS,
            MPU6050Scale500DPS => MPUdpsT::MPU6050Scale500DPS,
            MPU6050Scale250DPS => MPUdpsT::MPU6050Scale250DPS,
        }
    }
    
    pub fn SetRange(&mut self , range: MPURangeT) {
        
    }
    
    pub fn GetRange(&mut self ) -> MPURangeT {
        let mut value : u8;
        value = self.readregister(MPU6050_REG_ACCEL_CONFIG);
        value &= 0b00011000;
        value >>= 3;
        match value {
            MPU6050Range2G => MPURangeT::MPU6050Range2G,
            MPU6050Range4G => MPURangeT::MPU6050Range4G,
            MPU6050Range8G => MPURangeT::MPU6050Range8G,
            MPU6050Range16G => MPURangeT::MPU6050Range16G,
        }
    }
    
    pub fn SetClockSource(&mut self , source: MPUClockSourceT) {
        
    }

    pub fn GetClockSource(&mut self ) -> MPUClockSourceT {
        let mut value : u8;
        value = self.readregister(MPU6050_REG_PWR_MGMT_1);
        value &= 0b00000111;
        match value {
            MPU6050ClockInternal8MHZ => MPUClockSourceT::MPU6050ClockInternal8MHZ,
            MPU6050ClockPllGyrox => MPUClockSourceT::MPU6050ClockPllGyrox,
            MPU6050ClockPllGyroy => MPUClockSourceT::MPU6050ClockPllGyroy,
            MPU6050ClockPllGyroz => MPUClockSourceT::MPU6050ClockPllGyroz,
            MPU6050ClockExternal32MHZ => MPUClockSourceT::MPU6050ClockExternal32MHZ,
            MPU6050ClockExternal19MHZ => MPUClockSourceT::MPU6050ClockExternal19MHZ,
            MPU6050ClockKeepReset => MPUClockSourceT::MPU6050ClockKeepReset,
        }
    }

    pub fn SetIntFreeFallEnabled(&mut self ,state: bool){
        self.writeregisterBit(MPU6050_REG_INT_ENABLE, 7, state);
    }

    pub fn GetIntFreeFallEnabled(&mut self) -> bool {
        let mut value = self.readregister(MPU6050_REG_INT_ENABLE);
        return get_bit(value,6);
    }

    pub fn SetAccelPowerOnDelay(){
        
    }

    pub fn GetAccelPowerOnDelay(){

    }

    pub fn SetMotionDetectionThreshold(&mut self,threshold: u8){
        self.writeregister(MPU6050_REG_MOT_THRESHOLD, threshold);
    }

    pub fn GetMotionDetectionThreshold(&mut self,) -> u8 {
        return self.readregister(MPU6050_REG_MOT_THRESHOLD);
    }

    pub fn SetMotionDetectionDuration(&mut self, duration: u8){
        self.writeregister(MPU6050_REG_MOT_DURATION, duration);
    }

    pub fn GetMotionDetectionDuration(&mut self,) -> u8 {
        return self.readregister(MPU6050_REG_MOT_DURATION);
    }

    pub fn SetZeroMotionDetectionThreshold(&mut self,threshold: u8){
        self.writeregister(MPU6050_REG_ZMOT_THRESHOLD, threshold);
    }

    pub fn GetZeroMotionDetectionThreshold(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_ZMOT_THRESHOLD);
    }

    pub fn SetZeroMotionDetectionDuration(&mut self, duration: u8){
        self.writeregister(MPU6050_REG_ZMOT_DURATION, duration);
    }

    pub fn GetZeroMotionDetectionDuration(&mut self)->u8{
        return self.readregister(MPU6050_REG_ZMOT_DURATION);
    }

    pub fn SetFreeFallDetectionThreshold(&mut self, threshold: u8){
        self.writeregister(MPU6050_REG_FF_THRESHOLD, threshold);
    }

    pub fn GetFreeFallDetectionThreshold(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_FF_THRESHOLD);
    }

    pub fn SetFreeFallDetectionDuration(&mut self, duration: u8){
        self.writeregister(MPU6050_REG_FF_DURATION, duration);
    }

    pub fn GetFreeFallDetectionDuration(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_FF_DURATION);
    }

    pub fn SetSleepEnabled(&mut self, state: bool){
        self.writeregisterBit(MPU6050_REG_PWR_MGMT_1, 6, state);
    }

    pub fn GetSleepEnabled(&mut self) -> bool{
        let mut value = self.readregister(MPU6050_REG_PWR_MGMT_1);
        return get_bit(value,6);
    }

    pub fn GetIntZeroMotionEnabled(&mut self) -> bool{
        let mut value = self.readregister(MPU6050_REG_INT_ENABLE);
        return get_bit(value,5);
    }

    pub fn SetIntZeroMotionEnabled(&mut self, state: bool){
        self.writeregisterBit(MPU6050_REG_INT_ENABLE, 5, state);
    }

    pub fn GetIntMotionEnabled(&mut self) -> bool{
       let mut value = self.readregister(MPU6050_REG_INT_ENABLE);
        return get_bit(value,6);
    }

    pub fn SetIntMotionEnabled(&mut self, state: bool){
        self.writeregisterBit(MPU6050_REG_INT_ENABLE, 6, state);
    }

    pub fn SetI2CMasterModeEnabled(&mut self, state: bool){
        self.writeregisterBit(MPU6050_REG_USER_CTRL, 5, state);
    }

    pub fn GetI2CMasterModeEnabled(&mut self) -> bool{
        let mut value = self.readregister(MPU6050_REG_USER_CTRL);
        return get_bit(value,5);
    }

    pub fn SetI2CByepassEnabled(&mut self, state: bool){
        self.writeregisterBit(MPU6050_REG_INT_PIN_CFG , 1, state); ;
    }

    pub fn GetI2CByepassEnabled(&mut self) -> bool{
        let mut value = self.readregister(MPU6050_REG_INT_PIN_CFG);
        return get_bit(value,1);
    }

    pub fn GetIntStatus(&mut self) -> u8 {
        return self.readregister(MPU6050_REG_INT_STATUS);
    }

    pub fn Calibrategyro(){
        
    }

    pub fn SetThreshold(){
        
    }

    pub fn GetThreshold() -> u8{
        return actualThreshold;
    }

}