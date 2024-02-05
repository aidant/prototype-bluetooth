use alloc::vec;
use alloc::{boxed::Box, string::String, vec::Vec};
use embassy_stm32::{
    bind_interrupts, peripherals,
    usart::{self, Config, ConfigError, RingBufferedUartRx, UartRx},
};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

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
            msg: Vec::<u8>::new(),
        })
    }

    fn get_indication_from_msg(&mut self) -> Option<String> {
        let start = self.msg[0..]
            .windows(2)
            .position(|x| x == b"\r\n")
            .map(|x| x + 2)?;

        let end = self.msg[start..]
            .windows(2)
            .position(|x| x == b"\r\n")
            .map(|x| start + x)?;

        let indication =
            String::try_from(core::str::from_utf8(&self.msg[start..end]).ok()?).ok()?;

        self.msg.drain((start - 2)..(end + 2));

        Some(indication)
    }

    pub async fn read(&mut self) -> Result<String, usart::Error> {
        loop {
            if let Some(msg) = self.get_indication_from_msg() {
                return Ok(msg);
            }

            let len = self.rx.read(&mut self.buf).await?;

            for mut byte in self.buf.iter().take(len) {
                if *byte == 255 {
                    byte = &44
                }

                self.msg.push(*byte);
            }
        }
    }
}
