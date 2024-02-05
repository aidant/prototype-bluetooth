#![no_main]
#![no_std]

extern crate alloc;
extern crate defmt_rtt;
extern crate panic_probe;

mod app_state;
mod feasycom_bluetooth_rx;
mod feasycom_bluetooth_tx;
mod feasycom_indication;
mod piicodev_oled;

use alloc::string::ToString;
use app_state::{app_state_set, AppState};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::peripherals;
use embedded_alloc::Heap;
use feasycom_bluetooth_rx::FeasycomBluetoothRx;
use feasycom_bluetooth_tx::FeasycomBluetoothTx;
use feasycom_indication::Indication;
use piicodev_oled::piicodev_oled;

#[embassy_executor::task]
async fn task_print_bluetooth_lines(
    peri: peripherals::USART1,
    rx_pin: peripherals::PB7,
    rx_dma: peripherals::DMA2_CH2,
) -> ! {
    let mut feasycom_bluetooth_rx = FeasycomBluetoothRx::new(peri, rx_pin, rx_dma).unwrap();

    let mut app_state = AppState::default();

    loop {
        match feasycom_bluetooth_rx.read().await {
            Result::Ok(line) => {
                info!("{}", line.as_str());

                let indication = Indication::try_from(line.as_str()).unwrap();

                info!("{}", indication);

                match indication {
                    Indication::TRACKINFO(trackinfo) => {
                        app_state.song_album = Some(trackinfo.album.to_string());
                        app_state.song_artist = Some(trackinfo.artist.to_string());
                        app_state.song_title = Some(trackinfo.title.to_string());
                        app_state_set(app_state.clone()).await;
                    }
                    Indication::TRACKSTAT(trackstat) => {
                        app_state.playback_elapsed_time = Some(trackstat.elapsed_time);
                        app_state.playback_total_time = Some(trackstat.total_time);
                        app_state_set(app_state.clone()).await;
                    }
                    _ => {}
                }
            }
            Result::Err(error) => {
                error!("{}", error);
            }
        };
    }
}

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let p = embassy_stm32::init(Default::default());

    spawner
        .spawn(task_print_bluetooth_lines(p.USART1, p.PB7, p.DMA2_CH2))
        .unwrap();

    // spawner
    //     .spawn(piicodev_oled(p.I2C1, p.PB8, p.PB9, p.DMA1_CH6, p.DMA1_CH0))
    //     .unwrap();

    let mut feasycom_bluetooth_tx = FeasycomBluetoothTx::new(p.USART6, p.PA11, p.DMA2_CH6).unwrap();

    // feasycom_bluetooth_tx.write(b"AT+NAME\r\n").await.unwrap();
    // feasycom_bluetooth_tx.write(b"AT+I2SCFG\r\n").await.unwrap();
    // feasycom_bluetooth_tx.write(b"AT+VER\r\n").await.unwrap();
    feasycom_bluetooth_tx.write(b"AT+SPKVOL\r\n").await.unwrap();
    feasycom_bluetooth_tx
        .write(b"AT+SPKVOL=+\r\n")
        .await
        .unwrap();
    feasycom_bluetooth_tx
        .write(b"AT+SPKVOL=+\r\n")
        .await
        .unwrap();
}
