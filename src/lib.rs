#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![no_std]

/// Work with the on-board user button
pub mod button;
/// Work with the on-board user LED
pub mod led;
/// Work with the virtual serial port through the on-board ST-Link
pub mod serial;

use self::{
    button::Button,
    led::{LedBuilder, LedDigital},
    serial::SerialPort,
};
use stm32f4xx_hal::{prelude::*, timer::SysDelay};

/// The batteries-included way to work with the Nucleo board.
///
/// This struct is marked as `non_exhaustive` to make it unconstructable, thereby forcing the user to go through init().
#[non_exhaustive]
pub struct Nucleo<LED = LedDigital> {
    /// The user LED.
    pub user_led: LED,
    /// The user button.
    pub user_button: Button,
    /// The virtual serial port through the on-board ST-Link debugger.
    pub vcom: SerialPort,
    /// Gives the ability to delay (blocking) with millisecond resolution.
    pub delay: SysDelay,
}

impl<LED: LedBuilder> Nucleo<LED> {
    /// Initialize the Nucleo board.
    ///
    /// Use this if you don't have any other hardware needs, since the peripheral struct is taken and dropped
    /// internally and can't be used elsewhere. After the first call, always returns None.
    ///
    /// The LED generic parameter allows the user to specify what type of LED should control the LED hardware.
    #[must_use]
    pub fn init() -> Option<Self> {
        if let (Some(dp), Some(cp)) = (
            stm32f4xx_hal::pac::Peripherals::take(),
            cortex_m::peripheral::Peripherals::take(),
        ) {
            let rcc = dp.RCC.constrain();
            // Let's run as fast as we can, why not?
            let clocks = rcc
                .cfgr
                .use_hse(8.MHz()) // Per UM1724 the ST-Link MCO is used as HSE on OSC_IN, fixed at 8 MHz
                .bypass_hse_oscillator() // Bypass since it's not coming from a crystal
                .sysclk(180.MHz()) // 180 MHz is max SYSCLK
                .hclk(180.MHz()) // 180 MHz is max HCLK
                .freeze();

            let gpio_a = dp.GPIOA.split();
            let gpio_c = dp.GPIOC.split();

            let user_led = LED::build(gpio_a.pa5, dp.TIM2, &clocks);
            let user_button = Button::new(gpio_c.pc13);
            let vcom =
                SerialPort::new(gpio_a.pa2, gpio_a.pa3, dp.USART2, &clocks, 9600.bps()).ok()?;
            let delay = cp.SYST.delay(&clocks);

            Some(Self {
                user_led,
                user_button,
                vcom,
                delay,
            })
        } else {
            None
        }
    }
}
