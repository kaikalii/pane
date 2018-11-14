use std::collections::HashMap;
#[cfg(feature = "graphics")]
use std::fmt;

#[cfg(feature = "graphics")]
use graphics::{
    character::CharacterCache, math::Matrix2d, text as draw_text, Graphics, ImageSize, Transformed,
};
use rusttype::{Error, Font, GlyphId, Scale};

use math::{Rectangle, Scalar, Vector2, ZeroOneTwo};

/// A horizantal text justification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Justification {
    /// Align on the left
    Left,
    /// Center align
    Centered,
    /// Align on the right
    Right,
}

/// Lines that have starting positions
///
/// `V` usually implements `Vector2`
pub type PositionedLines<V> = Vec<(V, String)>;

/// A way of resizing text in a rectangle
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Resize {
    /// Make the text no larger than its original font size,
    /// but still try to fit it in the rectangle
    NoLarger,
    /// Make the text as large as possible while still
    /// fitting in the rectangle
    Max,
    /// Do not resize the text
    None,
}

/// A format for some text
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TextFormat<S>
where
    S: Scalar,
{
    /// The font size
    pub font_size: u32,
    /// The horizantal justification
    pub just: Justification,
    /// The spacing between lines. This should usually be somewhere
    /// between `1.0` and `2.0`, but any scalar is valid
    pub line_spacing: S,
    /// The number of spaces to indent the first line of a paragraph
    pub first_line_indent: usize,
    /// The number of spaces to indent all lines of a paragraph
    /// after the first
    pub lines_indent: usize,
    /// The color of the text
    pub color: [f32; 4],
    /// The resize strategy
    pub resize: Resize,
}

impl<S> TextFormat<S>
where
    S: Scalar,
{
    /// Create a default `TextFormat` with the given font size
    pub fn new(font_size: u32) -> TextFormat<S> {
        TextFormat {
            font_size,
            just: Justification::Left,
            line_spacing: S::ONE,
            first_line_indent: 0,
            lines_indent: 0,
            color: [0.0, 0.0, 0.0, 1.0],
            resize: Resize::NoLarger,
        }
    }
    /// Align the `TextFormat` to the left
    pub fn left(mut self) -> Self {
        self.just = Justification::Left;
        self
    }
    /// Center-align the `TextFormat`
    pub fn centered(mut self) -> Self {
        self.just = Justification::Centered;
        self
    }
    /// Align the `TextFormat` to the right
    pub fn right(mut self) -> Self {
        self.just = Justification::Right;
        self
    }
    /// Set the font size
    pub fn font_size(mut self, font_size: u32) -> Self {
        self.font_size = font_size;
        self
    }
    /// Set the line spacing
    pub fn line_spacing(mut self, line_spacing: S) -> Self {
        self.line_spacing = line_spacing;
        self
    }
    /// Changes the type of the line spacing and thus the `TextFormat` itself
    pub fn map_line_spacing<U>(&self) -> TextFormat<U>
    where
        U: Scalar + From<S>,
    {
        TextFormat {
            font_size: self.font_size,
            just: self.just,
            line_spacing: U::from(self.line_spacing),
            first_line_indent: self.first_line_indent,
            lines_indent: self.lines_indent,
            color: self.color,
            resize: self.resize,
        }
    }
    /// Set the indentation of the first line
    pub fn first_line_indent(mut self, first_line_indent: usize) -> Self {
        self.first_line_indent = first_line_indent;
        self
    }
    /// Set the indentation of all lines after the first
    pub fn lines_indent(mut self, lines_indent: usize) -> Self {
        self.lines_indent = lines_indent;
        self
    }
    /// Set the color
    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }
    /// Set the resize strategy
    pub fn resize(mut self, resize: Resize) -> Self {
        self.resize = resize;
        self
    }
    /// Change the font size depending on the the resize strategy
    ///
    /// The given max size is not used if the strategy is `Resize::None`
    pub fn resize_font(mut self, max_size: u32) -> Self {
        match self.resize {
            Resize::NoLarger => self.font_size = self.font_size.min(max_size),
            Resize::Max => self.font_size = max_size,
            Resize::None => (),
        }
        self
    }
}

