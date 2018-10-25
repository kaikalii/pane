extern crate rusttype;

mod math;
mod prelude {
    pub use math::{Rectangle, Vector2};
    pub use Orientation;
    pub use Pane;
}

use std::{collections::HashMap, ops};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Orientation {
    Automatic,
    Horizontal,
    Vertical,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation::Automatic
    }
}

#[derive(Debug, Clone)]
pub struct Pane {
    orientation: Orientation,
    names: HashMap<String, usize>,
    inner_panes: Vec<(f64, Pane)>,
}

impl Pane {
    pub fn new() -> Self {
        Pane {
            orientation: Orientation::default(),
            names: HashMap::new(),
            inner_panes: Vec::new(),
        }
    }
    pub fn with_panes<P>(mut self, weights_and_panes: P) -> Self
    where
        P: IntoIterator<Item = (f64, Pane)>,
    {
        self.inner_panes = weights_and_panes.into_iter().collect();
        self
    }
    pub fn with_named_panes<'a, P>(mut self, weights: P) -> Self
    where
        P: IntoIterator<Item = (&'a str, f64, Pane)>,
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
        self
    }
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }
    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }
    pub fn split_in_half(mut self) -> Self {
        self.inner_panes = vec![(1.0, Pane::new()); 2];
        self.names.clear();
        self
    }
    pub fn split_in_half_named(mut self, name1: &str, name2: &str) -> Self {
        self.inner_panes = vec![(1.0, Pane::new()); 2];
        self.names.clear();
        self.names.insert(name1.to_string(), 0);
        self.names.insert(name2.to_string(), 1);
        self
    }
    pub fn split_in_three(mut self) -> Self {
        self.inner_panes = vec![(1.0, Pane::new()); 3];
        self.names.clear();
        self
    }
    pub fn split_in_three_named(mut self, name1: &str, name2: &str, name3: &str) -> Self {
        self.inner_panes = vec![(1.0, Pane::new()); 3];
        self.names.clear();
        self.names.insert(name1.to_string(), 0);
        self.names.insert(name2.to_string(), 1);
        self.names.insert(name3.to_string(), 2);
        self
    }
    pub fn split_weighted<W>(mut self, weights: W) -> Self
    where
        W: IntoIterator<Item = f64>,
    {
        self.inner_panes = weights.into_iter().map(|w| (w, Pane::new())).collect();
        self.names.clear();
        self
    }
    pub fn split_weighted_named<'a, W>(mut self, weights: W) -> Self
    where
        W: IntoIterator<Item = (&'a str, f64)>,
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
        self
    }
}

impl ops::Index<usize> for Pane {
    type Output = Pane;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner_panes[index].1
    }
}

impl<'a> ops::Index<&'a str> for Pane {
    type Output = Pane;
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

impl Map<usize> for Pane {
    type Accessed = Pane;
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

impl<'a> Map<&'a str> for Pane {
    type Accessed = Pane;
    fn map<F>(self, index: &'a str, f: F) -> Self
    where
        F: Fn(Self::Accessed) -> Self::Accessed,
    {
        let index = self.names[index];
        self.map(index, f)
    }
}
