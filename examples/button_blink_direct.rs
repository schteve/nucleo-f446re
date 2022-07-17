#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nucleo_f446re::{button::Button, led::LedDigital};
use panic_probe as _;
use stm32f4xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    let dp = stm32f4xx_hal::pac::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();

    let mut user_led = LedDigital::new(gpioa.pa5);
    let user_button = Button::new(gpioc.pc13);

    loop {
        if user_button.is_pressed() {
            user_led.on();
        } else {
            user_led.off();
        }
    }
}
