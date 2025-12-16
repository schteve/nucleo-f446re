use stm32f4xx_hal::{
    gpio::{
        Input,
        gpioa::{PA2, PA3},
    },
    pac::USART2,
    prelude::*,
    rcc::Clocks,
    serial::{
        Rx, Tx,
        config::{Config, InvalidConfig},
    },
    time::Bps,
};

/// A serial port implementation that uses the on-board serial port via the ST-Link debugger.
///
/// Currently hard coded to send bytes but could be updated to support u16 for 9-bit mode.
#[expect(clippy::module_name_repetitions)]
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
    ///
    /// # Errors
    ///
    /// This can fail if the requested baud rate is invalid or if the clock tree
    /// configuration is incompatible.
    #[expect(clippy::similar_names)]
    pub fn new(
        pin_tx: PA2<Input>,
        pin_rx: PA3<Input>,
        usart: USART2,
        clocks: &Clocks,
        baud: Bps,
    ) -> Result<Self, InvalidConfig> {
        let pin_tx_af = pin_tx.into_alternate();
        let pin_rx_af = pin_rx.into_alternate();

        let serial = usart.serial(
            (pin_tx_af, pin_rx_af),
            Config::default().baudrate(baud),
            clocks,
        )?;

        let (tx, rx) = serial.split();
        Ok(Self { tx, rx })
    }
}
