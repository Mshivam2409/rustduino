//     RustDuino : A generic HAL implementation for Arduino Boards in Rust
//     Copyright (C) 2021  Tulika Shukla, Indian Institute of Technology Kanpur
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

//! Various pins and ports in the ATMEGA2560P chip is controlled here.
//! Section 13.2 to 13.4 of ATMEGA2560P datasheet.
//! https://ww1.microchip.com/downloads/en/devicedoc/atmel-2549-8-bit-avr-microcontroller-atmega640-1280-1281-2560-2561_datasheet.pdf

/// Core Crate functions required in the code for reading and writing to registers.
use core::{
    ptr::{read_volatile, write_volatile},
    usize,
};

/// Represents the name of the ports in ATMEGA2560P , can vary from A-L leaving I.
#[derive(Clone, Copy)]
pub enum PortName {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    J,
    K,
    L,
}

/// These will control the ports ( set of 8 pins each controlled by a bit ).
/// `DDR:  Data direction register`
///     This controls the direction of a particular pin.
///     Each bit of this register decides the I/O state of a particular pin on the microcontroller IC.
/// `PORT: Data Register`
///     Used when the particular pin is set to output.
///     It will give the value of digital input/output sent by the pin.
/// `PIN:  Port input pins`
///     This can be read to see the value at a particualar pin.
///     It is also used as a toggle controller.     
pub struct Port {
    pub pin: u8,
    pub ddr: u8,
    pub port: u8,
}

/// The structure Pin contains the address of the port to which the pin belongs and the pin's number.
pub struct Pin {
    port: *mut Port,
    pin: usize,
}

/// Type `IOMode`
/// Represents the Input/Output mode of the pin.
pub enum IOMode {
    Input,
    Output,
}

impl Port {
    /// Returns mutable reference to the Port of given PortName.
    pub fn new(name: PortName) -> &'static mut Port {
        match name {
            PortName::A => unsafe { &mut *(0x20 as *mut Port) },
            PortName::B => unsafe { &mut *(0x23 as *mut Port) },
            PortName::C => unsafe { &mut *(0x26 as *mut Port) },
            PortName::D => unsafe { &mut *(0x29 as *mut Port) },
            PortName::E => unsafe { &mut *(0x2C as *mut Port) },
            PortName::F => unsafe { &mut *(0x2F as *mut Port) },
            PortName::G => unsafe { &mut *(0x32 as *mut Port) },
            PortName::H => unsafe { &mut *(0x100 as *mut Port) },
            PortName::J => unsafe { &mut *(0x103 as *mut Port) },
            PortName::K => unsafe { &mut *(0x106 as *mut Port) },
            PortName::L => unsafe { &mut *(0x109 as *mut Port) },
        }
    }

    /// Returns PortName of port of the given address input.
    /// Panics if the address is invalid.
    pub fn name(&self) -> PortName {
        let address = (self as *const Port) as usize; // Gets address of port.
        match address {
            //  Return PortName based on the address read.
            0x20 => PortName::A,
            0x23 => PortName::B,
            0x26 => PortName::C,
            0x29 => PortName::D,
            0x2C => PortName::E,
            0x2F => PortName::F,
            0x32 => PortName::G,
            0x100 => PortName::H,
            0x103 => PortName::J,
            0x106 => PortName::K,
            0x109 => PortName::L,
            _ => unreachable!(),
        }
    }

    /// Returns a `Some<Pin>` if pin number is valid and returns none if not valid.
    pub fn pin(&mut self, pin: usize) -> Option<Pin> {
        if pin < 0x8 {
            Some(Pin { port: self, pin })
        } else {
            None
        }
    }

    /// Change pin mode to input or output by changing the DDr register.
    pub fn set_pin_mode(&mut self, mode: IOMode, pin_no: usize) {
        //  Read the value of DDxn register.
        let mut ddr_val = unsafe { read_volatile(&mut self.ddr) };

        //  Calculate the value to be written to DDxn register.
        //  This will set the register according to the mode in which the pin is to be set.
        ddr_val &= !(0x1 << pin_no);
        ddr_val |= match mode {
            IOMode::Input => 0x0,
            IOMode::Output => 0x1 << pin_no,
        };

        // Write the value to DDxn register.
        unsafe { write_volatile(&mut self.ddr, ddr_val) }
    }

    /// Toggles the appropriate bit in PINxn register so that the mode of the pin
    /// is changed from high to low or vice versa.
    pub fn toggle(&mut self, pin_no: usize) {
        unsafe { write_volatile(&mut self.pin, 0x1 << pin_no) }
    }

    /// Set the pin to high output value.
    pub fn high(&mut self, pin_no: usize) {
        // Checks if pin number is valid.
        if pin_no >= 8 {
            return;
        }
        let mut p = unsafe { read_volatile(&mut self.port) }; // Reading the value of PORTxn.
        p = p & (1 << pin_no);
        let ddr_value = unsafe { read_volatile(&mut self.ddr) }; // Read the DDRxn register.
        if p == 0 && ddr_value == (0x1 << pin_no) {
            // Toggling the value of PORTxn, if it isn't set to high.
            self.toggle(pin_no);
        }
    }

    /// Sets the pin to low output value.
    pub fn low(&mut self, pin_no: usize) {
        // Check if pin number is valid.
        if pin_no >= 8 {
            return;
        }
        let mut p = unsafe { read_volatile(&mut self.port) }; //Reading the value of PORTxn.
        p = p & (1 << pin_no);
        let ddr_value = unsafe { read_volatile(&mut self.ddr) }; // Read the DDRxn register.
        if p != 0 && ddr_value == (0x1 << pin_no) {
            //Toggling the value of PORTxn, if it isn't set to low.
            self.toggle(pin_no);
        }
    }

    /// Change pin mode to Output by changing the value of DDxn register.
    pub fn output(&mut self, pinno: usize) {
        self.set_pin_mode(IOMode::Output, pinno);
    }

    /// Change pin mode to Input by changing the value of DDxn register.
    pub fn input(&mut self, pinno: usize) {
        self.set_pin_mode(IOMode::Input, pinno);
    }
}

