use alloc::format;
use alloc::string::ToString;
use display_interface_i2c::I2CInterface;
use embassy_stm32::dma::NoDma;
use embassy_stm32::i2c::{self, Config, I2c};
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, peripherals};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::primitives::{Line, PrimitiveStyle, Rectangle};
use embedded_graphics::{mono_font::ascii::FONT_6X10, prelude::*};
use embedded_text::{alignment::HorizontalAlignment, TextBox};
use ssd1306::{prelude::*, Ssd1306};

use crate::app_state::app_state_get;

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::task]
pub async fn piicodev_oled(
    peri: peripherals::I2C1,
    scl: peripherals::PB8,
    sda: peripherals::PB9,
    tx_dma: peripherals::DMA1_CH6,
    rx_dma: peripherals::DMA1_CH0,
) {
    let i2c = I2c::new(
        peri,
        scl,
        sda,
        Irqs,
        /*
            Once the embedded-hal-async update lands in Ssd1306 see:
            https://github.com/jamwaffles/ssd1306/pull/189 theoretically
            buffered mode should support an async init and flush, if that is the
            case the DMA channels can be switched back to tx_dma and rx_dma.
        */
        NoDma,
        NoDma,
        Hertz(400_000),
        Config::default(),
    );

    let interface = I2CInterface::new(i2c, 0x3C, 0x40);

    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    loop {
        let app_state = app_state_get().await;

        display.clear(BinaryColor::Off).unwrap();

        TextBox::with_alignment(
            format!(
                "{} - {} - {}",
                app_state.song_title.unwrap_or("".to_string()),
                app_state.song_album.unwrap_or("".to_string()),
                app_state.song_artist.unwrap_or("".to_string())
            )
            .as_str(),
            Rectangle::new(Point::new(0, 0), Size::new(128, 48)),
            character_style,
            HorizontalAlignment::Center,
        )
        .draw(&mut display)
        .unwrap();

        let mut progress = 0;

        if let (Some(elapsed), Some(total)) = (
            app_state.playback_elapsed_time,
            app_state.playback_total_time,
        ) {
            let progress_ratio = elapsed.as_millis() as f32 / total.as_millis() as f32;
            let scaled_progress = progress_ratio * 118.0;

            progress = scaled_progress as i32
        }

        Line::new(Point::new(5, 54), Point::new(5 + progress, 54))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 5))
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();
    }
}
