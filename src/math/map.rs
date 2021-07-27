// RustDuino : A generic HAL implementation for Arduino Boards in Rust
// Copyright (C) 2021 Devansh Kumar Jha, Indian Institute of Technology Kanpur
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>

//! This code implements a mathematical data mapping function for a given value.

/// Function used to map a number from one scale to the other scale accordingly.
/// # Arguments
/// * `val` - a u64, the value which is to be mapped to any other data type.
/// * `in_min` - a u64, the minimum value possible in the data type of `val`.
/// * `in_max` - a u64, the maximum value possible in the data type of `val`.
/// * `out_min` - a u64, the minimum value possible in the required data type for `val`.
/// * `out_max` - a u64, the maximum value possible in the required data type for `val`.
/// # Returns
/// * `a u64` - the final value of `val` in the required data type.
/// # Usage
/// * rustduino::math::map::map(...)
pub fn map(val: u64, in_min: u64, in_max: u64, out_min: u64, out_max: u64) -> u64 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
