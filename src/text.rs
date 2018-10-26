use std::collections::HashMap;
#[cfg(feature = "graphics")]
use std::fmt;

#[cfg(feature = "graphics")]
use graphics::{
    character::CharacterCache, math::Matrix2d, text as draw_text, Graphics, ImageSize, Transformed,
};
use rusttype::{Error, Font, GlyphId, Scale};

use math::{Rectangle, Scalar, Vector2, ZeroOneTwo};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Justification {
    Left,
    Centered,
    Right,
}

pub type PositionedLines<V> = Vec<(V, String)>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TextFormat<S>
where
    S: Scalar,
{
    pub font_size: u32,
    pub just: Justification,
    pub line_spacing: S,
    pub first_line_indent: usize,
    pub lines_indent: usize,
    pub color: [f32; 4],
}

impl<S> TextFormat<S>
where
    S: Scalar,
{
    pub fn new(font_size: u32) -> TextFormat<S> {
        TextFormat {
            font_size,
            just: Justification::Left,
            line_spacing: S::ONE,
            first_line_indent: 0,
            lines_indent: 0,
            color: [1.0; 4],
        }
    }
    pub fn left(mut self) -> Self {
        self.just = Justification::Left;
        self
    }
    pub fn centered(mut self) -> Self {
        self.just = Justification::Centered;
        self
    }
    pub fn right(mut self) -> Self {
        self.just = Justification::Right;
        self
    }
    pub fn font_size(mut self, font_size: u32) -> Self {
        self.font_size = font_size;
        self
    }
    pub fn line_spacing(mut self, line_spacing: S) -> Self {
        self.line_spacing = line_spacing;
        self
    }
    pub fn first_line_indent(mut self, first_line_indent: usize) -> Self {
        self.first_line_indent = first_line_indent;
        self
    }
    pub fn lines_indent(mut self, lines_indent: usize) -> Self {
        self.lines_indent = lines_indent;
        self
    }
    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }
}

