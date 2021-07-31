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

mod map;

pub use map::*;
pub use micromath::*;

cfg_if::cfg_if! {
    if #[cfg(feature = "random")] {
        mod random;

        /// For more information
        /// `<https://gist.github.com/bloc97/b55f684d17edd8f50df8e918cbc00f94>`
        pub use random::*;
    }
}
