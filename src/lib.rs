#![deny(missing_docs)]

//! This crate provides a data structure for text alignment. Rectangular `Pane`s (which may have smaller, child `Pane`s)
//! can be defined, and the positions of characters of text within them can be calculated.
//!
//! The `graphics` feature, which is on by default, allow the direct rendering of a `Pane` with the `piston2d-graphics` crate.

#[cfg(feature = "graphics")]
extern crate graphics;
#[cfg(feature = "buffer")]
extern crate graphics_buffer;
extern crate rusttype;

pub mod math;
mod text;
/// A prelud containing commonly used items in `Pane`
pub mod prelude {
    pub use color;
    pub use math::{Rectangle, Scalar, Vector2};
    #[cfg(feature = "graphics")]
    pub use text::justified_text;
    pub use text::{Justification, TextFormat};
    pub use Contents;
    pub use Orientation;
    pub use Pane;
}

use std::{collections::HashMap, ops};

#[cfg(feature = "graphics")]
use graphics::{character::CharacterCache, math::Matrix2d, rectangle, Graphics, ImageSize};

use math::{Rectangle, Scalar, Vector2, ZeroOneTwo};

pub use text::*;

/// Possible content of a `Pane`
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Contents<S>
where
    S: Scalar,
{
    /// Text with some format
    Text(String, TextFormat<S>),
}

/// An orientation for splitting a `Pane`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Orientation {
    /// Split the pane horizantally
    Horizontal,
    /// Split the pane vertically
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Vertical
    }
}

impl Orientation {
    fn split_rect<R, W>(&self, margin: R::Scalar, rect: R, weights: W) -> Vec<R>
    where
        R: Rectangle,
        W: IntoIterator<Item = R::Scalar>,
    {
        let weights: Vec<R::Scalar> = weights.into_iter().collect();
        let sum: R::Scalar = weights
            .iter()
            .cloned()
            .fold(R::Scalar::ZERO, std::ops::Add::add);
        let margin_fraction: R::Scalar = margin / (weights.len() as u32).into();
        match self {
            Orientation::Horizontal => {
                let mut offset = rect.top_left().x();
                weights
                    .into_iter()
                    .map(|w| {
                        let top_left = R::Vector::new(offset, rect.top_left().y());
                        let size =
                            R::Vector::new(rect.width() * w / sum - margin_fraction, rect.height());
                        offset = offset + size.x() + margin;
                        R::new(top_left, size)
                    })
                    .collect()
            }
            Orientation::Vertical => {
                let mut offset = rect.top_left().y();
                weights
                    .into_iter()
                    .map(|w| {
                        let top_left = R::Vector::new(rect.top_left().x(), offset);
                        let size =
                            R::Vector::new(rect.width(), rect.height() * w / sum - margin_fraction);
                        offset = offset + size.y() + margin;
                        R::new(top_left, size)
                    })
                    .collect()
            }
        }
    }
}

/// A rectangle which automatically determines the positions and sizes
/// of things withing it
///
/// A `Pane` can have any number of child `Panes`, each of which has a size
/// constrained by their parent `Pane`. The size and position of each child
/// pane depends on its weight relative to its siblings as well as the split
/// `Orientation` of its parent. This allows panes to be resized while keeping
/// all their child panes consistently sized.
///
/// A pane can also have optional contents. Contents will be resized to fit
/// the `Pane`
#[derive(Debug, Clone)]
pub struct Pane<R = [f64; 4]>
where
    R: Rectangle,
{
    contents: Option<Contents<R::Scalar>>,
    orientation: Orientation,
    margin: R::Scalar,
    names: HashMap<String, usize>,
    rect: R,
    children: Vec<(R::Scalar, Pane<R>)>,
    color: Color,
}

