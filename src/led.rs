use stm32f4xx_hal::{
    gpio::{gpioa::PA5, Input, Output},
    pac::TIM2,
    prelude::*,
    rcc::Clocks,
    timer::PwmChannel,
};
use switch_hal::{ActiveHigh, IntoSwitch, OutputSwitch, Switch, ToggleableOutputSwitch};

/// A trait that builds some type of LED from the given PA5 pin and TIM2 timer.
pub trait LedBuilder {
    /// Build the timer.
    fn build(pin: PA5<Input>, tim: TIM2, clocks: &Clocks) -> Self;
}

/// A digital LED controller (on / off).
pub struct LedDigital(Switch<PA5<Output>, ActiveHigh>);

impl LedDigital {
    /// Initialize the digital LED.
    ///
    /// Since each pin can only be moved once, effectively this is a singleton.
    pub fn new(pin: PA5<Input>) -> Self {
        let mut me = Self(pin.into_push_pull_output().into_active_high_switch());
        me.off();
        me
    }

    /// Turn the LED on.
    pub fn on(&mut self) {
        self.0.on().unwrap()
    }

    /// Turn the LED off.
    pub fn off(&mut self) {
        self.0.off().unwrap()
    }

    /// Toggle the LED state.
    pub fn toggle(&mut self) {
        self.0.toggle().unwrap()
    }
}

impl LedBuilder for LedDigital {
    /// Build a digital LED. Timer and clocks are not needed for digital output so are ignored.
    fn build(pin: PA5<Input>, _tim: TIM2, _clocks: &Clocks) -> Self {
        Self::new(pin)
    }
}

/// An analog LED controller (brightness from 0 - 100%).
pub struct LedAnalog(PwmChannel<TIM2, 0>);

impl LedAnalog {
    /// Initialize the analog LED.
    ///
    /// PWM frequency is set to 20 kHz. In the future if it needs to be user-defined,
    /// this could be done through a generic parameter on LedAnalog.
    /// Since each pin can only be moved once, effectively this is a singleton.
    pub fn new(pin: PA5<Input>, tim: TIM2, clocks: &Clocks) -> Self {
        let mut pwm_ch1 = tim.pwm_hz(pin.into_alternate(), 20.kHz(), clocks).split();
        pwm_ch1.set_duty(0);
        pwm_ch1.enable();
        Self(pwm_ch1)
    }

    /// Set the LED's duty cycle as a percentage.
    ///
    /// Valid duty cycles range from 0% (completely off) to 100% (completely on).
    /// Assumes that the timer max duty cycle is large enough so that increments of
    /// 1% on the duty cycle are meaningful, and 100% is "close enough" to the max
    /// duty value (if not exactly).
    pub fn set_duty(&mut self, duty: u8) {
        assert!(duty <= 100);

        let max_duty = self.0.get_max_duty();
        let val = (max_duty / 100) * duty as u16;
        self.0.set_duty(val);
    }
}

impl LedBuilder for LedAnalog {
    /// Build an analog LED.
    fn build(pin: PA5<Input>, tim: TIM2, clocks: &Clocks) -> Self {
        Self::new(pin, tim, clocks)
    }
}
