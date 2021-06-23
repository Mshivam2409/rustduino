use rustduino::atmega328p::hal as arduino_uno;

fn main() {
    let pins = arduino_uno::pins::Pins::get();

    pins.digital[13].set_output();

    loop {
        pins.digital[13].high();

        avr_delay::delay_ms(1000);

        pins.digital[13].low();

        avr_delay::delay_ms(1000);
    }
}