/// Defines behavior of a cache of character widths.
///
/// In general, determining the width of a character glyphs with a given font size
/// is a non-trivial calculation. Caching a width calculation for each characters
/// and font size ensures that the calculation is only done once for each pair.
pub trait CharacterWidthCache {
    /// The scalar type for the width
    type Scalar: Scalar;
    /// Get the width of a character at a font size
    fn char_width(&mut self, character: char, font_size: u32) -> Self::Scalar;
    /// Get the width of a string at a font_size
    fn width(&mut self, text: &str, font_size: u32) -> Self::Scalar {
        text.chars()
            .map(|c| self.char_width(c, font_size))
            .fold(Self::Scalar::ZERO, std::ops::Add::add)
    }
    /// Split a string into a list of lines of text with the given format where no line
    /// is wider than the given max width. Newlines (`\n`) in the string are respected
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
    /// Get the width of the widest line after performing
    /// the calculation of `CharacterWidthCache::format_lines`
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
    /// Calculate a set of positioned lines of text with the given format
    /// that fit within the given rectangle
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
            })
            .collect()
    }
    /// Check if text with the given format fits within a rectangle's width
    fn text_fits_horizontal<R>(
        &mut self,
        text: &str,
        rect: R,
        format: TextFormat<Self::Scalar>,
    ) -> bool
    where
        R: Rectangle<Scalar = Self::Scalar>,
    {
        self.max_line_width(text, rect.width(), format) < rect.width()
    }
    /// Check if text with the given format fits within a rectangle's height
    fn text_fits_vertical<R>(
        &mut self,
        text: &str,
        rect: R,
        format: TextFormat<Self::Scalar>,
    ) -> bool
    where
        R: Rectangle<Scalar = Self::Scalar>,
    {
        let lines = self.format_lines(text, rect.width(), format);
        let last_line_y = rect.top()
            + format.font_size.into()
            + Self::Scalar::from((lines.len() - 1) as u32)
                * format.font_size.into()
                * format.line_spacing;
        last_line_y < rect.bottom()
    }
    /// Check if text with the given format fits within a rectangle
    fn text_fits<R>(&mut self, text: &str, rect: R, format: TextFormat<Self::Scalar>) -> bool
    where
        R: Rectangle<Scalar = Self::Scalar>,
    {
        self.text_fits_horizontal(text, rect.clone(), format)
            && self.text_fits_vertical(text, rect, format)
    }
    /// Determine the maximum font size for text with the given format
    /// that will still allow the text to fit within a rectangle
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
    /// Determine the minumum height for a rectangle such that text
    /// with the given format will still fit within the rectangle
    ///
    /// The given delta value defines how much to increment the
    /// rectangle's height on each check. Lower deltas will yield
    /// more accurate results, but will take longer to computer.
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
        while self.text_fits_vertical(text, rect.clone(), format) {
            rect = rect
                .clone()
                .with_size(R::Vector::new(rect.width(), rect.height() - delta))
        }
        while !self.text_fits_vertical(text, rect.clone(), format) {
            rect = rect
                .clone()
                .with_size(R::Vector::new(rect.width(), rect.height() + delta))
        }
        rect.height()
    }
    /// Determine the minumum width for a rectangle such that text
    /// with the given format will still fit within the rectangle
    ///
    /// The given delta value defines how much to increment the
    /// rectangle's width on each check. Lower deltas will yield
    /// more accurate results, but will take longer to computer.
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

/// A basic implememntor for `CharacterWidthCache`
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

/// Draw justified text to something using the `piston2d-graphics` crate
///
/// Text will be drawn in the given rectangle and use the given format
#[cfg(feature = "graphics")]
pub fn justified_text<U, R, T, C, G>(
    text: &str,
    rect: R,
    format: TextFormat<U>,
    glyphs: &mut C,
    transform: Matrix2d,
    graphics: &mut G,
) -> Result<(), C::Error>
where
    U: Scalar,
    f64: From<U>,
    R: Rectangle<Scalar = f64>,
    T: ImageSize,
    C: CharacterCache<Texture = T>,
    C::Error: fmt::Debug,
    G: Graphics<Texture = T>,
{
    for (pos, line) in glyphs.justify_text(text, rect, format.map_line_spacing::<f64>()) {
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
