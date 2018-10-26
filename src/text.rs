use std::collections::HashMap;
#[cfg(feature = "graphics")]
use std::fmt;

use rusttype::{Error, Font, GlyphId, Scale};

use math::{Rectangle, Scalar, ZeroOneTwo};

pub enum Justification {
    Left,
    Center,
    Right,
}

pub type PositionedChars<V> = Vec<(V, char)>;

pub trait TextBox: Rectangle {}

impl<R> TextBox for R where R: Rectangle {}

pub trait CharacterWidthCache {
    type Scalar: Scalar;
    fn char_width(&mut self, character: char, font_size: u32) -> Self::Scalar;
    fn width(&mut self, text: &str, font_size: u32) -> Self::Scalar {
        text.chars().map(|c| self.char_width(c, font_size)).sum()
    }
    fn max_width_lines(
        &mut self,
        text: &str,
        font_size: u32,
        max_width: Self::Scalar,
    ) -> Vec<String> {
        let mut curr_width = Self::Scalar::ZERO;
        let mut sized_lines = Vec::new();
        for line in text.lines() {
            let mut sized_line = String::new();
            for word in line.split_whitespace() {
                let width = self.width(word, font_size);
                if !(curr_width + width < max_width || curr_width == Self::Scalar::ZERO) {
                    curr_width = Self::Scalar::ZERO;
                    sized_line.pop();
                    if !sized_line.is_empty() {
                        sized_lines.push(sized_line);
                    }
                    sized_line = String::new();
                }
                sized_line.push_str(word);
                sized_line.push(' ');
                curr_width = curr_width + width + self.char_width(' ', font_size);
            }
            if !sized_line.is_empty() {
                sized_lines.push(sized_line);
            }
        }
        sized_lines
    }
}

#[derive(Clone)]
pub struct Glyphs<'f, S = f64>
where
    S: Scalar,
{
    widths: HashMap<(u32, char), S>,
    font: Font<'f>,
}

impl<'f, S> Glyphs<'f, S>
where
    S: Scalar,
{
    /// Loads a `Glyphs` from an array of font data.
    pub fn from_bytes(bytes: &'f [u8]) -> Result<Glyphs<'f, S>, Error> {
        Ok(Glyphs {
            widths: HashMap::new(),
            font: Font::from_bytes(bytes)?,
        })
    }
    /// Loads a `Glyphs` from a `Font`.
    pub fn from_font(font: Font<'f>) -> Glyphs<'f, S> {
        Glyphs {
            widths: HashMap::new(),
            font,
        }
    }
}

impl<'f, S> CharacterWidthCache for Glyphs<'f, S>
where
    S: Scalar,
{
    type Scalar = S;
    fn char_width(&mut self, character: char, font_size: u32) -> Self::Scalar {
        let font = &self.font;
        *self
            .widths
            .entry((font_size, character))
            .or_insert_with(|| {
                let scale = Scale::uniform(font_size as f32);
                let glyph = font.glyph(character).scaled(scale);
                let glyph = if glyph.id() == GlyphId(0) && glyph.shape().is_none() {
                    font.glyph('\u{FFFD}').scaled(scale)
                } else {
                    glyph
                };
                let h_metrics = glyph.h_metrics();

                h_metrics.advance_width.into()
            })
    }
}

#[cfg(feature = "graphics")]
impl<C> CharacterWidthCache for C
where
    C: graphics::character::CharacterCache,
    C::Error: fmt::Debug,
{
    type Scalar = f64;
    fn char_width(&mut self, character: char, font_size: u32) -> Self::Scalar {
        <Self as graphics::character::CharacterCache>::character(self, font_size, character)
            .unwrap_or_else(|e| panic!("{:?}", e))
            .size[0]
    }
}
