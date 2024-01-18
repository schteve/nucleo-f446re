[![Crates.io](https://img.shields.io/crates/v/nucleo-f446re)](https://crates.io/crates/nucleo-f446re)
[![docs.rs](https://img.shields.io/docsrs/nucleo-f446re)](https://docs.rs/nucleo-f446re/)

# Nucleo-F446RE

A Board Support Package (BSP) for the [Nucleo F446RE](https://www.st.com/en/evaluation-tools/nucleo-f446re.html) development board. This board has a few basic features as well as breakout headers for daughter boards such as Arduino shields. It is a starting point to evaluate or develop basic applications for the [STM32F446 product line](https://www.st.com/en/microcontrollers-microprocessors/stm32f446.html).

This crate intends to be a beginner-friendly way to use this Nucleo board. It handles linker configuration, board initialization, and provides access to implementations of the board's features without needing to directly use a HAL. It may also be helpful as an example for learning embedded Rust with the STM32F446. For advanced applications you likely want to use the [stm32f4xx-hal](https://github.com/stm32-rs/stm32f4xx-hal) crate directly.

Note: this crate doesn't select the build target for your application. You can do this by copying the `.cargo/config.toml` to your application, or by setting the target on each build e.g. `cargo run --target thumbv7em-none-eabi`.

# Board features

* User LED
* User button
* Serial port (via the on-board ST-Link)
* Blocking timer for delays

# Examples

There are two main ways to make use of this BSP:
* Let the BSP perform all board initialization and give you access to the board features. This is the simplest option if you don't have any other hardware connected to your board since the unused PAC peripherals are dropped.
* You perform the board initialization and then directly instantiate the BSP features you want to use. This lets you control what happens to the rest of the PAC peripherals.

Many examples are provided to illustrate both ways of using the BSP, which can be found in the `examples` directory. Use a USB cable to connect the Nucleo board's debugger to the host computer, and then run an example with cargo:
> `cargo run --example blinky`

Here, `cargo run` implicitly makes use of [probe-rs](https://github.com/probe-rs/probe-rs) to flash, execute, and handle panic and backtrace info. If you prefer GDB and OpenOCD, you can change the behavior of `cargo run` by editing `.cargo/config.toml` (basic scripts for GDB and OpenOCD are provided).

# Reference material

* [Nucleo user manual](https://www.st.com/resource/en/user_manual/um1724-stm32-nucleo64-boards-mb1136-stmicroelectronics.pdf)
* [STM32F446 datasheet](https://www.st.com/resource/en/datasheet/stm32f446re.pdf)
* [STM32F446 reference manual](https://www.st.com/resource/en/reference_manual/dm00135183-stm32f446xx-advanced-arm-based-32-bit-mcus-stmicroelectronics.pdf)

# License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
