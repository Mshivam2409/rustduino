//! General Digital I/O Implementation.

use core::ptr::{read_volatile, write_volatile};

/// Represents name of Port, can be either B, C, or D.
pub enum PortName {
    B,
    C,
    D,
}

/// Contains registers to control the port.
///
/// * `pin`: *Port input pins*. Writing a logic one to PINxn toggles the value of
/// PORTxn, independent on the value of DDRxn.
///
/// * `ddr`: *Data direction register*. The DDxn bit in the DDRx register selects the
/// direction of this pin. If DDxn is written logic one, Pxn is configured as
/// an output pin. If DDxn is written logic zero, Pxn is configured as an input pin.
///
/// * `port`: *Data register*. If PORTxn is written logic one when the pin is
/// configured as an input pin, the pull-up resistor is activated. To switch the
/// pull-up resistor off, PORTxn has to be written logic zero or the pin has to be
/// configured as an output pin. The port pins are tri-stated when reset condition
/// becomes active, even if no clocks are running.
///
///   If PORTxn is written logic one when the pin is configured as an output pin,
/// the port pin is driven high (one). If PORTxn is written logic zero when the pin
/// is configured as an output pin, the port pin is driven low (zero).
///
/// Section 13.2.1 and 13.2.2 of ATmega328P datasheet.
pub struct Port {
    pub pin: u8,
    pub ddr: u8,
    pub port: u8,
}

impl Port {
    /// Returns mutable reference to the `Port` given `PortName`.
    ///
    /// Section 13.4 of ATmega328P datasheet.
    pub fn new(port_name: PortName) -> &'static mut Port {
        unsafe {
            &mut *match port_name {
                PortName::B => 0x23 as *mut Port,
                PortName::C => 0x26 as *mut Port,
                PortName::D => 0x29 as *mut Port,
            }
        }
    }

    /// Returns PortName of the port based on its address.
    /// Panics if Port has invalid address.
    ///
    /// Section 13.4 of ATmega328P datasheet.
    pub fn name(&self) -> PortName {
        // Get address of port as usize.
        let addr = (self as *const Port) as usize;

        // Return PortName based on address.
        match addr {
            0x23 => PortName::B,
            0x26 => PortName::C,
            0x29 => PortName::D,
            _ => unreachable!(),
        }
    }
}

/// Represents a single `Pin`.
///
/// The struct contains reference to a `Port` under which the pin belong
/// and the pin number.
///
/// Section 13.4 of ATmega328P datasheet.
pub struct Pin {
    port: *mut Port,
    pin: u8,
}

/// The `IOMode` type. Represents the I/O mode for a pin.
pub enum IOMode {
    Input,
    Output,
}

impl Port {
    /// Returns a `Some<Pin>` if pin number is valid.
    pub fn pin(&mut self, pin: u8) -> Option<Pin> {
        if pin < 0x8 {
            Some(Pin { port: self, pin })
        } else {
            None
        }
    }
}

impl Pin {
    /// Return a pin given port_name and pin number
    pub fn new(port_name: PortName, pin: u8) -> Option<Pin> {
        Port::new(port_name).pin(pin)
    }

    /// Change pin mode to input or output by changing the DDR bit
    /// of that pin to 0 and 1 respectively.
    ///
    /// `io_mode` can be either `IOMode::Input` or `IOMode::Output`.
    ///
    /// Section 13.2 of ATmega328P datasheet.
    pub fn set_mode(&mut self, io_mode: IOMode) {
        // Check if pin number is valid
        if self.pin >= 8 {
            return;
        }

        // Read the DDRxn register.
        let mut ddr_val = unsafe { read_volatile(&mut (*self.port).ddr) };

        // Calculate the value to write to DDRxn register.
        ddr_val &= !(0x1 << self.pin);

        ddr_val |= match io_mode {
            IOMode::Input => 0x0,
            IOMode::Output => 0x1 << self.pin,
        };

        // Write the value to DDRxn register.
        unsafe { write_volatile(&mut (*self.port).ddr, ddr_val) }
    }

    /// Toggles value of PORTxn, independent of value of DDRxn.
    pub fn toggle(&mut self) {
        // Check if pin number is valid
        if self.pin >= 8 {
            return;
        }

        // Set the bit at offset self.pin in PINxn register
        unsafe { write_volatile(&mut (*self.port).pin, 0x1 << self.pin) }
    }

    /// Set pin to high.
    ///
    /// This function checks if the pin is already high or not by reading
    /// PINxn register. If it is not high then it calls `Pin::toggle`.
    pub fn high(&mut self) {
        // Check if pin number is valid.
        if self.pin >= 8 {
            return;
        }

        // Get value of PORTxn register
        let port_val = unsafe { read_volatile(&mut (*self.port).port) };

        // Check if value of PORTxn is already high, toggle if it isn't.
        if port_val & (1 << self.pin) == 0 {
            self.toggle();
        }
    }

    /// Set pin to low.
    ///
    /// This function checks if the pin is already low or not by reading
    /// PINxn register. If it is not low then it calls `Pin::toggle`.
    pub fn low(&mut self) {
        // Check if pin number is valid.
        if self.pin >= 8 {
            return;
        }

        // Get value of PORTxn register
        let port_val = unsafe { read_volatile(&mut (*self.port).port) };

        // Check if value of PORTxn is already low, toggle if it isn't.
        if port_val & (1 << self.pin) != 0 {
            self.toggle();
        }
    }

    /// Change pin mode to output by changing the DDR bit of that pin to 1.
    ///
    /// Section 13.2 of ATmega328P datasheet.
    pub fn set_output(&mut self) {
        self.set_mode(IOMode::Output);
    }

    pub fn set_input(&mut self) {
        self.set_mode(IOMode::Input);
    }

    pub fn read(&mut self) -> u8 {
        let port_val = unsafe { read_volatile(&mut (*self.port).port) };

        if port_val & (1 << self.pin) == 0 {
            return 0;
        } else {
            return 1;
        }
    }
}
