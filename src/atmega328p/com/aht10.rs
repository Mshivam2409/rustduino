use bit_field::BitField;
use crate::delay::delay_ms;
use volatile::Volatile;
use fixed_slice_vec::FixedSliceVec;


pub enum Temp_sensor {
    AHT10_SENSOR ,
}


pub struct AHT10 {
    address: Volatile<u8>,
    twi : Twi,
}

const AHT10_ADDRESS_0X38  :u8= 0x38;      //chip I2C address no.1 for AHT10/AHT15/AHT20, address pin connected to GND
const AHT10_ADDRESS_0X39 :u8=0x39;         //chip I2C address no.2 for AHT10 only, address pin connected to Vcc

const AHT10_INIT_CMD :u8= 0xE1 ;              //initialization command for AHT10/AHT15
const  AHT20_INIT_CMD :u8=0xBE ;              //initialization command for AHT20
const  AHT10_START_MEASURMENT_CMD :u8= 0xAC;  //start measurment command
const  AHT10_NORMAL_CMD: u8=0xA8;             //normal cycle mode command, no info in datasheet!!!
const  AHT10_SOFT_RESET_CMD :u8= 0xBA;        //soft reset command

const  AHT10_INIT_NORMAL_MODE : u8=   0x00;  //enable normal mode
const  AHT10_INIT_CYCLE_MODE  : u8=  0x20;  //enable cycle mode
const  AHT10_INIT_CMD_MODE    : u8=  0x40;  //enable command mode
const  AHT10_INIT_CAL_ENABLE  : u8=  0x08;  //load factory calibration coeff
const  AHT10_INIT_BUSY        : u8=  0x08;

const  AHT10_DATA_MEASURMENT_CMD  :u8=0x33;  //no info in datasheet!!! my guess it is DAC resolution, saw someone send 0x00 instead
const  AHT10_DATA_NOP             :u8=0x00; //no info in datasheet!!!


const  AHT10_MEASURMENT_DELAY   :u8= 80;    //at least 75 milliseconds
const  AHT10_POWER_ON_DELAY     :u8= 40;    //at least 20..40 milliseconds
const  AHT10_CMD_DELAY          :u32=350;   //at least 300 milliseconds, no info in datasheet!!!
const  AHT10_SOFT_RESET_DELAY   :u8=20;     //less than 20 milliseconds

const  AHT10_FORCE_READ_DATA     :bool= true;  //force to read data
const  AHT10_USE_READ_DATA       :bool= false; //force to use data from previous read
const AHT10_ERROR                :u8=0xFF ;   //returns 255, if communication error is occurred



     
//let mut address=AHT10_ADDRESS_0X38; //isko bhi global karna hoga kya
impl AHT10 {

    pub fn new()-> &'static mut Self {
        delay_ms(20); //20ms delay to wake up
        //let mut address=AHT10_ADDRESS_0X38;//isko bhi global karna hoga kya
        self.soft_reset();  // not sure on this

        if !self.intialise(){
            !panic("Could not intialise!");
        }
    unsafe { &mut *(0x38 as *mut Self) }
    }                  

    pub fn soft_reset(&mut self) {
        let mut vec : FixedSliceVec<u8> = FixedSliceVec::new(&mut []);  
        let dum1 :u8;
        vec=vec![dum1,AHT10_SOFT_RESET_CMD];
        if !write_to_slave(address,&vec){
            !panic("Error!");
        }                                  //yeh bool return kar raha hai...ese likhna sahi hoga kya ?
        delay_ms(20);
        
    }

    pub fn initialise(&mut self) -> bool{
        let mut vec : FixedSliceVec<u8> = FixedSliceVec::new(&mut []);  
        vec=vec![u8,AHT10_INIT_CMD,0x08,0x00];
        if !write_to_slave(address,&vec){
            !panic("error!");
        }
        wait_for_idle();
        if self.status()!=AHT10_INIT_CAL_ENABLE{//yeh python vale code se check karne ek bar ...not sure mene sahi ki hu ya nhi
            return False;
        }
        return True;
    }

    pub fn read_to_buffer(&mut self){
        if !read_from_slave(address,&vec){
            !panic("Error!");
        }
    }

    pub fn trigger_slave(&mut self){
        let mut vec : FixedSliceVec<u8> = FixedSliceVec::new(&mut []);  
        vec=vec![u8,AHT10_START_MEASURMENT_CMD,0x33,0x00];//start measurement
        if !write_to_slave(address,&vec){
            !panic("Error!");
        }
    }
    //yeh python vale code se check karne ek bar ...not sure mene sahi ki hu ya nhi
    pub fn wait_for_idle(&mut self){
        while self.status()==AHT10_INIT_BUSY{
            delay_ms(5);
        }

        delay_ms(5);
    }

    pub fn perform_measurement(&mut self){
        self.trigger_slave();
        self.wait_for_idle();
        self.read_to_buffer();
    }
    pub fn status(&mut self){
        self.read_to_buffer();
        return(vec[0]);
    }

    pub fn realative_humidity(&mut self){
        self.perform_measurement();
        let mut humid:u32 =(vec[1]<<12)|(vec[2]<<4)|(vec[3]>>4);
        humid =(humid*100)/0x100000;
        return humid;
    }

    pub fn temperature(&mut self){
        self.perform_measurement();
        let mut temp:u32=((vec[3]&0xF)<<16)|vec[4]<<8|vec[5];
        temp=((temp*200.0)/0x100000)-50;
        return temp;
    }   
}