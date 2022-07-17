#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;
use nucleo_f446re::led::LedAnalog;
use panic_probe as _;
use stm32f4xx_hal::prelude::*;

#[entry]
fn main() -> ! {
    let dp = stm32f4xx_hal::pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    let gpioa = dp.GPIOA.split();

    let mut user_led = LedAnalog::new(gpioa.pa5, dp.TIM2, &clocks);
    let mut delay = cp.SYST.delay(&clocks);

    let mut pct = 0;
    let mut up = true;
    loop {
        user_led.set_duty(pct);

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
        delay.delay_ms(10_u32);
    }
}
