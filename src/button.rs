use stm32f4xx_hal::gpio::{gpioc::PC13, Input};
use switch_hal::{ActiveLow, InputSwitch, IntoSwitch, Switch};

/// The on-board user button.
pub struct Button(Switch<PC13<Input>, ActiveLow>);

impl Button {
    /// Initialize the button.
    ///
    /// Since each pin can only be moved once, effectively this is a singleton.
    pub fn new(pin: PC13<Input>) -> Self {
        Self(pin.into_active_low_switch())
    }

    /// Check if the button is currently pressed.
    ///
    /// This is an instantaneous reading of the button with no software debouncing.
    /// However, the Nucleo board seems to be debounced in hardware (presumably via
    /// C14 / C15 / R29 / R30 but the user manual does not specify).
    pub fn is_pressed(&self) -> bool {
        self.0.is_active().unwrap()
    }
}
