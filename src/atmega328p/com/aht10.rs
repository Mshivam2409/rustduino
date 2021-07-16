use bit_field::BitField;
use crate::delay::delay_ms;
use volatile::Volatile;


pub enum Temp_sensor {
    AHT10_SENSOR = 0x00;
}

const AHT10_ADDRESS_0X38  u8: 0x38       //chip I2C address no.1 for AHT10/AHT15/AHT20, address pin connected to GND
const AHT10_ADDRESS_0X39 u8:0x39         //chip I2C address no.2 for AHT10 only, address pin connected to Vcc

const AHT10_INIT_CMD u8: 0xE1               //initialization command for AHT10/AHT15
const  AHT20_INIT_CMD u8:0xBE               //initialization command for AHT20
const  AHT10_START_MEASURMENT_CMD u8: 0xAC  //start measurment command
const  AHT10_NORMAL_CMD u8:0xA8             //normal cycle mode command, no info in datasheet!!!
const  AHT10_SOFT_RESET_CMD u8: 0xBA        //soft reset command

const  AHT10_INIT_NORMAL_MODE  u8:   0x00  //enable normal mode
const  AHT10_INIT_CYCLE_MODE   u8:  0x20  //enable cycle mode
const  AHT10_INIT_CMD_MODE     u8:  0x40  //enable command mode
const  AHT10_INIT_CAL_ENABLE   u8:  0x08  //load factory calibration coeff


const  AHT10_DATA_MEASURMENT_CMD  u8:0x33  //no info in datasheet!!! my guess it is DAC resolution, saw someone send 0x00 instead
const  AHT10_DATA_NOP             u8:0x00  //no info in datasheet!!!


const  AHT10_MEASURMENT_DELAY    u8: 80    //at least 75 milliseconds
const  AHT10_POWER_ON_DELAY      u8: 40    //at least 20..40 milliseconds
const  AHT10_CMD_DELAY           u32:350   //at least 300 milliseconds, no info in datasheet!!!
const  AHT10_SOFT_RESET_DELAY    u8:20     //less than 20 milliseconds

const  AHT10_FORCE_READ_DATA     bool: true  //force to read data
const  AHT10_USE_READ_DATA       bool: false //force to use data from previous read
const AHT10_ERROR                u8: 0xFF    //returns 255, if communication error is occurred



        
pub fn initialize(self) {
    delay_ms(20); //20ms delay to wake up
    let mut address=AHT10_ADDRESS_0X38;
    let mut buf:[u8,8];//isko initialise krna hoga kya

}                  

pub fn soft_reset(self){
    self.buf[0].write(AHT10_SOFT_RESET_CMD);
    delay_ms(20);
}
