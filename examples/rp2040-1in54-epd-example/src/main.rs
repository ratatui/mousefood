#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_alloc::LlffHeap as Heap;
use epd_waveshare::{
    epd1in54_v2::{Display1in54, Epd1in54},
    prelude::WaveshareDisplay,
};
use fugit::RateExtU32;
use panic_halt as _;
use rp2040_hal::{
    Sio,
    clocks::{Clock, init_clocks_and_plls},
    gpio::{FunctionSio, FunctionSpi, Pin, PullDown, PullUp, SioInput, SioOutput},
    pac,
    spi::Spi,
    timer::Timer,
    watchdog::Watchdog,
};

use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

use core::convert::Infallible;

use mousefood::prelude::*;
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{Frame, Terminal};

#[global_allocator]
static HEAP: Heap = Heap::empty();
extern crate alloc;
use alloc::boxed::Box;

// --- THE IGNITION KEY ---
// This places the 256-byte bootloader at the very start of the flash memory.
// Without this, the RP2040 ROM refuses to jump to our code.
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;
// ------------------------

pub struct DisplayAdapter(pub Display1in54);

impl Dimensions for DisplayAdapter {
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        self.0.bounding_box()
    }
}

impl DrawTarget for DisplayAdapter {
    type Color = Rgb888;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let converted = pixels.into_iter().map(|Pixel(p, c)| Pixel(p, c.into()));
        self.0.draw_iter(converted)
    }
}

#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 100000;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe {
            let heap_ptr = core::ptr::addr_of_mut!(HEAP_MEM) as usize;
            HEAP.init(heap_ptr, HEAP_SIZE);
        }
    }

    let mut pac = pac::Peripherals::take().unwrap();
    let _cp = pac::CorePeripherals::take().unwrap();

    let mut wdt = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        12_000_000u32,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut wdt,
    )
    .ok()
    .unwrap();

    let mut timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let sio = Sio::new(pac.SIO);
    let pins = rp2040_hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let sclk = pins.gpio2.into_function::<FunctionSpi>();
    let mosi = pins.gpio3.into_function::<FunctionSpi>();

    let spi = Spi::<_, _, _, 8>::new(pac.SPI0, (mosi, sclk)).init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        4_000_000.Hz(),
        embedded_hal::spi::MODE_0,
    );

    let cs: Pin<_, FunctionSio<SioOutput>, PullDown> = pins.gpio5.into_push_pull_output();
    let dc: Pin<_, FunctionSio<SioOutput>, PullDown> = pins.gpio6.into_push_pull_output();
    let rst: Pin<_, FunctionSio<SioOutput>, PullDown> = pins.gpio10.into_push_pull_output();
    let busy: Pin<_, FunctionSio<SioInput>, PullUp> = pins.gpio11.into_pull_up_input();

    let mut spi_bus = embedded_hal_bus::spi::ExclusiveDevice::new(spi, cs, timer).unwrap();

    let mut epd = Epd1in54::new(&mut spi_bus, busy, dc, rst, &mut timer, None).unwrap();

    // Uncomment the below lines when you wanna clear the display

    //epd.clear_frame(&mut spi_bus, &mut timer).unwrap();
    //epd.display_frame(&mut spi_bus, &mut timer).unwrap();
    //epd.sleep(&mut spi_bus, &mut timer).unwrap();

    let mut display = Display1in54::default();
    display.set_rotation(epd_waveshare::prelude::DisplayRotation::Rotate270);

    let mut adapter = DisplayAdapter(display);

    let backend_config = EmbeddedBackendConfig {
        flush_callback: Box::new(move |adapter: &mut DisplayAdapter| {
            epd.update_and_display_frame(&mut spi_bus, adapter.0.buffer(), &mut timer)
                .unwrap();
        }),
        ..Default::default()
    };

    let backend = EmbeddedBackend::new(&mut adapter, backend_config);
    let mut terminal = Terminal::new(backend).unwrap();

    // Comment this line when clearng the display (Uncommenting the above commennted lines)
    terminal.draw(draw).unwrap();

    loop {
        cortex_m::asm::wfi();
    }
}

fn draw(frame: &mut Frame) {
    let text = "Ratatui on Embedded devices";

    let epaper_theme = Style::default()
        .fg(ratatui::style::Color::Black)
        .bg(ratatui::style::Color::White);

    let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
    let bordered_block = Block::bordered().title("Mousefood").style(epaper_theme);
    frame.render_widget(paragraph.block(bordered_block), frame.area());
}
