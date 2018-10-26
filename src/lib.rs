#[cfg(feature = "graphics")]
extern crate graphics;
#[cfg(feature = "buffer")]
extern crate graphics_buffer;
extern crate rusttype;

pub mod math;
mod text;
pub mod prelude {
    pub use math::{Rectangle, Scalar, Vector2};
    #[cfg(feature = "graphics")]
    pub use text::justified_text;
    pub use text::{CharacterWidthCache, Glyphs, Justification, TextFormat};
    pub use Contents;
    pub use Orientation;
    pub use Pane;
}

use std::{collections::HashMap, ops};

pub use math::{Rectangle, Scalar, Vector2, ZeroOneTwo};
pub use text::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Contents<S>
where
    S: Scalar,
{
    Text(String, TextFormat<S>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Vertical
    }
}

impl Orientation {
    fn split_rect<R, W>(&self, rect: R, weights: W) -> Vec<R>
    where
        R: Rectangle,
        W: IntoIterator<Item = R::Scalar>,
    {
        let weights: Vec<R::Scalar> = weights.into_iter().collect();
        let sum: R::Scalar = weights
            .iter()
            .cloned()
            .fold(R::Scalar::ZERO, std::ops::Add::add);
        match self {
            Orientation::Horizontal => {
                let mut offset = rect.top_left().x();
                weights
                    .into_iter()
                    .map(|w| {
                        let top_left = R::Vector::new(offset, rect.top_left().y());
                        let size = R::Vector::new(rect.width() * w / sum, rect.height());
                        offset = offset + size.x();
                        R::new(top_left, size)
                    }).collect()
            }
            Orientation::Vertical => {
                let mut offset = rect.top_left().y();
                weights
                    .into_iter()
                    .map(|w| {
                        let top_left = R::Vector::new(rect.top_left().x(), offset);
                        let size = R::Vector::new(rect.width(), rect.height() * w / sum);
                        offset = offset + size.y();
                        R::new(top_left, size)
                    }).collect()
            }
        }
    }
}

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
    inner_panes: Vec<(R::Scalar, Pane<R>)>,
}

impl<R> Pane<R>
where
    R: Rectangle,
{
    pub fn new() -> Self {
        Pane {
            contents: None,
            orientation: Orientation::default(),
            margin: R::Scalar::ZERO,
            names: HashMap::new(),
            inner_panes: Vec::new(),
            rect: R::new(
                R::Vector::new(R::Scalar::ZERO, R::Scalar::ZERO),
                R::Vector::new(R::Scalar::ONE, R::Scalar::ONE),
            ),
        }
    }
    pub fn contents(&self) -> Option<&Contents<R::Scalar>> {
        self.contents.as_ref()
    }
    pub fn with_contents(mut self, contents: Contents<R::Scalar>) -> Self {
        self.contents = Some(contents);
        self
    }
    pub fn with_no_contents(mut self) -> Self {
        self.contents = None;
        self
    }
    pub fn rect(&self) -> R {
        self.rect.clone()
    }
    pub fn with_rect(mut self, rect: R) -> Self {
        self.rect = rect;
        self
    }
    pub fn size(&self) -> R::Vector {
        self.rect.size()
    }
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
    pub fn top_left(&self) -> R::Vector {
        self.rect.top_left()
    }
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
    pub fn with_panes<P>(mut self, weights_and_panes: P) -> Self
    where
        P: IntoIterator<Item = (R::Scalar, Pane<R>)>,
    {
        self.inner_panes = weights_and_panes.into_iter().collect();
        self.update_rects();
        self
    }
    pub fn with_named_panes<'a, P>(mut self, weights: P) -> Self
    where
        P: IntoIterator<Item = (&'a str, R::Scalar, Pane<R>)>,
    {
        let mut new_names = HashMap::new();
        self.inner_panes = weights
            .into_iter()
            .enumerate()
            .map(|(i, (name, weight, pane))| {
                new_names.insert(name.to_string(), i);
                (weight, pane)
            }).collect();
        self.names = new_names;
        self.update_rects();
        self
    }
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }
    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self.update_rects();
        self
    }
    pub fn margin(&self) -> R::Scalar {
        self.margin
    }
    pub fn with_margin(mut self, margin: R::Scalar) -> Self {
        self.margin = margin;
        self.update_rects();
        self
    }
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
    fn update_rects(&mut self) {
        let margin_rect = self.margin_rect();
        let new_rects = self
            .orientation
            .split_rect(margin_rect, self.inner_panes.iter().map(|(w, _)| *w));
        for (pane, rect) in self.inner_panes.iter_mut().zip(new_rects) {
            pane.1.rect = rect;
            pane.1.update_rects();
        }
    }
    pub fn fit_text<C>(mut self, glyphs: &mut C) -> Self
    where
        C: CharacterWidthCache<Scalar = R::Scalar>,
    {
        let margin_rect = self.margin_rect();
        if let Some(Contents::Text(ref text, ref mut format)) = self.contents {
            format.font_size = glyphs.fit_max_font_size(text, margin_rect, *format);
        }
        self.inner_panes = self
            .inner_panes
            .into_iter()
            .map(|(w, pane)| (w, pane.fit_text(glyphs)))
            .collect();
        self
    }
    pub fn split_in_half(mut self) -> Self {
        self.inner_panes = vec![(R::Scalar::ONE, Pane::new()); 2];
        self.names.clear();
        self.update_rects();
        self
    }
    pub fn split_in_half_named(mut self, name1: &str, name2: &str) -> Self {
        self.inner_panes = vec![(R::Scalar::ONE, Pane::new()); 2];
        self.names.clear();
        self.names.insert(name1.to_string(), 0);
        self.names.insert(name2.to_string(), 1);
        self.update_rects();
        self
    }
    pub fn split_in_three(mut self) -> Self {
        self.inner_panes = vec![(R::Scalar::ONE, Pane::new()); 3];
        self.names.clear();
        self.update_rects();
        self
    }
    pub fn split_in_three_named(mut self, name1: &str, name2: &str, name3: &str) -> Self {
        self.inner_panes = vec![(R::Scalar::ONE, Pane::new()); 3];
        self.names.clear();
        self.names.insert(name1.to_string(), 0);
        self.names.insert(name2.to_string(), 1);
        self.names.insert(name3.to_string(), 2);
        self.update_rects();
        self
    }
    pub fn split_weighted<W>(mut self, weights: W) -> Self
    where
        W: IntoIterator<Item = R::Scalar>,
    {
        self.inner_panes = weights.into_iter().map(|w| (w, Pane::new())).collect();
        self.names.clear();
        self.update_rects();
        self
    }
    pub fn split_weighted_named<'a, W>(mut self, weights: W) -> Self
    where
        W: IntoIterator<Item = (&'a str, R::Scalar)>,
    {
        let mut new_names = HashMap::new();
        self.inner_panes = weights
            .into_iter()
            .enumerate()
            .map(|(i, (name, weight))| {
                new_names.insert(name.to_string(), i);
                (weight, Pane::new())
            }).collect();
        self.names = new_names;
        self.update_rects();
        self
    }
}

impl<R> ops::Index<usize> for Pane<R>
where
    R: Rectangle,
{
    type Output = Pane<R>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner_panes[index].1
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

pub trait Map<I> {
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
        self.inner_panes = self
            .inner_panes
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
