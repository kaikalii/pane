use std::collections::HashMap;
#[cfg(feature = "graphics")]
use std::fmt;

use rusttype::{Error, Font, GlyphId, Scale};

use math::Scalar;

pub trait CharacterWidthCache {
    type Scalar: Scalar;
    fn width(&mut self, character_size: u32, character: char) -> Self::Scalar;
}

#[derive(Clone)]
pub struct Glyphs<'f, S>
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
    fn width(&mut self, font_size: u32, character: char) -> Self::Scalar {
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
    fn width(&mut self, font_size: u32, character: char) -> Self::Scalar {
        <Self as graphics::character::CharacterCache>::character(self, font_size, character)
            .unwrap_or_else(|e| panic!("{:?}", e))
            .size[0]
    }
}
