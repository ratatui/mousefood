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

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use core::sync::atomic::{AtomicUsize, Ordering};

use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::spi::Mode;
use esp_hal::spi::master::{Config as SpiConfig, Spi};
use esp_hal::time::Rate;
use esp_hal::timer::timg::TimerGroup;
use mousefood::prelude::*;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use weact_studio_epd::WeActStudio213BlackWhiteDriver;
use weact_studio_epd::graphics::{Display213BlackWhite, DisplayRotation};

esp_bootloader_esp_idf::esp_app_desc!();

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    defmt::error!("PANIC: {:?}", defmt::Debug2Format(info));
    loop {}
}

const MESSAGES: [&str; 5] = [
    "Hello World",
    "Hello Rat",
    "Hello Ferris",
    "Hello Embassy",
    "Hello EPaper",
];

static CURRENT_IDX: AtomicUsize = AtomicUsize::new(0);

#[embassy_executor::task]
async fn printer_task() {
    let mut idx = 0usize;
    loop {

        CURRENT_IDX.store(idx % MESSAGES.len(), Ordering::Relaxed);
        info!("Printer update: {}", MESSAGES[idx % MESSAGES.len()]);

        idx = idx.wrapping_add(1);


        Timer::after(Duration::from_secs(5)).await;
    }
}

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    rtt_target::rtt_init_defmt!();
    info!("Booting...");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 128 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_interrupt =
        esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, sw_interrupt.software_interrupt0);

    info!("Embassy initialized!");

    let cs = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());
    let dc = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
    let rst = Output::new(peripherals.GPIO10, Level::High, OutputConfig::default());
    let busy = Input::new(
        peripherals.GPIO11,
        InputConfig::default().with_pull(Pull::Up),
    );

    let spi = Spi::new(
        peripherals.SPI2,
        SpiConfig::default()
            .with_frequency(Rate::from_mhz(4))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(peripherals.GPIO2)
    .with_mosi(peripherals.GPIO3);

    let spi_device = ExclusiveDevice::new(spi, cs, Delay::new()).unwrap();
    let interface = display_interface_spi::SPIInterface::new(spi_device, dc);

    let mut driver = WeActStudio213BlackWhiteDriver::new(interface, busy, rst, Delay::new());
    driver.init().unwrap();
    info!("EPD driver initialized");

    let mut display = Display213BlackWhite::new();
    display.set_rotation(DisplayRotation::Rotate270);

    let backend_config = EmbeddedBackendConfig {
        flush_callback: Box::new(move |d: &mut Display213BlackWhite| {
            driver.full_update(d).expect("EPD update failed");
        }),
        ..Default::default()
    };
    let backend = EmbeddedBackend::new(&mut display, backend_config);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| draw(f, MESSAGES[0])).unwrap();
    info!("Initial draw complete");

    spawner.spawn(printer_task().unwrap());

    loop {
        Timer::after(Duration::from_secs(15)).await;

        let idx = CURRENT_IDX.load(Ordering::Relaxed);
        let msg = MESSAGES[idx];
        info!("Redrawing: {}", msg);
        terminal.draw(|f| draw(f, msg)).unwrap();
    }
}


fn draw(frame: &mut Frame, msg: &str) {
    let paragraph = Paragraph::new(msg.white()).wrap(Wrap { trim: true });
    let block = Block::bordered().title("Mousefood");
    frame.render_widget(paragraph.block(block), frame.area());
}
