#![no_std]
#![no_main]
#![deny(warnings)]

/// Crates included to show the Transmission of String example.
use rustduino::com::serial::Serial;

#[no_mangle]
pub fn main() {
    // Create a new Serial struct to access all USART's
    let serial = Serial::serial_new();

    // This initializes USART0 and makes it ready to transmit and recieve.
    serial.usart[0].begin_set_baud(9600);

    // Loop to send a string using the USART multiple times.
    let mut i: u8 = 10;
    while i!=0 {
        // This sends string from arduino through TxD0 pin.
        serial.usart[0].write_string("Hello World!");

        rustduino::delay::delay_ms(1000);

        i = i - 1;
    }

    // This disables USART0.
    serial.usart[0].end();
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
