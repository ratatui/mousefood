//! Cursor configuration, styles, and rendering.

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry;
use embedded_graphics::pixelcolor::{PixelColor, Rgb888};
use embedded_graphics::prelude::RgbColor;
use ratatui_core::layout;

/// How the cursor is rendered on screen.
#[derive(Clone, Copy, PartialEq)]
pub enum CursorStyle {
    /// Invert all pixels in the character cell (requires framebuffer).
    /// Falls back to `Underline` without framebuffer.
    Inverse,
    /// Thin line at the bottom of the character cell.
    Underline,
    /// Outline around the character cell.
    Outline,
    /// Corner brackets — top-left and bottom-right corners.
    Japanese,
}

/// Cursor appearance and behavior.
#[derive(Clone, Copy)]
pub struct CursorConfig {
    /// Visual style of the cursor.
    pub style: CursorStyle,
    /// Whether the cursor blinks. Uses `BlinkConfig::slow` timing.
    #[cfg(feature = "blink")]
    pub blink: bool,
    /// Cursor color for non-inverse styles.
    pub color: Rgb888,
}

impl Default for CursorConfig {
    fn default() -> Self {
        Self {
            style: CursorStyle::Inverse,
            #[cfg(feature = "blink")]
            blink: true,
            color: Rgb888::WHITE,
        }
    }
}

pub(crate) struct Cursor {
    pub visible: bool,
    pub position: layout::Position,
    pub config: CursorConfig,
}

impl Cursor {
    pub fn new(config: CursorConfig) -> Self {
        Self {
            visible: false,
            position: layout::Position::new(0, 0),
            config,
        }
    }

    pub fn draw<D, C>(
        &self,
        display: &mut D,
        #[cfg(feature = "framebuffer")] buffer: &crate::framebuffer::HeapBuffer<C>,
        char_offset: geometry::Point,
        char_w: i32,
        char_h: i32,
    ) -> crate::error::Result<()>
    where
        D: DrawTarget<Color = C>,
        C: PixelColor + Into<Rgb888> + From<Rgb888>,
    {
        let top_left = geometry::Point::new(
            self.position.x as i32 * char_w,
            self.position.y as i32 * char_h,
        ) + char_offset;

        match self.config.style {
            #[cfg(feature = "framebuffer")]
            CursorStyle::Inverse => Self::draw_inverse(display, buffer, top_left, char_w, char_h),

            #[cfg(not(feature = "framebuffer"))]
            CursorStyle::Inverse => {
                let color: C = self.config.color.into();
                Self::draw_line(display, top_left, char_h - 1, 0, char_w, 1, color)
            }

            CursorStyle::Underline => {
                let color: C = self.config.color.into();
                Self::draw_line(display, top_left, char_h - 1, 0, char_w, 1, color)
            }

            CursorStyle::Outline => {
                let color: C = self.config.color.into();
                Self::draw_line(display, top_left, 0, 0, char_w, 1, color)?;
                Self::draw_line(display, top_left, char_h - 1, 0, char_w, 1, color)?;
                Self::draw_line(display, top_left, 0, 0, 1, char_h, color)?;
                Self::draw_line(display, top_left, 0, char_w - 1, 1, char_h, color)
            }

            CursorStyle::Japanese => {
                let color: C = self.config.color.into();
                let corner = (char_w / 2).max(2);
                Self::draw_line(display, top_left, 0, 0, corner, 1, color)?;
                Self::draw_line(display, top_left, 0, 0, 1, corner, color)?;
                Self::draw_line(
                    display,
                    top_left,
                    char_h - corner,
                    char_w - 1,
                    1,
                    corner,
                    color,
                )?;
                Self::draw_line(
                    display,
                    top_left,
                    char_h - 1,
                    char_w - corner,
                    corner,
                    1,
                    color,
                )
            }
        }
    }

    fn draw_line<D, C>(
        display: &mut D,
        top_left: geometry::Point,
        dy: i32,
        dx: i32,
        w: i32,
        h: i32,
        color: C,
    ) -> crate::error::Result<()>
    where
        D: DrawTarget<Color = C>,
        C: PixelColor,
    {
        display
            .fill_solid(
                &embedded_graphics::primitives::Rectangle::new(
                    geometry::Point::new(top_left.x + dx, top_left.y + dy),
                    geometry::Size::new(w as u32, h as u32),
                ),
                color,
            )
            .map_err(|_| crate::error::Error::DrawError)
    }

    #[cfg(feature = "framebuffer")]
    fn draw_inverse<D, C>(
        display: &mut D,
        buffer: &crate::framebuffer::HeapBuffer<C>,
        top_left: geometry::Point,
        char_w: i32,
        char_h: i32,
    ) -> crate::error::Result<()>
    where
        D: DrawTarget<Color = C>,
        C: PixelColor + Into<Rgb888> + From<Rgb888>,
    {
        for y in top_left.y..top_left.y + char_h {
            let row_rect = embedded_graphics::primitives::Rectangle::new(
                geometry::Point::new(top_left.x, y),
                geometry::Size::new(char_w as u32, 1),
            );
            display
                .fill_contiguous(
                    &row_rect,
                    (top_left.x..top_left.x + char_w).map(|x| {
                        let rgb: Rgb888 = buffer.get_pixel(geometry::Point::new(x, y)).into();
                        Rgb888::new(!rgb.r(), !rgb.g(), !rgb.b()).into()
                    }),
                )
                .map_err(|_| crate::error::Error::DrawError)?;
        }
        Ok(())
    }
}
