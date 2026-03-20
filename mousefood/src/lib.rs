#![no_std]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
extern crate alloc;

mod backend;
mod colors;
mod default_font;
pub mod error;
#[cfg(feature = "framebuffer")]
mod framebuffer;
mod macros;
pub mod prelude;

#[cfg(feature = "blink")]
pub use backend::{BlinkConfig, BlinkTiming};
pub use backend::{EmbeddedBackend, EmbeddedBackendConfig, TerminalAlignment};
pub use colors::ColorTheme;
pub use embedded_graphics;
pub mod cursor;
pub use cursor::{CursorConfig, CursorStyle};

#[cfg(feature = "fonts")]
pub use embedded_graphics_unicodefonts as fonts;
