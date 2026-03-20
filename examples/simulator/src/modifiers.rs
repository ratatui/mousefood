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
//! cargo run -p simulator --bin modifiers
//! ```
//!
//! A window will open with the simulator running.
//! Use arrow keys or WASD to move the cursor.

use embedded_graphics_simulator::{
    OutputSettings, SimulatorDisplay, SimulatorEvent, Window, sdl2::Keycode,
};
use mousefood::embedded_graphics::geometry;
use mousefood::error::Error;
use mousefood::prelude::*;
use ratatui::backend::Backend;
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::{Frame, Terminal, style::*};
use std::cell::RefCell;
use std::rc::Rc;

fn main() -> Result<(), Error> {
    let mut simulator_window = Window::new(
        "mousefood simulator",
        &OutputSettings {
            scale: 4,
            ..Default::default()
        },
    );
    simulator_window.set_max_fps(30);

    let mut display = SimulatorDisplay::<Bgr565>::new(geometry::Size::new(128, 64));

    let events: Rc<RefCell<Vec<SimulatorEvent>>> = Rc::new(RefCell::new(Vec::new()));
    let events_cb = events.clone();

    let backend_config = EmbeddedBackendConfig {
        flush_callback: Box::new(move |display| {
            simulator_window.update(display);
            let mut ev = events_cb.borrow_mut();
            ev.clear();
            ev.extend(simulator_window.events());
        }),
        color_theme: ColorTheme::tokyo_night(),
        ..Default::default()
    };
    let backend: EmbeddedBackend<SimulatorDisplay<_>, _> =
        EmbeddedBackend::new(&mut display, backend_config);

    let mut terminal = Terminal::new(backend)?;
    let mut cursor_x: u16 = 1;
    let mut cursor_y: u16 = 1;

    loop {
        terminal.draw(|frame| draw(frame, cursor_x, cursor_y))?;

        for event in events.borrow().iter() {
            match event {
                SimulatorEvent::KeyDown { keycode, .. } => match *keycode {
                    Keycode::Up | Keycode::W => {
                        cursor_y = cursor_y.saturating_sub(1);
                    }
                    Keycode::Down | Keycode::S => {
                        cursor_y = cursor_y.saturating_add(1);
                    }
                    Keycode::Left | Keycode::A => {
                        cursor_x = cursor_x.saturating_sub(1);
                    }
                    Keycode::Right | Keycode::D => {
                        cursor_x = cursor_x.saturating_add(1);
                    }
                    _ => {}
                },
                SimulatorEvent::Quit => return Ok(()),
                _ => {}
            }
        }

        if let Ok(size) = terminal.backend_mut().size() {
            cursor_x = cursor_x.min(size.width.saturating_sub(1));
            cursor_y = cursor_y.min(size.height.saturating_sub(1));
        }
    }
}

fn draw(frame: &mut Frame, cursor_x: u16, cursor_y: u16) {
    use ratatui::style::Modifier;
    use ratatui::text::{Line, Span};

    let line = Line::from(vec![
        Span::styled(format!("F:{} ", frame.count()), Style::new().yellow()),
        Span::styled("RED ", Style::new().fg(Color::Red)),
        Span::styled(
            "DIM ",
            Style::new().fg(Color::Red).add_modifier(Modifier::DIM),
        ),
        Span::styled("UNDR ", Style::new().add_modifier(Modifier::UNDERLINED)),
        Span::styled("SLOW ", Style::new().add_modifier(Modifier::SLOW_BLINK)),
        Span::styled("FAST ", Style::new().add_modifier(Modifier::RAPID_BLINK)),
        Span::styled("REV ", Style::new().add_modifier(Modifier::REVERSED)),
        Span::styled("HIDE ", Style::new().add_modifier(Modifier::HIDDEN)),
        Span::styled("XOUT ", Style::new().add_modifier(Modifier::CROSSED_OUT)),
        Span::styled(
            "D+U ",
            Style::new().add_modifier(Modifier::DIM | Modifier::UNDERLINED),
        ),
        // combos
        Span::styled(
            "GHOST ",
            Style::new()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM | Modifier::ITALIC),
        ),
        Span::styled(
            "ALARM ",
            Style::new()
                .fg(Color::Red)
                .add_modifier(Modifier::RAPID_BLINK | Modifier::REVERSED),
        ),
        Span::styled(
            "DEAD ",
            Style::new()
                .fg(Color::Gray)
                .add_modifier(Modifier::CROSSED_OUT | Modifier::DIM),
        ),
        Span::styled(
            "SHOUT ",
            Style::new()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        ),
        Span::styled(
            "HAUNT ",
            Style::new()
                .fg(Color::Magenta)
                .add_modifier(Modifier::SLOW_BLINK | Modifier::DIM),
        ),
        Span::styled(
            "CRIT",
            Style::new()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK),
        ),
    ]);

    let paragraph = Paragraph::new(vec![line]).wrap(Wrap { trim: true });
    let bordered_block = Block::bordered()
        .border_style(Style::new().yellow())
        .title("Mods");
    frame.render_widget(paragraph.block(bordered_block), frame.area());
    frame.set_cursor_position((cursor_x, cursor_y));
}
