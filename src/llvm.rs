// RustDuino : A generic HAL implementation for Arduino Boards in Rust
// Copyright (C) 2021 Shivam Malhotra, Indian Institute of Technology Kanpur

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>

/// The `__nop` function is equivalent to the NOP machine instruction.
/// A NOP is a computer instruction that does nothing and still takes fixed clock cyles to process.
pub fn __nop() {
    unsafe { llvm_asm!("nop") }
}
