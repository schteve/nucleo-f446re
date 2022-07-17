#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use nucleo_f446re::{led::LedDigital, Nucleo};
use panic_probe as _;

#[entry]
fn main() -> ! {
    let mut nucleo = Nucleo::<LedDigital>::init().unwrap();
    // Can also use the LED default generic argument with <Nucleo>::init().unwrap();

    loop {
        nucleo.user_led.toggle();
        nucleo.delay.delay_ms(500_u32);
    }
}
