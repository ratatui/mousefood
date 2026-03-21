#![no_std]
#![no_main]

extern crate alloc;
extern crate lilygo_epd47;

use alloc::boxed::Box;

use embedded_graphics::prelude::RgbColor;

#[allow(unused_imports)]
use esp_backtrace as _;
use esp_hal::{delay::Delay, main};
use lilygo_epd47::{Display, DrawMode, pin_config};

use mousefood::prelude::*;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{Frame, Terminal};

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default();
    let config = config.with_cpu_clock(esp_hal::clock::CpuClock::_240MHz);
    let peripherals = esp_hal::init(config);

    // Create PSRAM allocator
    esp_alloc::psram_allocator!(peripherals.PSRAM, esp_hal::psram);

    let mut display = Display::new(
        pin_config!(peripherals),
        peripherals.DMA_CH0,
        peripherals.LCD_CAM,
        peripherals.RMT,
    )
    .expect("to initialize correctly");

    let delay = Delay::new();
    display.power_on();
    delay.delay_millis(10);
    display.clear().expect("to clear display");

    let theme = ColorTheme {
        background: Rgb888::WHITE,
        foreground: Rgb888::BLACK,
        ..ColorTheme::ansi()
    };

    // setup mousefood
    let backend = EmbeddedBackendConfig {
        color_theme: theme,
        font_regular: mousefood::fonts::mono_10x20_atlas(),
        flush_callback: Box::new(move |display: &mut Display| {
            display
                .flush(DrawMode::BlackOnWhite)
                .expect("to flush to the display")
        }),
        ..Default::default()
    };

    let backend = EmbeddedBackend::new(&mut display, backend);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(draw).unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}

fn draw(frame: &mut Frame) {
    let text = "Ratatui on embedded devices!";
    let paragraph = Paragraph::new(text.black()).wrap(Wrap { trim: true });
    let bordered_block = Block::bordered().title("Mousefood");
    frame.render_widget(paragraph.block(bordered_block), frame.area());
}