pub trait CharacterWidthCache {
    type Scalar: Scalar;
    fn char_width(&mut self, character: char, font_size: u32) -> Self::Scalar;
    fn width(&mut self, text: &str, font_size: u32) -> Self::Scalar {
        text.chars()
            .map(|c| self.char_width(c, font_size))
            .fold(Self::Scalar::ZERO, std::ops::Add::add)
    }
    fn format_lines(
        &mut self,
        text: &str,
        max_width: Self::Scalar,
        format: TextFormat<Self::Scalar>,
    ) -> Vec<String> {
        let mut sized_lines = Vec::new();
        let mut first_line = false;
        // Iterate through lines
        for line in text.lines() {
            // Initialize a result line
            let mut sized_line = String::new();
            // Apply the indentation
            let indent = (0..if first_line {
                format.first_line_indent
            } else {
                format.lines_indent
            })
                .map(|_| ' ')
                .collect::<String>();
            sized_line.push_str(&indent);
            let mut curr_width = self.width(&indent, format.font_size);
            // Iterate through words
            for word in line.split_whitespace() {
                // Get the word's width
                let width = self.width(word, format.font_size);
                // If the word goes past the max width...
                if !(curr_width + width < max_width || curr_width == Self::Scalar::ZERO) {
                    // Pop off the trailing space
                    sized_line.pop();
                    // Push the result line onto the result list
                    sized_lines.push(sized_line);
                    // Init next line
                    first_line = false;
                    sized_line = String::new();
                    // Apply the indentation
                    let indent = (0..if first_line {
                        format.first_line_indent
                    } else {
                        format.lines_indent
                    })
                        .map(|_| ' ')
                        .collect::<String>();
                    sized_line.push_str(&indent);
                    curr_width = self.width(&indent, format.font_size);
                }
                // Push the word onto the result line
                sized_line.push_str(word);
                sized_line.push(' ');
                curr_width = curr_width + width + self.char_width(' ', format.font_size);
            }
            // Push the result line onto the result list
            sized_line.pop();
            sized_lines.push(sized_line);
            first_line = false;
        }
        sized_lines
    }
    fn max_line_width(
        &mut self,
        text: &str,
        max_width: Self::Scalar,
        format: TextFormat<Self::Scalar>,
    ) -> Self::Scalar {
        let lines = self.format_lines(text, max_width, format);
        lines
            .into_iter()
            .map(|line| self.width(&line, format.font_size))
            .max_by(|a, b| a.partial_cmp(b).expect("Incomperable scalars. Is one NaN?"))
            .unwrap_or(Self::Scalar::ZERO)
    }
    fn justify_text<R>(
        &mut self,
        text: &str,
        rect: R,
        format: TextFormat<Self::Scalar>,
    ) -> PositionedLines<R::Vector>
    where
        R: Rectangle<Scalar = Self::Scalar>,
    {
        self.format_lines(text, rect.width(), format)
            .into_iter()
            .enumerate()
            .map(|(i, line)| {
                let y_offset = rect.top()
                    + format.font_size.into()
                    + Self::Scalar::from(i as u32) * format.font_size.into() * format.line_spacing;
                use self::Justification::*;
                let line_width = self.width(&line, format.font_size);
                let x_offset = match format.just {
                    Left => rect.left(),
                    Centered => rect.center().x() - line_width / Self::Scalar::TWO,
                    Right => rect.right() - line_width,
                };
                (R::Vector::new(x_offset, y_offset), line)
            }).collect()
    }
    fn text_fits<R>(&mut self, text: &str, rect: R, format: TextFormat<Self::Scalar>) -> bool
    where
        R: Rectangle<Scalar = Self::Scalar>,
    {
        self.max_line_width(text, rect.width(), format) < rect.width() && {
            let lines = self.format_lines(text, rect.width(), format);
            let last_line_y = rect.top()
                + format.font_size.into()
                + Self::Scalar::from((lines.len() - 1) as u32)
                    * format.font_size.into()
                    * format.line_spacing;
            last_line_y < rect.bottom()
        }
    }
    fn fit_max_font_size<R>(
        &mut self,
        text: &str,
        rect: R,
        mut format: TextFormat<Self::Scalar>,
    ) -> u32
    where
        R: Rectangle<Scalar = Self::Scalar>,
    {
        while !self.text_fits(text, rect.clone(), format) {
            format.font_size -= 1;
        }
        format.font_size
    }
    fn fit_min_height<R>(
        &mut self,
        text: &str,
        mut rect: R,
        format: TextFormat<Self::Scalar>,
        delta: Self::Scalar,
    ) -> Self::Scalar
    where
        R: Rectangle<Scalar = Self::Scalar>,
    {
        let delta = delta.abs().max(Self::Scalar::ONE);
        while self.text_fits(text, rect.clone(), format) {
            rect = rect
                .clone()
                .with_size(R::Vector::new(rect.width(), rect.height() - delta))
        }
        while !self.text_fits(text, rect.clone(), format) {
            rect = rect
                .clone()
                .with_size(R::Vector::new(rect.width(), rect.height() + delta))
        }
        rect.height()
    }
    fn fit_min_width<R>(
        &mut self,
        text: &str,
        mut rect: R,
        format: TextFormat<Self::Scalar>,
        delta: Self::Scalar,
    ) -> Self::Scalar
    where
        R: Rectangle<Scalar = Self::Scalar>,
    {
        let delta = delta.abs().max(Self::Scalar::ONE);
        while self.text_fits(text, rect.clone(), format) {
            rect = rect
                .clone()
                .with_size(R::Vector::new(rect.width() - delta, rect.height()))
        }
        while !self.text_fits(text, rect.clone(), format) {
            rect = rect
                .clone()
                .with_size(R::Vector::new(rect.width() + delta, rect.height()))
        }
        rect.width()
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
    C: CharacterCache,
    C::Error: fmt::Debug,
{
    type Scalar = f64;
    fn char_width(&mut self, character: char, font_size: u32) -> Self::Scalar {
        <Self as CharacterCache>::character(self, font_size, character)
            .unwrap_or_else(|e| panic!("{:?}", e))
            .size[0]
    }
}

#[cfg(feature = "graphics")]
pub fn justified_text<R, T, C, G>(
    text: &str,
    rect: R,
    format: TextFormat<R::Scalar>,
    glyphs: &mut C,
    transform: Matrix2d,
    graphics: &mut G,
) -> Result<(), C::Error>
where
    R: Rectangle<Scalar = f64>,
    T: ImageSize,
    C: CharacterCache<Texture = T>,
    C::Error: fmt::Debug,
    G: Graphics<Texture = T>,
{
    for (pos, line) in glyphs.justify_text(text, rect, format) {
        draw_text(
            format.color,
            format.font_size,
            &line,
            glyphs,
            transform.trans(pos.x(), pos.y()),
            graphics,
        )?;
    }
    Ok(())
}
