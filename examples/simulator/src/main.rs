//! # Simulator
//!
//! Run mousefood apps on your computer inside a simulator! Uses [embedded-graphics-simulator](https://crates.io/crates/embedded-graphics-simulator).
//!
//! ## Requirements
//!
//! This app requires [SDL2](https://wiki.libsdl.org/SDL2/Installation) to be installed.
//!
//! If you use [nix](https://nixos.org) you can run `nix-shell -p SDL2`
//! before running the application.
//!
//! ## Run
//!
//! To start this demo, simply run:
//!
//! ```shell
//! cargo run -p simulator
//! ```
//!
//! A window will open with the simulator running.

use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};
use mousefood::embedded_graphics::geometry;
use mousefood::error::Error;
use mousefood::prelude::*;
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{Frame, Terminal, style::*};

fn main() -> Result<(), Error> {
    // Create window where the simulation will happen
    let mut simulator_window = Window::new(
        "mousefood simulator",
        &OutputSettings {
            scale: 4,
            ..Default::default()
        },
    );
    simulator_window.set_max_fps(30);

    // Define properties of the display which will be shown in the simulator window
    let mut display = SimulatorDisplay::<Bgr565>::new(geometry::Size::new(128, 200));

    let backend_config = EmbeddedBackendConfig {
        // Define how to display newly rendered widgets to the simulator window
        flush_callback: Box::new(move |display| {
            simulator_window.update(display);
            if simulator_window.events().any(|e| e == SimulatorEvent::Quit) {
                panic!("simulator window closed");
            }
        }),
        ..Default::default()
    };
    let backend: EmbeddedBackend<SimulatorDisplay<_>, _> =
        EmbeddedBackend::new(&mut display, backend_config);

    // Start ratatui with our simulator backend
    let mut terminal = Terminal::new(backend)?;

    // Run an infinite loop, where widgets will be rendered
    loop {
        terminal.draw(draw2)?;
    }
}

fn draw(frame: &mut Frame) {
    let text = "Ratatui on embedded devices!";
    let paragraph = Paragraph::new(text.dark_gray()).wrap(Wrap { trim: true });
    let bordered_block = Block::bordered()
        .border_style(Style::new().yellow())
        .title("Mousefood");
    frame.render_widget(paragraph.block(bordered_block), frame.area());
}
fn draw2(frame: &mut Frame) {
    use ratatui::style::Modifier;
    use ratatui::text::{Line, Span};

    let lines = vec![
        Line::from(Span::styled(
            "BOLD text",
            Style::new().add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "DIM text",
            Style::new().add_modifier(Modifier::DIM),
        )),
        Line::from(Span::styled(
            "ITALIC text",
            Style::new().add_modifier(Modifier::ITALIC),
        )),
        Line::from(Span::styled(
            "UNDERLINED text",
            Style::new().add_modifier(Modifier::UNDERLINED),
        )),
        Line::from(Span::styled(
            "SLOW_BLINK text",
            Style::new().add_modifier(Modifier::SLOW_BLINK),
        )),
        Line::from(Span::styled(
            "RAPID_BLINK text",
            Style::new().add_modifier(Modifier::RAPID_BLINK),
        )),
        Line::from(Span::styled(
            "REVERSED text",
            Style::new().add_modifier(Modifier::REVERSED),
        )),
        Line::from(Span::styled(
            "HIDDEN text",
            Style::new().add_modifier(Modifier::HIDDEN),
        )),
        Line::from(Span::styled(
            "CROSSED_OUT text",
            Style::new().add_modifier(Modifier::CROSSED_OUT),
        )),
        Line::from(Span::styled(
            "BOLD + ITALIC",
            Style::new().add_modifier(Modifier::BOLD | Modifier::ITALIC),
        )),
        Line::from(Span::styled(
            "DIM + UNDERLINED",
            Style::new().add_modifier(Modifier::DIM | Modifier::UNDERLINED),
        )),
        Line::from(Span::raw("Normal text (no modifier)")),
    ];

    let paragraph = Paragraph::new(lines).wrap(Wrap { trim: true });
    let bordered_block = Block::bordered()
        .border_style(Style::new().yellow())
        .title("Modifier Test");
    frame.render_widget(paragraph.block(bordered_block), frame.area());
}
