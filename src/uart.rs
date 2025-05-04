use esp_idf_hal::gpio::{Gpio1, Gpio3};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::prelude::*;
use esp_idf_hal::uart::{config, UartDriver, Uart};
use anyhow::Result;

pub struct SerialComm {
    driver: UartDriver<'static>,
}

impl SerialComm {
    pub fn new(
        uart: impl Peripheral<P = impl Uart> + 'static,
        tx_pin: impl Peripheral<P = Gpio1> + 'static,
        rx_pin: impl Peripheral<P = Gpio3> + 'static,
        baud_rate: u32,
    ) -> Result<Self> {
        let config = config::Config::new().baudrate(Hertz(baud_rate));

        // Convert tx_pin to OutputPin and rx_pin to InputPin
        let tx_pin = tx_pin.into_output()?;
        let rx_pin = rx_pin.into_input()?;

        // Create the UART driver
        let driver = UartDriver::new(
            uart,
            tx_pin,
            rx_pin,
            None,
            None,
            &config,
        )?;

        Ok(Self { driver })
    }

    pub fn send(&mut self, message: &str) -> Result<()> {
        self.driver.write(message.as_bytes())?;
        Ok(())
    }
}

impl core::fmt::Write for SerialComm {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.send(s).map_err(|_| core::fmt::Error)
    }
}