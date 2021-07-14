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


/// Crates which would be used in the implementation.
use bit_field::BitField;
use core;
use volatile::Volatile;


pub struct Twi{
    twbr:Volatile<u8>,
    twcr:Volatile<u8>,
    twsr:Volatile<u8>,
    twdr:Volatile<u8>,
    twar:Volatile<u8>,
    twamr:Volatile<u8>,
}

/// ## TWCR register's bits definitions
static TWINT: u8 = 0;
static TWEA: u8 = 1;
static TWSTA: u8 = 2;
static TWSTO: u8 = 3;
static TWWC: u8 = 4;
static TWEN: u8 = 5;
static TWIE: u8 = 7;

//TWSR bits
const TWPS1:u8=6;
const TWPS0:u8=7;





