use stm32f4xx_hal::{
    gpio::{
        gpioa::{PA2, PA3},
        Input,
    },
    pac::USART2,
    prelude::*,
    rcc::Clocks,
    serial::{config::Config, Rx, Tx},
};

/// A serial port implementation that uses the on-board serial port via the ST-Link debugger.
///
/// Currently hard coded to send bytes but could be updated to support u16 for 9-bit mode.
pub struct SerialPort {
    /// The transmit line of the port. Can send bytes or strings.
    pub tx: Tx<USART2, u8>,
    /// The receive line of the port. Can receive bytes.
    pub rx: Rx<USART2, u8>,
}

impl SerialPort {
    /// Initialize the serial port. After this, the constituent Tx and Rx lines can
    /// be directly used to send and receive data.
    ///
    /// Since each pin can only be moved once, effectively this is a singleton.
    pub fn new(pin_tx: PA2<Input>, pin_rx: PA3<Input>, usart: USART2, clocks: &Clocks) -> Self {
        let pin_tx_af = pin_tx.into_alternate();
        let pin_rx_af = pin_rx.into_alternate();

        let serial = usart
            .serial(
                (pin_tx_af, pin_rx_af),
                Config::default().baudrate(9600.bps()),
                clocks,
            )
            .unwrap();
        let (tx, rx) = serial.split();

        Self { tx, rx }
    }
}
