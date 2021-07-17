//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Aniket Sharma, Indian Institute of Technology Kanpur
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


use bit_field::BitField;
use core;
use crate::rustduino::hal::interrupts;
use volatile::Volatile;
use crate::rustduino::atmega2560p::com::{usart_initialize,usart_initialize::Usart};

/// to select the data length to be transmitted
#[derive(Clone, Copy)]
pub enum datalen {
    bit5,
    bit6,
    bit7,
    bit8,
    bit9,
}                  

impl Usart{

    // initialization setting begin function 

    ///This function is to enable the Transmitter
    ///Once it is enabled it takes control of the TXDn pin as a transmitting output   
    pub fn Transmit_enable(&mut self) {
        unsafe {

            self.ucsrb.set_bit(3,true);

        }               
    }  

    /// storing data in Transmit Buffer it takes parameter as a u32 and and data bit length
    pub fn Transmitting_data (&self,data: Volatile<u32>,Len: datalen) {
        unsafe{
            
            /// checks if the Transmit buffer is empty to receive data
            while !(avai_write()) {};

            match Len{

                Len::bit5 =>self.udr.set_bits(0..5, data.get_bits(0..5)),
                Len::bit6 =>self.udr.set_bits(0..6, data.get_bits(0..6)),
                Len::bit7 =>self.udr.set_bits(0..7, data.get_bits(0..7)),
                Len::bit8 =>self.udr.set_bits(0..8, data.get_bits(0..8)),
                Len::bit9 =>{

                    self.ucsrb.update(|ctrl| {
                        ctrl.set_bit(0, data.get_bit(8)); 
                    });
                    
                    self.udr.set_bits(0..8,data.get_bits(0..8));

                }
            }
        }
    }  

    pub fn begin(&mut self, baud: i64, mode:UsartModes, stop : UsartStop, size : UsartDataSize, parity : UsartParity){
        
        initialize(&mut self, mode, baud, stop, size, parity);
        Transmit_enable(); 
        recieve_enable();

    }
    
    pub fn end(){

        Transmit_disable();
        recieve_disable();
    }
    ///This function tells if you can write in transmit buffer or not by checking UDREn
    /// if UDREn bit is set means you transmit buffer is empty and ready to receive data 
    pub fn avai_write(&mut self) -> bool{
        
        unsafe {
            let mut ucsrc =read_volatile(&self.ucsrc);
            
            if ucsrc.get_bit(5) {
                true
            }
            else {
                false
            }
        }
    }
    
    ///This functions waits for the transmission to complete by checking TXCn bit in the ucsrna register
    ///TXCn is set 1 when the transmit is completed and it can start transmitting new data 
    pub fn flush(&mut self){
        let ucsra = read_volatile(&self.ucsrc);

        while !(ucsra.get_bit(6)) {
            let ucsra = read_volatile(&self.ucsra);
        };
    }

    ///This enables parity generator for the frame 
    pub fn parity_enable(&mut self){
        unsafe{
            self.ucsrc.set_bit(5,true); 
        }
    }
     
    ///This disables the parity generator for the frame 
    pub fn parity_disable(&mut self){
         
        unsafe{
            self.ucsrc.set_bit(5,false); 
        }

    }

    ///This function is used to disable the Transmitter and once disabled the TXDn pin is no longer
    ///used as the transmitter output pin and functions as a normal I/O pin
    pub fn Transmit_disable(&mut self) {

         let uscra6=git_bit(&self.uscra,6);
         let uscra5=get_bit(&self.uscra,5);

        ///check for data in Transmit Buffer and Tansmit shift register,
        ///if data is present in either then disabling of transmitter is not effective
        while !(uscra6 & uscra5) {

            let uscra6=git_bit(&self.uscra,6);
            let uscra5=get_bit(&self.uscra,5);

        };
        
        unsafe{

        self.ucsrb.set_bit(3,false);
        }
    }  
    
    ///This function sends a character byte of 5,6,7 or 8 bits
    pub fn Transmit_data (&self,data: Volatile<u8>) {
        unsafe{
            let ucsra = read_volatile(&self.ucsra) ;
            let udre = ucsra.get_bit(5);

            while ( !( udre)) {
                let ucsra = read_volatile(&self.ucsra) ;
                let udre = ucsra.get_bit(5);
            };
              self.udr.write(data);
                
        }
    }

    ///This function send data type of string byte by byte
    pub fn write(&mut self,data:String){
        self.Transmit_enable();
      for b in data.byte(){
          self.Transmit_data(b);
      }
      self.Transmit_disable();
    } 
   pub fn write(&mut self,data:u32){
       let mut v=Vec::new();
   }
     
}