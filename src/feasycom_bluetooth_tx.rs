use embassy_stm32::{
    peripherals,
    usart::{self, Config, ConfigError, UartTx},
};

pub struct FeasycomBluetoothTx<'a> {
    tx: UartTx<'a, peripherals::USART6, peripherals::DMA2_CH6>,
}

impl<'a> FeasycomBluetoothTx<'a> {
    pub fn new(
        peri: peripherals::USART6,
        tx_pin: peripherals::PA11,
        tx_dma: peripherals::DMA2_CH6,
    ) -> Result<Self, ConfigError> {
        let tx = UartTx::new(peri, tx_pin, tx_dma, Config::default())?;

        Ok(Self { tx })
    }

    pub async fn write(&mut self, buffer: &[u8]) -> Result<(), usart::Error> {
        self.tx.write(buffer).await?;

        Ok(())
    }
}
