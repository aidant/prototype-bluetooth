use defmt::Format;
use embassy_stm32::{
    bind_interrupts, peripherals,
    usart::{self, Config, ConfigError, RingBufferedUartRx, UartRx},
};
use heapless::{String, Vec};
use static_cell::StaticCell;

static DMA_BUF: StaticCell<[u8; 256]> = StaticCell::new();

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

pub struct FseasycomBluetoothRx<'a> {
    rx: RingBufferedUartRx<'a, peripherals::USART1, peripherals::DMA2_CH2>,
    buf: [u8; 64],
    msg: Vec<u8, 96>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Format)]
pub enum Error {
    SerialFraming,
    SerialNoise,
    SerialOverrun,
    SerialParity,
    SerialBufferTooLong,
    SerialOther,

    StringUtf8Error,

    StringError,
}

impl From<usart::Error> for Error {
    fn from(value: usart::Error) -> Self {
        match value {
            usart::Error::Framing => Self::SerialFraming,
            usart::Error::Noise => Self::SerialNoise,
            usart::Error::Overrun => Self::SerialOverrun,
            usart::Error::Parity => Self::SerialParity,
            usart::Error::BufferTooLong => Self::SerialBufferTooLong,
            _ => Self::SerialOther,
        }
    }
}

impl From<core::str::Utf8Error> for Error {
    fn from(_value: core::str::Utf8Error) -> Self {
        Self::StringUtf8Error
    }
}

impl From<()> for Error {
    fn from(_value: ()) -> Self {
        Self::StringError
    }
}

impl<'a> FseasycomBluetoothRx<'a> {
    pub fn new(
        peri: peripherals::USART1,
        rx_pin: peripherals::PB7,
        rx_dma: peripherals::DMA2_CH2,
    ) -> Result<Self, ConfigError> {
        let rx = UartRx::new(peri, Irqs, rx_pin, rx_dma, Config::default())?
            .into_ring_buffered(DMA_BUF.init([0u8; 256]));

        Ok(Self {
            rx,
            buf: [0u8; 64],
            msg: Vec::<u8, 96>::new(),
        })
    }

    pub async fn read(&mut self) -> Result<String<96>, Error> {
        let mut line = Option::<String<96>>::None;

        loop {
            let len = self.rx.read(&mut self.buf).await?;

            for mut byte in self.buf.iter().take(len) {
                if *byte == 255 {
                    byte = &44
                }

                match self.msg.push(*byte) {
                    Err(_) => {
                        self.msg.clear();
                    }
                    Ok(_) => {
                        if *byte == b'\n' {
                            let contents =
                                String::try_from(core::str::from_utf8(&self.msg)?.trim())?;

                            if contents.len() > 0 {
                                line = Some(contents)
                            }

                            self.msg.clear();
                        }
                    }
                }
            }

            if let Some(contents) = line {
                return Ok(contents);
            }
        }
    }
}
