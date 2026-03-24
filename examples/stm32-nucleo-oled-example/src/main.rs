#![no_std]
#![no_main]

extern crate alloc;

use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts,
    i2c::{self, Config},
    peripherals,
    time::Hertz,
};
use embassy_time::Timer;
use panic_halt as _;
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};

use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::{
    Terminal,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
};

use core::mem::MaybeUninit;
use embedded_alloc::LlffHeap as Heap;

// Heap allocator used by ratatui + mousefood
#[global_allocator]
static HEAP: Heap = Heap::empty();

// 30 KB heap for dynamic allocations
const HEAP_SIZE: usize = 30720;

// Static backing memory for allocator
static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

// Interrupt bindings required by Embassy I2C driver
bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
    DMA1_CHANNEL6 => embassy_stm32::dma::InterruptHandler<peripherals::DMA1_CH6>;
    DMA1_CHANNEL7 => embassy_stm32::dma::InterruptHandler<peripherals::DMA1_CH7>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize the global heap allocator
    unsafe {
        HEAP.init(core::ptr::addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE);
    }

    // Configure MCU clock tree
    let mut config = embassy_stm32::Config::default();

    // Enable internal 16 MHz oscillator
    config.rcc.hsi = true;

    // Use PLL output as system clock
    config.rcc.sys = embassy_stm32::rcc::Sysclk::PLL1_R;

    config.rcc.pll = Some(embassy_stm32::rcc::Pll {
        source: embassy_stm32::rcc::PllSource::HSI,
        prediv: embassy_stm32::rcc::PllPreDiv::DIV1,
        mul: embassy_stm32::rcc::PllMul::MUL10,
        divp: None,
        divq: None,

        // 16 MHz * 10 / 2 = 80 MHz system clock
        divr: Some(embassy_stm32::rcc::PllRDiv::DIV2),
    });

    let p = embassy_stm32::init(config);

    // Configure I2C bus used by the OLED
    let mut i2c_config = Config::default();

    // Standard I2C speed
    i2c_config.frequency = Hertz::khz(100);

    let i2c = embassy_stm32::i2c::I2c::new(
        p.I2C1, p.PB8, p.PB7, p.DMA1_CH6, p.DMA1_CH7, Irqs, i2c_config,
    );

    // Initialize SSD1306 display
    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    // Setup Ratatui terminal backend
    let backend = EmbeddedBackend::new(
        &mut display,
        EmbeddedBackendConfig {
            // Push framebuffer to OLED after each draw
            flush_callback: alloc::boxed::Box::new(|d| {
                d.flush().unwrap();
            }),

            ..Default::default()
        },
    );

    let mut terminal = Terminal::new(backend).unwrap();

    // UI render loop
    loop {
        let _ = terminal.draw(|f| {
            let area = f.area();

            let block = Block::default()
                .title("STM32 Ratatui")
                .borders(Borders::ALL);

            let paragraph = Paragraph::new("Hello mousefood")
                .alignment(Alignment::Center)
                .block(block);

            f.render_widget(paragraph, area);
        });

        Timer::after_millis(100).await;
    }
}
