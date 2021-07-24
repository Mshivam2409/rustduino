//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Sahil Aggarwal, Indian Institute of Technology Kanpur
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

use crate::avr::shift::*;
use crate::hal::port::*;
//use crate::hal::pin::*;
//use crate::{com, delay::delay_ms};

// makes pin struct given pin number
fn make_pin(pin: u8) -> Pin {
    match pin {
        0 => return Pin::new(PortName::D, 0).unwrap(),
        1 => return Pin::new(PortName::D, 1).unwrap(),
        2 => return Pin::new(PortName::D, 2).unwrap(),
        3 => return Pin::new(PortName::D, 3).unwrap(),
        4 => return Pin::new(PortName::D, 4).unwrap(),
        5 => return Pin::new(PortName::D, 5).unwrap(),
        6 => return Pin::new(PortName::D, 6).unwrap(),
        7 => return Pin::new(PortName::D, 7).unwrap(),

        8 => return Pin::new(PortName::B, 8).unwrap(),
        9 => return Pin::new(PortName::B, 9).unwrap(),
        10 => return Pin::new(PortName::B, 10).unwrap(),
        11 => return Pin::new(PortName::B, 11).unwrap(),
        12 => return Pin::new(PortName::B, 12).unwrap(),
        13 => return Pin::new(PortName::B, 13).unwrap(),

        _ => unreachable!(),
    }
}

pub fn setup(datapin: u8, clockpin: u8,latchpin:u8,decpt:bool,common_anode:bool,value:u8){
    let mut data = make_pin(datapin);
    let mut clock = make_pin(clockpin);
    let mut latch = make_pin(latchpin);

    data.set_pin_mode(IOMode::Output);
    latch.set_pin_mode(IOMode::Output);
    clock.set_pin_mode(IOMode::Output);

    out(datapin,clockpin,latchpin,decpt,common_anode,value);

}

pub fn myfnUpdateDisplay(datapin: u8, clockpin: u8,latchpin:u8,decpt:bool,common_anode:bool,value:u8){
    let mut latch = make_pin(latchpin);
    
    if common_anode {                      //if using a common anode display
        value = value ^ 0b11111111;  // then flip all bits using XOR 
    }
    latch.low();                        // prepare shift register for data
    shift_out(datapin,clockpin,BitOrder::LSBFIRST,value); // send data
    latch.high();                    //update display
}

pub fn out(datapin: u8, clockpin: u8,latchpin:u8,decpt:bool,common_anode:bool,value:u8){
    let mut bits:u8=myfnNumToBits(value);
    if decpt {
        bits = bits | 0b00000001;  // add decimal point if needed
    }
    myfnUpdateDisplay(datapin,clockpin,latchpin,decpt,common_anode,value);    // display alphanumeric digit
}

pub fn myfnNumToBits (somenumber:u8) -> u8 {
    if somenumber == 0{
        return 0b11111100;
    }else if somenumber==1{
        return 0b01100000;
    }else if somenumber==2{
        return 0b11011010;
    }else if somenumber==3{
        return 0b11110010;
    }else if somenumber==4{
        return 0b01100110;
    }else if somenumber==5{
        return 0b10110110;
    }else if somenumber==6{
        return 0b10111110;
    }else if somenumber==7{
        return 0b11100000;
    }else if somenumber==8{
        return 0b11111110;
    }else if somenumber==9{
        return 0b11110110;
    }else if somenumber==10{
        return 0b11101110;        //10=='A'
    }else if somenumber==11{
        return 0b00111110; // Hexidecimal B
    }else if somenumber==12{
        return 0b10011100; // Hexidecimal C or use for Centigrade
    }else if somenumber==13{
        return 0b01111010; // Hexidecimal D
    }else if somenumber==14{
        return 0b10011110; // Hexidecimal E
    }else if somenumber==15{
        return 0b10001110; // Hexidecimal F or use for Fahrenhei
    }else{
        return 0b10010010; // Error condition, displays three vertical bars
    }
}
