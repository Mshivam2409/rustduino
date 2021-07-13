//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Nikhil Gupta,Indian Institute of Technology Kanpur
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

#![no_std]
#![no_main]
#![deny(warnings)]

/// Crates included which are to be used for the Blinking LED example.
use rustduino::atmega2560p::hal::pin::Pins;
use rustduino::atmega2560p::hal::watchdog::WatchDog;

#[no_mangle]
pub extern "C" fn main() {
    // Disable watchdog
    let watchdog = unsafe { WatchDog::new() };
    watchdog.disable();

    // This pins represents pin 7 of port B ( pin 13).
    let mut pins = Pins::new();

    //This sets pin 7 of port B (pin 13) as output.
    pins.digital[13].output();

    loop {
        //This sets pin high.
        pins.digital[13].high();

        rustduino::delay::delay_ms(1000);
        //This sets pin as low.
        pins.digital[13].low();

        rustduino::delay::delay_ms(1000);
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}