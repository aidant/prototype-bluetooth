use defmt::{error, info};
use embassy_stm32::peripherals;

use crate::{
    feasycom_bluetooth::{FeasycomBluetoothRx, FeasycomBluetoothTx},
    feasycom_protocol::command::Command,
    feasycom_protocol::indication::Indication,
};

enum FeasycomState {}

impl FeasycomState {}

#[embassy_executor::task]
pub async fn feasycom_task(
    tx_peri: peripherals::USART6,
    tx_pin: peripherals::PA11,
    tx_dma: peripherals::DMA2_CH6,
    rx_peri: peripherals::USART1,
    rx_pin: peripherals::PB7,
    rx_dma: peripherals::DMA2_CH2,
) -> ! {
    let mut feasycom_bluetooth_tx = FeasycomBluetoothTx::new(tx_peri, tx_pin, tx_dma).unwrap();
    let mut feasycom_bluetooth_rx = FeasycomBluetoothRx::new(rx_peri, rx_pin, rx_dma).unwrap();

    feasycom_bluetooth_tx
        .write(b"AT+AVRCPCFG=3\r\n")
        .await
        .unwrap();

    loop {
        let msg = match feasycom_bluetooth_rx.read().await {
            Ok(msg) => msg,
            Err(e) => {
                error!("{}", e);
                continue;
            }
        };

        let indication = match Indication::try_from(msg) {
            Ok(indication) => indication,
            Err(e) => {
                error!("{}", defmt::Debug2Format(&e));
                continue;
            }
        };

        info!("{}", indication);
    }
}
