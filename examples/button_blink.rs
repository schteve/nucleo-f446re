#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nucleo_f446re::{led::LedDigital, Nucleo};
use panic_probe as _;

#[entry]
fn main() -> ! {
    let mut nucleo = Nucleo::<LedDigital>::init().unwrap();

    loop {
        if nucleo.user_button.is_pressed() {
            nucleo.user_led.on();
        } else {
            nucleo.user_led.off();
        }
    }
}