impl Pin {
    ///Return a pin for the given port name and pin number.
    pub fn new(port: PortName, pin: usize) -> Option<Pin> {
        Port::new(port).pin(pin)
    }

    /// Change pin mode to input or output by changing the DDr register.
    pub fn set_pin_mode(&mut self, mode: IOMode) {
        //  Read the value of DDxn register.
        let mut ddr_val = unsafe { read_volatile(&mut (*self.port).ddr) };

        //  Calculate the value to be written to DDxn register.
        //  This will set the register according to the mode in which the pin is to be set.
        ddr_val &= !(0x1 << self.pin);
        ddr_val |= match mode {
            IOMode::Input => 0x0,
            IOMode::Output => 0x1 << self.pin,
        };

        // Write the value to DDxn register.
        unsafe { write_volatile(&mut (*self.port).ddr, ddr_val) }
    }

    /// Toggles the appropriate bit in PINxn register so that the mode of the pin
    /// is changed from high to low or vice versa.
    pub fn toggle(&mut self) {
        unsafe { write_volatile(&mut (*self.port).pin, 0x1 << self.pin) }
    }

    /// Set the pin to high output value.
    pub fn high(&mut self) {
        // Checks if pin number is valid.
        if self.pin >= 8 {
            return;
        }
        let mut p = unsafe { read_volatile(&mut (*self.port).port) }; // Reading the value of PORTxn.
        p = p & (1 << self.pin);
        let ddr_value = unsafe { read_volatile(&mut (*self.port).ddr) }; // Read the DDRxn register.
        if p == 0 && ddr_value == (0x1 << self.pin) {
            // Toggling the value of PORTxn, if it isn't set to high.
            self.toggle();
        }
    }

    /// Sets the pin to low output value.
    pub fn low(&mut self) {
        // Check if pin number is valid.
        if self.pin >= 8 {
            return;
        }
        let mut p = unsafe { read_volatile(&mut (*self.port).port) }; //Reading the value of PORTxn.
        p = p & (1 << self.pin);
        let ddr_value = unsafe { read_volatile(&mut (*self.port).ddr) }; // Read the DDRxn register.
        if p != 0 && ddr_value == (0x1 << self.pin) {
            //Toggling the value of PORTxn, if it isn't set to low.
            self.toggle();
        }
    }

    /// Change pin mode to Output by changing the value of DDxn register.
    pub fn output(&mut self) {
        self.set_pin_mode(IOMode::Output);
    }

    /// Change pin mode to Input by changing the value of DDxn register.
    pub fn input(&mut self) {
        self.set_pin_mode(IOMode::Input);
    }
}
