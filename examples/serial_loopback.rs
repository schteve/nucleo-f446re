#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nucleo_f446re::{Nucleo, led::LedDigital};
use panic_probe as _;
use stm32f4xx_hal::{block, prelude::*};

#[entry]
fn main() -> ! {
    let mut nucleo = Nucleo::<LedDigital>::init().unwrap();

    loop {
        if let Ok(c) = block!(nucleo.vcom.rx.read()) {
            let _ = block!(nucleo.vcom.tx.write(c));
            nucleo.user_led.toggle();
        }
    }
}
