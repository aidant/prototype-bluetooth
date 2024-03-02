#![no_main]
#![no_std]
#![feature(byte_slice_trim_ascii, slice_split_once)]

extern crate alloc;
extern crate defmt_rtt;
extern crate panic_probe;

mod app_state;
mod feasycom_bluetooth;
mod feasycom_protocol;
mod piicodev_oled;

use embassy_executor::Spawner;
use embedded_alloc::Heap;
use feasycom_bluetooth::feasycom_bluetooth;

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
        .spawn(feasycom_bluetooth(
            p.USART6, p.PA11, p.DMA2_CH6, p.USART1, p.PB7, p.DMA2_CH2,
        ))
        .unwrap();
}
