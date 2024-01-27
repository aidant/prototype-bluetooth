#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod feasycom_bluetooth_rx;
mod feasycom_bluetooth_tx;

extern crate defmt_rtt;
extern crate panic_probe;

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::peripherals;
use feasycom_bluetooth_rx::FeasycomBluetoothRx;
use feasycom_bluetooth_tx::FeasycomBluetoothTx;

#[embassy_executor::task]
async fn task_print_bluetooth_lines(
    peri: peripherals::USART1,
    rx_pin: peripherals::PB7,
    rx_dma: peripherals::DMA2_CH2,
) -> ! {
    let mut feasycom_bluetooth_rx = FeasycomBluetoothRx::new(peri, rx_pin, rx_dma).unwrap();

    loop {
        match feasycom_bluetooth_rx.read().await {
            Result::Ok(line) => {
                info!("{}", line);
            }
            Result::Err(error) => {
                error!("{}", error);
            }
        };
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    info!("Hello World!");

    spawner
        .spawn(task_print_bluetooth_lines(p.USART1, p.PB7, p.DMA2_CH2))
        .unwrap();

    let mut feasycom_bluetooth_tx = FeasycomBluetoothTx::new(p.USART6, p.PA11, p.DMA2_CH6).unwrap();

    feasycom_bluetooth_tx.write(b"AT+NAME\r\n").await.unwrap();
}
