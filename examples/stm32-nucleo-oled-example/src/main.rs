#![no_std]
#![no_main]

extern crate alloc;

use embassy_executor::Spawner;
use embassy_stm32::{
    bind_interrupts, i2c::{self, Config}, peripherals, time::Hertz
};
use embassy_time::Timer; 
use panic_halt as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306Async};

use mousefood::{EmbeddedBackend, EmbeddedBackendConfig};
use ratatui::{Terminal, layout::Alignment, widgets::{Block, Borders, Paragraph}};

use embedded_alloc::LlffHeap as Heap;
use core::mem::MaybeUninit;

#[global_allocator]
static HEAP: Heap = Heap::empty();
const HEAP_SIZE: usize = 30720; 
static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    unsafe {
        HEAP.init(core::ptr::addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE);
    }

    let mut config = embassy_stm32::Config::default();
    config.rcc.hsi = true;         // Turn on the 16MHz High-speed Internal Clock
    config.rcc.sys = embassy_stm32::rcc::Sysclk::PLL1_R;  // Route it through the PLL multiplier
    config.rcc.pll = Some(embassy_stm32::rcc::Pll {
        source: embassy_stm32::rcc::PllSource::HSI,
        prediv: embassy_stm32::rcc::PllPreDiv::DIV1,
        mul: embassy_stm32::rcc::PllMul::MUL10,
        divp: None,
        divq: None,
        divr: Some(embassy_stm32::rcc::PllRDiv::DIV2),     // 16MHz * 10 / 2 = 80MHz
    });

    let p = embassy_stm32::init(config);

    let mut i2c_config = Config::default();
    i2c_config.frequency = Hertz::khz(100);

    let i2c = embassy_stm32::i2c::I2c::new(
        p.I2C1, p.PB8, p.PB7,         // D15, D14
        Irqs, p.DMA1_CH6, p.DMA1_CH7, 
        i2c_config
    );

    let interface = I2CDisplayInterface::new(i2c);

    let mut display = Ssd1306Async::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().await.unwrap();


    loop {
        {
            let backend = EmbeddedBackend::new(&mut display, EmbeddedBackendConfig::default());
            let mut terminal = Terminal::new(backend).unwrap();

            terminal.draw(|f| {
                let area = f.area();
                let block = Block::default()
                    .title("STM32 RataTui")
                    .borders(Borders::ALL);

                let paragraph = Paragraph::new("Hello mousefood")
                    .alignment(Alignment::Center)
                    .block(block);

                f.render_widget(paragraph, area);
            }).unwrap();
        }

        let _ = display.flush().await;

        Timer::after_millis(100).await;
    }
}
