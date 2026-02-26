use crate::colors::*;
use crate::cursor::{Cursor, CursorConfig};
use crate::default_font;
use crate::error::Result;
use alloc::boxed::Box;
#[cfg(feature = "blink")]
use alloc::collections::BTreeMap;
use core::marker::PhantomData;
use embedded_graphics::Drawable;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{self, Dimensions};
use embedded_graphics::mono_font::{MonoFont, MonoTextStyleBuilder};
use embedded_graphics::pixelcolor::{PixelColor, Rgb888};
use embedded_graphics::text::Text;
use ratatui_core::backend::{Backend, ClearType};
use ratatui_core::layout;
use ratatui_core::style;

/// Terminal alignment
#[derive(Clone, Copy)]
pub enum TerminalAlignment {
    /// Alignment with the start of the terminal: left or top.
    Start,
    /// Best effort alignment with the center of the terminal.
    Center,
    /// Alignment with the end of the terminal: right or bottom.
    End,
}

/// Timing parameters for a single blink pattern.
#[cfg(feature = "blink")]
#[derive(Clone, Copy)]
pub struct BlinkTiming {
    /// How many times per second the element toggles.
    pub blinks_per_sec: u16,
    /// Percentage of each cycle spent hidden (0–100).
    /// e.g. 15 means hidden 15% of each cycle.
    pub duty_percent: u16,
    hidden: bool,
}

#[cfg(feature = "blink")]
impl BlinkTiming {
    /// Returns `true` if the element is currently hidden.
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    /// Update hidden state based on the current frame count and FPS.
    fn update(&mut self, frame_count: u16, fps: u16) {
        if self.blinks_per_sec == 0 || fps == 0 {
            self.hidden = false;
            return;
        }
        let cycle_len = fps / self.blinks_per_sec;
        if cycle_len == 0 {
            self.hidden = false;
            return;
        }
        let pos = frame_count % cycle_len;
        let hidden_frames = ((self.duty_percent * cycle_len + 50) / 100).max(1);
        self.hidden = pos >= cycle_len - hidden_frames;
    }
}

/// Blink configuration for text modifiers and cursor.
///
/// Owns all blink state. Call [`BlinkConfig::tick`] once per frame to advance.
#[cfg(feature = "blink")]
#[derive(Clone, Copy)]
pub struct BlinkConfig {
    /// Display refresh rate. Converts frame counts to time.
    pub fps: u16,
    /// Timing for [`Modifier::SLOW_BLINK`] and cursor blink.
    pub slow: BlinkTiming,
    /// Timing for [`Modifier::RAPID_BLINK`].
    pub fast: BlinkTiming,
    prev_state: (bool, bool),
}

#[cfg(feature = "blink")]
impl BlinkConfig {
    /// Advance blink state for the current frame. Returns `true` if visibility changed.
    pub fn tick(&mut self, frame_count: u16) -> bool {
        self.slow.update(frame_count, self.fps);
        self.fast.update(frame_count, self.fps);
        let state = (self.slow.hidden, self.fast.hidden);
        let toggled = state != self.prev_state;
        self.prev_state = state;
        toggled
    }
}

#[cfg(feature = "blink")]
impl Default for BlinkConfig {
    fn default() -> Self {
        Self {
            fps: 30,
            slow: BlinkTiming {
                blinks_per_sec: 1,
                duty_percent: 15,
                hidden: false,
            },
            fast: BlinkTiming {
                blinks_per_sec: 3,
                duty_percent: 50,
                hidden: false,
            },
            prev_state: (false, false),
        }
    }
}

