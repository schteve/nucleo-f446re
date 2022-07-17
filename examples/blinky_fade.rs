#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use nucleo_f446re::{led::LedAnalog, Nucleo};
use panic_probe as _;

#[entry]
fn main() -> ! {
    let mut nucleo = Nucleo::<LedAnalog>::init().unwrap();

    let mut pct = 0;
    let mut up = true;
    loop {
        nucleo.user_led.set_duty(pct);

        if up == true {
            pct += 1;
            if pct >= 100 {
                up = false;
            }
        } else {
            pct -= 1;
            if pct == 0 {
                up = true;
            }
        }
        nucleo.delay.delay_ms(10_u32);
    }
}