impl<R> Pane<R>
where
    R: Rectangle,
{
    /// Create a new `Pane`
    pub fn new() -> Self {
        Pane {
            contents: None,
            orientation: Orientation::default(),
            margin: R::Scalar::ZERO,
            names: HashMap::new(),
            children: Vec::new(),
            rect: R::new(
                R::Vector::new(R::Scalar::ZERO, R::Scalar::ZERO),
                R::Vector::new(R::Scalar::ONE, R::Scalar::ONE),
            ),
            color: color::TRANSPARENT,
        }
    }
    /// Immutable iterate over the `Pane`'s children
    pub fn children(&self) -> impl DoubleEndedIterator<Item = &Pane<R>> {
        self.children.iter().map(|(_, pane)| pane)
    }
    /// Mutable iterate over the `Pane`'s children
    pub fn children_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Pane<R>> {
        self.children.iter_mut().map(|(_, pane)| pane)
    }
    /// Get the `Pane`'s contents
    pub fn contents(&self) -> Option<&Contents<R::Scalar>> {
        self.contents.as_ref()
    }
    /// Change the `Pane`'s contents
    pub fn with_contents(mut self, contents: Contents<R::Scalar>) -> Self {
        self.contents = Some(contents);
        self
    }
    /// Remove the `Pane`'s contents
    pub fn with_no_contents(mut self) -> Self {
        self.contents = None;
        self
    }
    /// Get the `Pane`'s rectangle
    pub fn rect(&self) -> R {
        self.rect.clone()
    }
    /// Set the `Pane`'s rectangle
    pub fn with_rect(mut self, rect: R) -> Self {
        self.rect = rect;
        self
    }
    /// Get the `Pane`'s size
    pub fn size(&self) -> R::Vector {
        self.rect.size()
    }
    /// Set the `Pane`'s size
    pub fn with_size<T, V>(mut self, size: V) -> Self
    where
        T: Scalar,
        R::Scalar: From<T>,
        V: Vector2<Scalar = T>,
    {
        self.rect = self.rect.with_size(size.map());
        self.update_rects();
        self
    }
    /// Get the position of the `Pane`'s top-left corner
    pub fn top_left(&self) -> R::Vector {
        self.rect.top_left()
    }
    /// Set the position of the `Pane`'s top-left corner
    pub fn with_top_left<T, V>(mut self, top_left: V) -> Self
    where
        T: Scalar,
        R::Scalar: From<T>,
        V: Vector2<Scalar = T>,
    {
        self.rect = self.rect.with_top_left(top_left.map());
        self.update_rects();
        self
    }
    /// Set the `Pane`'s inner `Pane`s. Each inner `Pane` has a
    /// weight which defines how it is resized relative to its
    /// siblings. `Pane`s can also have optional names that can be
    /// used to index their parent.
    pub fn with_panes<'a, P, I>(mut self, panes: I) -> Self
    where
        P: NamedWeightedPane<'a, R>,
        I: IntoIterator<Item = P>,
    {
        let mut new_names = HashMap::new();
        self.children = panes
            .into_iter()
            .map(NamedWeightedPane::named_weighted_pane)
            .enumerate()
            .map(|(i, (name, weight, pane))| {
                if let Some(name) = name {
                    new_names.insert(name.to_string(), i);
                }
                (weight, pane)
            })
            .collect();
        self.names = new_names;
        self.update_rects();
        self
    }
    /// Get the split orientation of the `Pane`'s children
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }
    /// Set the split orientation of the `Pane`'s children
    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self.update_rects();
        self
    }
    /// Get the `Pane`'s color
    pub fn color(&self) -> Color {
        self.color
    }
    /// Set the `Pane`'s color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    /// Get the `Pane`'s margin
    pub fn margin(&self) -> R::Scalar {
        self.margin
    }
    /// Set the `Pane`'s margin
    pub fn with_margin(mut self, margin: R::Scalar) -> Self {
        self.margin = margin;
        self.update_rects();
        self
    }
    /// Get the inner rectangle created by the `Pane` and its margin
    pub fn margin_rect(&self) -> R {
        R::new(
            self.rect
                .top_left()
                .add(R::Vector::new(self.margin, self.margin)),
            self.rect
                .size()
                .sub(R::Vector::new(self.margin, self.margin).mul(R::Scalar::TWO)),
        )
    }
    /// Update the size of all inner `Pane`s' rectangles
    fn update_rects(&mut self) {
        let margin_rect = self.margin_rect();
        let new_rects = self.orientation.split_rect(
            self.margin,
            margin_rect,
            self.children.iter().map(|(w, _)| *w),
        );
        for (pane, rect) in self.children.iter_mut().zip(new_rects) {
            pane.1.rect = rect;
            pane.1.update_rects();
        }
    }
    /// Recursively fit the text of any `Contents::Text` in the `Pane`'s tree
    pub fn fit_text<C>(mut self, glyphs: &mut C) -> Self
    where
        C: CharacterWidthCache<Scalar = R::Scalar>,
    {
        self.update_rects();
        let margin_rect = self.margin_rect();
        if let Some(Contents::Text(ref text, ref mut format)) = self.contents {
            *format = format.resize_font(glyphs.fit_max_font_size(text, margin_rect, *format));
        }
        self.children = self
            .children
            .into_iter()
            .map(|(w, pane)| (w, pane.fit_text(glyphs)))
            .collect();
        self
    }
}

