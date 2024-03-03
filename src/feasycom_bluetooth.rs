use alloc::vec;
use alloc::{boxed::Box, vec::Vec};
use defmt::{error, info};
use embassy_stm32::{
    bind_interrupts, peripherals,
    usart::{self, Config, ConfigError, RingBufferedUartRx, UartRx, UartTx},
};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

const RING_BUFFER_SIZE: usize = 256;

pub struct FeasycomBluetoothTx<'a> {
    tx: UartTx<'a, peripherals::USART6, peripherals::DMA2_CH6>,
}

pub struct FeasycomBluetoothRx<'a> {
    rx: RingBufferedUartRx<'a, peripherals::USART1>,
    buf: [u8; RING_BUFFER_SIZE / 2],
    msg: Vec<u8>,
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

impl<'a> FeasycomBluetoothRx<'a> {
    pub fn new(
        peri: peripherals::USART1,
        rx_pin: peripherals::PB7,
        rx_dma: peripherals::DMA2_CH2,
    ) -> Result<Self, ConfigError> {
        let rx = UartRx::new(peri, Irqs, rx_pin, rx_dma, Config::default())?
            .into_ring_buffered(Box::leak(vec![0; RING_BUFFER_SIZE].into_boxed_slice()));

        Ok(Self {
            rx,
            buf: [0u8; RING_BUFFER_SIZE / 2],
            msg: Vec::new(),
        })
    }

    pub async fn read(&mut self) -> Result<Vec<u8>, usart::Error> {
        loop {
            if let Some(msg_len) = self.msg[0..]
                .windows(2)
                .position(|x| x == b"\r\n")
                .map(|x| x + 2)
            {
                let msg = self.msg.drain(0..msg_len);
                let line = msg.as_slice().trim_ascii();

                if line.len() > 0 {
                    return Ok(line.to_vec());
                }
            }

            let len = self.rx.read(&mut self.buf).await?;
            self.msg.extend(self.buf.iter().take(len));
        }
    }
}
