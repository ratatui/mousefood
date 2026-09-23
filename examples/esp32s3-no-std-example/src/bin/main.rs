#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

extern crate alloc;

use alloc::boxed::Box;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::spi::Mode;
use esp_hal::spi::master::{Config as SpiConfig, Spi};
use esp_hal::time::Rate;
use mipidsi::Builder;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ILI9341Rgb565;
use mipidsi::options::Orientation;
use mousefood::prelude::*;
use ratatui::layout::Alignment;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{Frame, Terminal};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

const DISPLAY_SIZE: (u16, u16) = (240, 320);

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32s3

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 256 * 1024);

    esp_println::println!("Init!");

    // ESP32-S3 -> ILI9341 wiring:
    // 3V3 -> VCC and LED, GND -> GND, GPIO12 -> SCK, GPIO11 -> SDI (MOSI),
    // GPIO10 -> CS, GPIO9 -> DC, GPIO14 -> RESET. SDO (MISO) is unused.

    let spi = Spi::new(
        peripherals.SPI2,
        SpiConfig::default()
            .with_frequency(Rate::from_mhz(40))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(peripherals.GPIO12)
    .with_mosi(peripherals.GPIO11);

    let cs = Output::new(peripherals.GPIO10, Level::High, OutputConfig::default());
    let spi_device = ExclusiveDevice::new(spi, cs, Delay::new()).unwrap();
    let dc = Output::new(peripherals.GPIO9, Level::High, OutputConfig::default());
    let rst = Output::new(peripherals.GPIO14, Level::High, OutputConfig::default());
    let buffer = Box::leak(Box::new([0_u8; 4096]));
    let spi_interface = SpiInterface::new(spi_device, dc, buffer);

    let mut delay = Delay::new();
    let mut display = Builder::new(ILI9341Rgb565, spi_interface)
        .reset_pin(rst)
        .display_size(DISPLAY_SIZE.0, DISPLAY_SIZE.1)
        .orientation(Orientation::new().flip_horizontal())
        .init(&mut delay)
        .expect("Failed to initialize ILI9341 display");

    let backend = EmbeddedBackend::new(&mut display, Default::default());
    let mut terminal = Terminal::new(backend).expect("Failed to create Mousefood terminal");

    loop {
        esp_println::println!("Squeak!");
        terminal.draw(draw).expect("Failed to draw terminal");
        delay.delay_millis(500);
    }
}

#[allow(
    clippy::large_stack_frames,
    reason = "Ratatui widget rendering is slightly above Clippy's embedded threshold"
)]
fn draw(frame: &mut Frame) {
    let block = Block::bordered().title("Mousefood");
    let paragraph = Paragraph::new("Hello from Mousefood!")
        .alignment(Alignment::Center)
        .block(block);
    frame.render_widget(paragraph, frame.area());
}