impl<R> Pane<R>
where
    R: Rectangle,
    f64: From<R::Scalar>,
{
    /// Draw the `Pane` and all its contents to something using
    /// the `piston2d-graphics` crate
    #[cfg(feature = "graphics")]
    pub fn draw<T, C, G>(
        &self,
        glyphs: &mut C,
        transform: Matrix2d,
        graphics: &mut G,
    ) -> Result<(), C::Error>
    where
        T: ImageSize,
        C: CharacterCache<Texture = T>,
        G: Graphics<Texture = T>,
    {
        rectangle(
            self.color,
            self.rect().map::<[f64; 4]>(),
            transform,
            graphics,
        );
        if let Some(ref contents) = self.contents {
            match contents {
                Contents::Text(text, format) => justified_text(
                    text,
                    self.margin_rect().map::<[f64; 4]>(),
                    *format,
                    glyphs,
                    transform,
                    graphics,
                )?,
            }
        }
        for (_, pane) in &self.children {
            pane.draw(glyphs, transform, graphics)?;
        }
        Ok(())
    }
}

impl<R> ops::Index<usize> for Pane<R>
where
    R: Rectangle,
{
    type Output = Pane<R>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.children[index].1
    }
}

impl<'a, R> ops::Index<&'a str> for Pane<R>
where
    R: Rectangle,
{
    type Output = Pane<R>;
    fn index(&self, index: &'a str) -> &Self::Output {
        let index = self.names[index];
        &self[index]
    }
}

trait Map<I> {
    type Accessed;
    fn map<F>(self, index: I, f: F) -> Self
    where
        F: Fn(Self::Accessed) -> Self::Accessed;
}

impl<R> Map<usize> for Pane<R>
where
    R: Rectangle,
{
    type Accessed = Pane<R>;
    fn map<F>(mut self, index: usize, f: F) -> Self
    where
        F: Fn(Self::Accessed) -> Self::Accessed,
    {
        self.children = self
            .children
            .into_iter()
            .enumerate()
            .map(|(i, (w, pane))| (w, if i == index { f(pane) } else { pane }))
            .collect();
        self
    }
}

impl<'a, R> Map<&'a str> for Pane<R>
where
    R: Rectangle,
{
    type Accessed = Pane<R>;
    fn map<F>(self, index: &'a str, f: F) -> Self
    where
        F: Fn(Self::Accessed) -> Self::Accessed,
    {
        let index = self.names[index];
        self.map(index, f)
    }
}

/// Defines conversion into a child `Pane` with a weight and optional name
pub trait NamedWeightedPane<'a, R>
where
    R: Rectangle,
{
    /// Converts into a child `Pane` with a weight and optional name
    fn named_weighted_pane(self) -> (Option<&'a str>, R::Scalar, Pane<R>);
}

impl<'a, R> NamedWeightedPane<'a, R> for Pane<R>
where
    R: Rectangle,
{
    fn named_weighted_pane(self) -> (Option<&'a str>, R::Scalar, Pane<R>) {
        (None, R::Scalar::ONE, self)
    }
}

impl<'a, R> NamedWeightedPane<'a, R> for (R::Scalar, Pane<R>)
where
    R: Rectangle,
{
    fn named_weighted_pane(self) -> (Option<&'a str>, R::Scalar, Pane<R>) {
        (None, self.0, self.1)
    }
}

impl<'a, R> NamedWeightedPane<'a, R> for (Option<&'a str>, R::Scalar, Pane<R>)
where
    R: Rectangle,
{
    fn named_weighted_pane(self) -> (Option<&'a str>, R::Scalar, Pane<R>) {
        self
    }
}

impl<'a, R> NamedWeightedPane<'a, R> for (&'a str, R::Scalar, Pane<R>)
where
    R: Rectangle,
{
    fn named_weighted_pane(self) -> (Option<&'a str>, R::Scalar, Pane<R>) {
        (Some(self.0), self.1, self.2)
    }
}

impl<'a, R> NamedWeightedPane<'a, R> for &'a str
where
    R: Rectangle,
{
    fn named_weighted_pane(self) -> (Option<&'a str>, R::Scalar, Pane<R>) {
        (Some(self), R::Scalar::ONE, Pane::new())
    }
}

/// A color
pub type Color = [f32; 4];

/// Defines serveral color constants
pub mod color {
    use Color;
    /// Red
    pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
    /// Orange
    pub const ORANGE: Color = [1.0, 0.5, 0.0, 1.0];
    /// Yellow
    pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
    /// Green
    pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
    /// Cyan
    pub const CYAN: Color = [0.0, 1.0, 1.0, 1.0];
    /// Blue
    pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
    /// Purple
    pub const PURPLE: Color = [0.5, 0.0, 0.5, 1.0];
    /// Magenta
    pub const MAGENTA: Color = [1.0, 0.0, 1.0, 1.0];
    /// Black
    pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
    /// Gray (same as `GREY`)
    pub const GRAY: Color = [0.5, 0.5, 0.5, 1.0];
    /// Grey (same as `GRAY`)
    pub const GREY: Color = [0.5, 0.5, 0.5, 1.0];
    /// White
    pub const WHITE: Color = [1.0; 4];
    /// Transparent
    pub const TRANSPARENT: Color = [0.0; 4];
}