/// Embedded backend configuration.
pub struct EmbeddedBackendConfig<D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    /// Callback fired after each buffer flush.
    pub flush_callback: Box<dyn FnMut(&mut D)>,
    /// Regular font.
    pub font_regular: MonoFont<'static>,
    /// Bold font.
    pub font_bold: Option<MonoFont<'static>>,
    /// Italic font.
    pub font_italic: Option<MonoFont<'static>>,

    /// Determines how the view is vertically aligned when the display height
    /// is not an exact multiple of the font height.
    pub vertical_alignment: TerminalAlignment,

    /// Determines how the view is horizontally aligned when the display width
    /// is not an exact multiple of the font width.
    pub horizontal_alignment: TerminalAlignment,

    /// Color theme that maps Ratatui colors to display pixels.
    pub color_theme: ColorTheme,

    /// Cursor appearance and blink behavior.
    pub cursor: CursorConfig,

    /// Blink timing for text modifiers and cursor.
    #[cfg(feature = "blink")]
    pub blink: BlinkConfig,
}

impl<D, C> Default for EmbeddedBackendConfig<D, C>
where
    D: DrawTarget<Color = C>,
    C: PixelColor,
{
    fn default() -> Self {
        Self {
            flush_callback: Box::new(|_| {}),
            font_regular: default_font::get_regular(),
            font_bold: None,
            font_italic: None,
            vertical_alignment: TerminalAlignment::Start,
            horizontal_alignment: TerminalAlignment::Start,
            color_theme: ColorTheme::default(),
            cursor: CursorConfig::default(),
            #[cfg(feature = "blink")]
            blink: BlinkConfig::default(),
        }
    }
}

/// Embedded backend for Ratatui.
///
/// # Examples
///
/// ```rust
/// use mousefood::embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888};
/// use mousefood::prelude::*;
/// use ratatui::widgets::{Block, Paragraph};
/// use ratatui::{Frame, Terminal};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut display = MockDisplay::<Rgb888>::new();
///     let backend = EmbeddedBackend::new(&mut display, EmbeddedBackendConfig::default());
///     let mut terminal = Terminal::new(backend)?;
///
///     terminal.draw(draw)?;
///     Ok(())
/// }
///
/// fn draw(frame: &mut Frame) {
///     let block = Block::bordered().title("Mousefood");
///     let paragraph = Paragraph::new("Hello from Mousefood!").block(block);
///     frame.render_widget(paragraph, frame.area());
/// }
/// ```
pub struct EmbeddedBackend<'display, D, C>
where
    D: DrawTarget<Color = C> + 'display,
    C: PixelColor + 'display,
{
    display: &'display mut D,
    display_type: PhantomData<D>,

    flush_callback: Box<dyn FnMut(&mut D)>,

    #[cfg(feature = "framebuffer")]
    buffer: crate::framebuffer::HeapBuffer<C>,

    font_regular: MonoFont<'static>,
    font_bold: Option<MonoFont<'static>>,
    font_italic: Option<MonoFont<'static>>,

    char_offset: geometry::Point,

    columns_rows: layout::Size,
    pixels: layout::Size,
    color_theme: ColorTheme,
    cursor: Cursor,
    #[cfg(feature = "blink")]
    frame_count: u16,
    #[cfg(feature = "blink")]
    blink_config: BlinkConfig,
    #[cfg(feature = "blink")]
    blink_cells: BTreeMap<(u16, u16), ratatui_core::buffer::Cell>,
}

