use itertools::Itertools;
use ratatui_core::buffer::Buffer;
use ratatui_core::layout::Rect;
use ratatui_core::style::Color;
use ratatui_core::widgets::Widget;

const MOUSEFOOD_LOGO: &str = indoc::indoc! {"
                             abccccccc
dddd   ddd  d   d ddddd ddeebbbbbbbacc
d d d d   d d   d d     d   babbbcaaccc
d d d d   d d   d ddddd eee  abbbbabccc
d d d d   d d   d     e e bbabbbbbbccc
d d d  ddd   ddd  deeee eebbbbbbbbbcccc
                         bbbcabbbbcccbcc
ddddd  ddd   ddd  eeee  bbbcaabbbccccbac
d     d   d d   e e   e bbbbabbbbccccccc
ddd   d   d e   e e   ebaabbbcaacccbcccc
d     d   e e   e e   e abbbbbabcccbcc
d      dde   eeeb eeee aabbbbbcbccabc
               abc  b bbbbccacccc
"};

const EMPTY: char = ' ';
const LIGHT_TEXT: char = 'd';
const DARK_TEXT: char = 'e';
const LIGHT_CHEESE: char = 'c';
const MIDDLE_CHEESE: char = 'b';
const DARK_CHEESE: char = 'a';
const TERM: char = '░';
const TERM_BORDER: char = '▒';
const TERM_CURSOR: char = '▓';

#[derive(Default)]
pub struct MouseFoodLogo;

impl MouseFoodLogo {
    pub fn new() -> Self {
        Self::default()
    }

    const fn color_for(&self, c: char) -> Option<Color> {
        match c {
            LIGHT_TEXT => Some(Color::Rgb(139, 151, 182)), // #8b97b6
            DARK_TEXT => Some(Color::Rgb(53, 54, 88)),     // #353658
            LIGHT_CHEESE => Some(Color::Rgb(236, 233, 16)), // #ece910
            MIDDLE_CHEESE => Some(Color::Rgb(236, 171, 17)), // #ecab11
            DARK_CHEESE => Some(Color::Rgb(239, 110, 16)), // #ef6e10
            TERM => Some(Color::Indexed(232)),
            TERM_BORDER => Some(Color::Indexed(237)),
            TERM_CURSOR => Some(Color::Indexed(248)),
            _ => None,
        }
    }
}

impl Widget for MouseFoodLogo {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        if area.is_empty() {
            return;
        }

        for (y, (line1, line2)) in MOUSEFOOD_LOGO.lines().tuples().enumerate() {
            for (x, (ch1, ch2)) in line1.chars().zip(line2.chars()).enumerate() {
                let x = area.left() + x as u16;
                let y = area.top() + y as u16;

                // Check if coordinates are within the buffer area
                if x >= area.right() || y >= area.bottom() {
                    continue;
                }

                let cell = &mut buf[(x, y)];
                // given two cells which make up the top and bottom of the character,
                // Foreground color should be the non-space, non-terminal
                let (fg, bg) = match (ch1, ch2) {
                    (EMPTY, EMPTY) => (None, None),
                    (c, EMPTY) | (EMPTY, c) => (self.color_for(c), None),
                    (TERM, TERM_BORDER) => (self.color_for(TERM_BORDER), self.color_for(TERM)),
                    (TERM, c) | (c, TERM) => (self.color_for(c), self.color_for(TERM)),
                    (c1, c2) => (self.color_for(c1), self.color_for(c2)),
                };
                // symbol should make the empty space or terminal bg as the empty part of the block
                let symbol = match (ch1, ch2) {
                    (EMPTY, EMPTY) => None,
                    (TERM, TERM) => Some(EMPTY),
                    (_, EMPTY | TERM) => Some('▀'),
                    (EMPTY | TERM, _) => Some('▄'),
                    (c, d) if c == d => Some('█'),
                    (_, _) => Some('▀'),
                };
                if let Some(fg) = fg {
                    cell.fg = fg;
                }
                if let Some(bg) = bg {
                    cell.bg = bg;
                }
                if let Some(symb) = symbol {
                    cell.set_char(symb);
                }
            }
        }
    }
}
