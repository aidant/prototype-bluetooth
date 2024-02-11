use crate::feasycom_indication::Indication;
use alloc::vec;
use alloc::{boxed::Box, vec::Vec};
use anyhow::{Error, Result};
use core::fmt::Display;
use defmt::Format;
use embassy_stm32::{
    bind_interrupts, peripherals,
    usart::{self, Config, ConfigError, RingBufferedUartRx, UartRx},
};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

#[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
enum UsartError {
    Framing,
    Noise,
    Overrun,
    Parity,
    BufferTooLong,

    Unknown,
}

impl From<usart::Error> for UsartError {
    fn from(value: usart::Error) -> Self {
        match value {
            usart::Error::Framing => Self::Framing,
            usart::Error::Noise => Self::Noise,
            usart::Error::Overrun => Self::Overrun,
            usart::Error::Parity => Self::Parity,
            usart::Error::BufferTooLong => Self::BufferTooLong,
            _ => Self::Unknown,
        }
    }
}

impl Display for UsartError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Serial {} Error", self)
    }
}

pub struct FeasycomBluetoothRx<'a> {
    rx: RingBufferedUartRx<'a, peripherals::USART1, peripherals::DMA2_CH2>,
    buf: [u8; 128],
    msg: Vec<u8>,
}

impl<'a> FeasycomBluetoothRx<'a> {
    pub fn new(
        peri: peripherals::USART1,
        rx_pin: peripherals::PB7,
        rx_dma: peripherals::DMA2_CH2,
    ) -> Result<Self, ConfigError> {
        let rx = UartRx::new(peri, Irqs, rx_pin, rx_dma, Config::default())?
            .into_ring_buffered(Box::leak(vec![0; 256].into_boxed_slice()));

        Ok(Self {
            rx,
            buf: [0u8; 128],
            msg: Vec::new(),
        })
    }

    fn get_indication_from_msg(&mut self) -> Result<Option<Indication>> {
        let Some(start) = self.msg[0..]
            .windows(2)
            .position(|x| x == b"\r\n")
            .map(|x| x + 2)
        else {
            return Ok(None);
        };

        let Some(end) = self.msg[start..]
            .windows(2)
            .position(|x| x == b"\r\n")
            .map(|x| start + x)
        else {
            return Ok(None);
        };

        let msg = self.msg.drain((start - 2)..(end + 2));
        let indication = msg.as_slice().trim_ascii();

        Ok(Some(Indication::try_from(indication)?))
    }

    pub async fn read(&mut self) -> Result<Indication> {
        loop {
            if let Some(msg) = self.get_indication_from_msg()? {
                return Ok(msg);
            }

            let len = self
                .rx
                .read(&mut self.buf)
                .await
                .map_err(|err| Error::msg(UsartError::from(err)))?;

            for byte in self.buf.iter().take(len) {
                self.msg.push(*byte);
            }
        }
    }
}