impl<'display, D, C> EmbeddedBackend<'display, D, C>
where
    D: DrawTarget<Color = C> + Dimensions + 'static,
    C: PixelColor + Into<Rgb888> + From<Rgb888> + for<'a> From<TermColor<'a>> + 'static,
{
    fn init(
        display: &'display mut D,
        config: EmbeddedBackendConfig<D, C>,
    ) -> EmbeddedBackend<'display, D, C> {
        let EmbeddedBackendConfig {
            flush_callback,
            font_regular,
            font_bold,
            font_italic,
            vertical_alignment,
            horizontal_alignment,
            color_theme,
            cursor,
            #[cfg(feature = "blink")]
            blink,
        } = config;
        let pixels = layout::Size {
            width: display.bounding_box().size.width as u16,
            height: display.bounding_box().size.height as u16,
        };

        let extra_x = pixels.width % font_regular.character_size.width as u16;
        let extra_y = pixels.height % font_regular.character_size.height as u16;

        let off_x = match horizontal_alignment {
            TerminalAlignment::Start => 0,
            TerminalAlignment::Center => extra_x / 2,
            TerminalAlignment::End => extra_x,
        } as i32;
        let off_y = match vertical_alignment {
            TerminalAlignment::Start => 0,
            TerminalAlignment::Center => extra_y / 2,
            TerminalAlignment::End => extra_y,
        } as i32;

        let char_offset = geometry::Point::new(off_x, off_y);

        Self {
            #[cfg(feature = "framebuffer")]
            buffer: crate::framebuffer::HeapBuffer::new(display.bounding_box(), color_theme),
            display,
            display_type: PhantomData,
            flush_callback: Box::new(flush_callback),
            font_regular,
            font_bold,
            font_italic,
            char_offset,
            columns_rows: layout::Size {
                height: pixels.height / font_regular.character_size.height as u16,
                width: pixels.width / font_regular.character_size.width as u16,
            },
            pixels,
            color_theme,
            cursor: Cursor::new(cursor),
            #[cfg(feature = "blink")]
            frame_count: 0,
            #[cfg(feature = "blink")]
            blink_config: blink,
            #[cfg(feature = "blink")]
            blink_cells: BTreeMap::new(),
        }
    }

    /// Creates a new `EmbeddedBackend` using default fonts.
    pub fn new(
        display: &'display mut D,
        config: EmbeddedBackendConfig<D, C>,
    ) -> EmbeddedBackend<'display, D, C> {
        Self::init(display, config)
    }

    /// Borrow the display
    pub fn display(&self) -> &D {
        self.display
    }

    /// Mutably borrow the display
    pub fn display_mut(&mut self) -> &mut D {
        self.display
    }
}

impl<D, C> Backend for EmbeddedBackend<'_, D, C>
where
    D: DrawTarget<Color = C> + 'static,
    C: PixelColor + Into<Rgb888> + From<Rgb888> + for<'a> From<TermColor<'a>> + 'static,
{
    type Error = crate::error::Error;

    fn draw<'a, I>(&mut self, content: I) -> Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a ratatui_core::buffer::Cell)>,
    {
        #[cfg(feature = "blink")]
        {
            self.frame_count = self.frame_count.wrapping_add(1);
            let blink_toggled = self.blink_config.tick(self.frame_count);
            if blink_toggled {
                self.redraw_blink_cells()?;
            }
        }

        for (x, y, cell) in content {
            #[cfg(feature = "blink")]
            self.track_blink_cell(x, y, cell);

            self.draw_cell(x, y, cell)?;
        }

        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<()> {
        self.cursor.visible = false;
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<()> {
        self.cursor.visible = true;
        Ok(())
    }

    fn get_cursor_position(&mut self) -> Result<layout::Position> {
        Ok(self.cursor.position)
    }

    fn set_cursor_position<P: Into<layout::Position>>(&mut self, position: P) -> Result<()> {
        self.cursor.position = position.into();
        Ok(())
    }

    #[cfg(feature = "framebuffer")]
    fn clear(&mut self) -> Result<()> {
        self.buffer
            .clear(
                TermColor::new(
                    style::Color::Reset,
                    TermColorType::Background,
                    &self.color_theme,
                )
                .into(),
            )
            .map_err(|_| crate::error::Error::DrawError)
    }

    #[cfg(not(feature = "framebuffer"))]
    fn clear(&mut self) -> Result<()> {
        self.display
            .clear(
                TermColor::new(
                    style::Color::Reset,
                    TermColorType::Background,
                    &self.color_theme,
                )
                .into(),
            )
            .map_err(|_| crate::error::Error::DrawError)
    }

    fn clear_region(&mut self, clear_type: ClearType) -> Result<()> {
        match clear_type {
            ClearType::All => self.clear(),
            ClearType::AfterCursor
            | ClearType::BeforeCursor
            | ClearType::CurrentLine
            | ClearType::UntilNewLine => Err(crate::error::Error::ClearTypeUnsupported(
                alloc::format!("{:?}", clear_type),
            )),
        }
    }

    fn size(&self) -> Result<layout::Size> {
        Ok(self.columns_rows)
    }

    fn window_size(&mut self) -> Result<ratatui_core::backend::WindowSize> {
        Ok(ratatui_core::backend::WindowSize {
            columns_rows: self.columns_rows,
            pixels: self.pixels,
        })
    }

    fn flush(&mut self) -> Result<()> {
        #[cfg(feature = "framebuffer")]
        self.display
            .fill_contiguous(&self.display.bounding_box(), &self.buffer)
            .map_err(|_| crate::error::Error::DrawError)?;

        if self.cursor.visible {
            #[cfg(feature = "blink")]
            let hidden = self.cursor.config.blink && self.blink_config.slow.is_hidden();
            #[cfg(not(feature = "blink"))]
            let hidden = false;

            if !hidden {
                let char_w = self.font_regular.character_size.width as i32;
                let char_h = self.font_regular.character_size.height as i32;
                self.cursor.draw(
                    self.display,
                    #[cfg(feature = "framebuffer")]
                    &self.buffer,
                    self.char_offset,
                    char_w,
                    char_h,
                )?;
            }
        }

        (self.flush_callback)(self.display);
        Ok(())
    }
}

impl<D, C> EmbeddedBackend<'_, D, C>
where
    D: DrawTarget<Color = C> + 'static,
    C: PixelColor + Into<Rgb888> + From<Rgb888> + for<'a> From<TermColor<'a>> + 'static,
{
    fn draw_cell(&mut self, x: u16, y: u16, cell: &ratatui_core::buffer::Cell) -> Result<()> {
        let position = geometry::Point::new(
            x as i32 * self.font_regular.character_size.width as i32,
            y as i32 * self.font_regular.character_size.height as i32,
        );
        let mut fg_color: C =
            TermColor::new(cell.fg, TermColorType::Foreground, &self.color_theme).into();
        let mut bg_color: C =
            TermColor::new(cell.bg, TermColorType::Background, &self.color_theme).into();
        let mut style_builder = MonoTextStyleBuilder::new()
            .font(&self.font_regular)
            .text_color(fg_color)
            .background_color(bg_color);

        for modifier in cell.modifier.iter() {
            style_builder = match modifier {
                style::Modifier::BOLD => match &self.font_bold {
                    None => style_builder,
                    Some(font) => style_builder.font(font),
                },
                style::Modifier::DIM => {
                    fg_color = dim_color(fg_color);
                    style_builder
                }
                style::Modifier::ITALIC => match &self.font_italic {
                    None => style_builder,
                    Some(font) => style_builder.font(font),
                },
                style::Modifier::UNDERLINED => style_builder.underline(),
                #[cfg(feature = "blink")]
                style::Modifier::SLOW_BLINK => {
                    if self.blink_config.slow.is_hidden() {
                        fg_color = bg_color;
                    }
                    style_builder
                }
                #[cfg(feature = "blink")]
                style::Modifier::RAPID_BLINK => {
                    if self.blink_config.fast.is_hidden() {
                        fg_color = bg_color;
                    }
                    style_builder
                }
                style::Modifier::REVERSED => {
                    core::mem::swap(&mut fg_color, &mut bg_color);
                    style_builder
                }
                style::Modifier::HIDDEN => {
                    fg_color = bg_color;
                    style_builder
                }
                style::Modifier::CROSSED_OUT => style_builder.strikethrough(),
                _ => style_builder,
            }
        }

        style_builder = style_builder
            .text_color(fg_color)
            .background_color(bg_color);

        #[cfg(feature = "underline-color")]
        if cell.underline_color != style::Color::Reset {
            style_builder = style_builder.underline_with_color(
                TermColor::new(
                    cell.underline_color,
                    TermColorType::Foreground,
                    &self.color_theme,
                )
                .into(),
            );
        }

        Text::with_baseline(
            cell.symbol(),
            position + self.char_offset,
            style_builder.build(),
            embedded_graphics::text::Baseline::Top,
        )
        .draw(
            #[cfg(feature = "framebuffer")]
            &mut self.buffer,
            #[cfg(not(feature = "framebuffer"))]
            self.display,
        )
        .map_err(|_| crate::error::Error::DrawError)?;

        Ok(())
    }

    #[cfg(feature = "blink")]
    fn track_blink_cell(&mut self, x: u16, y: u16, cell: &ratatui_core::buffer::Cell) {
        if cell.modifier.contains(style::Modifier::SLOW_BLINK)
            || cell.modifier.contains(style::Modifier::RAPID_BLINK)
        {
            self.blink_cells.insert((x, y), cell.clone());
        } else {
            self.blink_cells.remove(&(x, y));
        }
    }

    #[cfg(feature = "blink")]
    fn redraw_blink_cells(&mut self) -> Result<()> {
        if self.blink_cells.is_empty() {
            return Ok(());
        }
        let cells = core::mem::take(&mut self.blink_cells);
        for (&(x, y), cell) in &cells {
            self.draw_cell(x, y, cell)?;
        }
        self.blink_cells = cells;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        embedded_graphics::{
            mock_display::MockDisplay,
            mono_font::{MonoTextStyle, ascii::FONT_4X6},
            pixelcolor::{Rgb888, RgbColor},
            prelude::*,
            text::{Alignment, LineHeight, Text, TextStyleBuilder},
        },
        ratatui::Terminal,
        rstest::{fixture, rstest},
    };

    #[fixture]
    fn display0() -> MockDisplay<Rgb888> {
        let mut d = MockDisplay::new();
        d.set_allow_overdraw(true);
        d
    }

    #[fixture]
    fn display1() -> MockDisplay<Rgb888> {
        display0()
    }

    #[rstest]
    fn renders_direct_as_expected(
        mut display0: MockDisplay<Rgb888>,
        mut display1: MockDisplay<Rgb888>,
    ) {
        let config = || EmbeddedBackendConfig {
            font_regular: FONT_4X6,
            font_bold: None,
            vertical_alignment: TerminalAlignment::Start,
            horizontal_alignment: TerminalAlignment::Start,
            ..Default::default()
        };

        //render "T" via ratatui and then " est" directly to the display retrieved from the backend
        {
            let backend = EmbeddedBackend::new(&mut display0, config());
            let mut terminal = Terminal::new(backend).expect("to create terminal");
            terminal
                .draw(|frame| {
                    use ratatui::text::Line;
                    let content = Line::from("T").left_aligned();
                    frame.render_widget(content, frame.area());
                })
                .expect("to draw");

            let display = terminal.backend_mut().display_mut();

            let text = {
                let text_style = TextStyleBuilder::new()
                    .alignment(Alignment::Left)
                    .line_height(LineHeight::Percent(100))
                    .baseline(embedded_graphics::text::Baseline::Top)
                    .build();

                Text::with_text_style(
                    " est",
                    Point::new(0, 0),
                    MonoTextStyle::new(&FONT_4X6, Rgb888::WHITE),
                    text_style,
                )
            };
            text.draw(display).unwrap();
        }

        //render "Test" via ratatui
        {
            let backend = EmbeddedBackend::new(&mut display1, config());
            let mut terminal = Terminal::new(backend).expect("to create terminal");

            terminal
                .draw(|frame| {
                    use ratatui::text::Line;
                    let content = Line::from("Test").left_aligned();
                    frame.render_widget(content, frame.area());
                })
                .expect("to draw");
        }

        display0.assert_eq(&display1);
    }
}
