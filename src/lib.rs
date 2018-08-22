extern crate graphics;

use std::{collections::HashMap, hash::Hash, mem, ops, rc::Rc};

use graphics::{
    character::CharacterCache, math::Matrix2d, rectangle, text, Graphics, ImageSize, Transformed,
};

fn add(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] + b[0], a[1] + b[1]]
}

fn sub(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rect {
    pub pos: [f64; 2],
    pub size: [f64; 2],
}

impl Rect {
    pub fn new(pos: [f64; 2], size: [f64; 2]) -> Rect {
        Rect { pos, size }
    }
    pub fn actualize(&self, rect: Rect) -> Rect {
        Rect {
            pos: add(
                rect.pos,
                [self.pos[0] * rect.size[0], self.pos[1] * rect.size[1]],
            ),
            size: [self.size[0] * rect.size[0], self.size[1] * rect.size[1]],
        }
    }
    pub fn contains(&self, point: [f64; 2]) -> bool {
        self.pos[0] <= point[0]
            && point[0] <= self.pos[0] + self.size[0]
            && self.pos[1] <= point[1]
            && point[1] <= self.pos[1] + self.size[1]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Orientation {
    Automatic,
    Horizantal,
    Vertical,
}

#[derive(Clone)]
pub enum WidgetKind<'a, I: 'a, S: 'a>
where
    I: Copy + Eq + Hash,
    S: Sized,
{
    SubWidget {
        subs: Vec<Widget<'a, I, S>>,
        subs_map: HashMap<I, usize>,
    },
    Label {
        string: String,
        max_text_size: u32,
        color: [f32; 4],
    },
    Button {
        string: Option<(String, [f32; 4])>,
        max_text_size: u32,
        callback: Rc<Box<FnMut(&mut Widget<'a, I, S>, &mut S)>>,
    },
    Slider {
        color: [f32; 4],
        callback: Rc<Box<FnMut(&'a mut Widget<'a, I, S>, &'a mut S) -> (f64, f64, &'a mut f64)>>,
    },
    EntryField {
        background_string: Option<(String, [f32; 4])>,
        max_text_size: u32,
        color: [f32; 4],
        callback: Rc<Box<FnMut(&'a mut Widget<'a, I, S>, &'a mut S) -> &'a mut String>>,
    },
}

static DEFAULT_MAX_TEXT_SIZE: u32 = 100;

impl<'a, I: 'a, S: 'a> WidgetKind<'a, I, S>
where
    I: Copy + Eq + Hash,
    S: Sized,
{
}

#[derive(Clone)]
pub struct Widget<'a, I: 'a, S: 'a>
where
    I: Copy + Eq + Hash,
    S: Sized,
{
    rect: Rect,
    kind: WidgetKind<'a, I, S>,
    orientation: Orientation,
    spacing: f64,
    background_color: [f32; 4],
}

impl<'a, I, S, R> From<R> for Widget<'a, I, S>
where
    I: Copy + Eq + Hash,
    S: Sized,
    R: IntoIterator<Item = Self>,
{
    fn from(regions: R) -> Self {
        Widget::new().with_sub_regions(regions)
    }
}

impl<'a, I, S> ops::Index<I> for Widget<'a, I, S>
where
    I: Copy + Eq + Hash,
    S: Sized,
{
    type Output = Widget<'a, I, S>;
    fn index(&self, i: I) -> &Self::Output {
        if let WidgetKind::SubWidget {
            ref subs,
            ref subs_map,
        } = self.kind
        {
            &subs[subs_map[&i]]
        } else {
            panic!("Can only index subwidget widgets")
        }
    }
}

impl<'a, I, S> ops::IndexMut<I> for Widget<'a, I, S>
where
    I: Copy + Eq + Hash,
    S: Sized,
{
    fn index_mut(&mut self, i: I) -> &mut Self::Output {
        if let WidgetKind::SubWidget {
            ref mut subs,
            ref subs_map,
        } = self.kind
        {
            &mut subs[subs_map[&i]]
        } else {
            panic!("Can only index subregion widgets")
        }
    }
}

impl<'a, I, S> Widget<'a, I, S>
where
    I: Copy + Eq + Hash,
    S: Sized,
{
    pub fn new() -> Self {
        Widget {
            rect: Rect {
                pos: [0.0, 0.0],
                size: [1.0, 1.0],
            },
            kind: WidgetKind::SubWidget {
                subs: Vec::new(),
                subs_map: HashMap::new(),
            },
            orientation: Orientation::Automatic,
            spacing: 10.0,
            background_color: [0.0, 0.0, 0.0, 0.0],
        }
    }
    pub fn label(string: &str) -> Widget<'a, I, S> {
        Widget::new().with_kind(WidgetKind::Label {
            string: string.to_string(),
            max_text_size: DEFAULT_MAX_TEXT_SIZE,
            color: [1.0, 1.0, 1.0, 1.0],
        })
    }
    pub fn button<F>(string: Option<&str>, callback: F) -> Widget<'a, I, S>
    where
        F: 'static + FnMut(&mut Widget<'a, I, S>, &mut S),
    {
        Widget::new().with_kind(WidgetKind::Button {
            string: string.map(|s| (s.to_string(), [1.0, 1.0, 1.0, 1.0])),
            max_text_size: DEFAULT_MAX_TEXT_SIZE,
            callback: Rc::new(Box::new(callback)),
        })
    }
    pub fn slider<F>(callback: F) -> Widget<'a, I, S>
    where
        F: 'static + FnMut(&'a mut Widget<'a, I, S>, &'a mut S) -> (f64, f64, &'a mut f64),
    {
        Widget::new().with_kind(WidgetKind::Slider {
            color: [1.0, 1.0, 1.0, 1.0],
            callback: Rc::new(Box::new(callback)),
        })
    }
    pub fn entry_field<F>(
        background_string: Option<(&str, [f32; 4])>,
        callback: F,
    ) -> Widget<'a, I, S>
    where
        F: 'static + FnMut(&'a mut Widget<'a, I, S>, &'a mut S) -> &'a mut String,
    {
        Widget::new().with_kind(WidgetKind::EntryField {
            background_string: background_string.map(|(s, c)| (s.to_string(), c)),
            max_text_size: DEFAULT_MAX_TEXT_SIZE,
            color: [1.0, 1.0, 1.0, 1.0],
            callback: Rc::new(Box::new(callback)),
        })
    }
    pub fn with_color(mut self, col: [f32; 4]) -> Self {
        use self::WidgetKind::*;
        match self.kind {
            Label { ref mut color, .. }
            | Slider { ref mut color, .. }
            | EntryField { ref mut color, .. } => *color = col,
            Button { ref mut string, .. } => if let Some((_, color)) = string {
                *color = col;
            },
            SubWidget { .. } => (),
        }
        self
    }
    pub fn with_max_text_size(mut self, max_size: u32) -> Self {
        use self::WidgetKind::*;
        match self.kind {
            Label {
                ref mut max_text_size,
                ..
            }
            | EntryField {
                ref mut max_text_size,
                ..
            }
            | Button {
                ref mut max_text_size,
                ..
            } => *max_text_size = max_size,
            SubWidget { .. } | Slider { .. } => (),
        }
        self
    }
    pub fn with_spacing(mut self, spacing: f64) -> Self {
        self.spacing = spacing;
        self
    }
    pub fn with_background_color(mut self, color: [f32; 4]) -> Self {
        self.background_color = color;
        self
    }
    pub fn with_orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }
    pub fn with_sub_regions<R: IntoIterator<Item = Self>>(
        mut self,
        regions: R,
    ) -> Widget<'a, I, S> {
        self.kind = WidgetKind::SubWidget {
            subs: regions.into_iter().collect(),
            subs_map: HashMap::new(),
        };
        self
    }
    pub fn with_kind(mut self, kind: WidgetKind<'a, I, S>) -> Self {
        self.kind = kind;
        self
    }
    pub fn with_from_to(mut self, top_left: [f64; 2], bottom_right: [f64; 2]) -> Self {
        self.rect.pos = top_left;
        self.rect.size = sub(bottom_right, self.rect.pos);
        self
    }
    pub fn insert_region(&mut self, region: Self) {
        if let WidgetKind::SubWidget { ref mut subs, .. } = self.kind {
            subs.push(region);
        } else {
            panic!("Cannot insert region into sub region widget");
        }
    }
    pub fn insert_indexed(&mut self, index: I, region: Self) -> Option<Self> {
        if let WidgetKind::SubWidget {
            ref mut subs,
            ref mut subs_map,
        } = self.kind
        {
            if let Some(replace_index) = subs_map.get(&index).cloned() {
                Some(mem::replace(&mut subs[replace_index], region))
            } else {
                subs_map.insert(index, subs.len());
                subs.push(region);
                None
            }
        } else {
            panic!("Cannot insert region into sub region widget");
        }
    }
    pub fn draw<T, G, C>(
        &self,
        window_size: [f64; 2],
        transform: Matrix2d,
        graphics: &mut G,
        glyphs: &mut C,
    ) where
        T: ImageSize,
        G: Graphics<Texture = T>,
        C: CharacterCache<Texture = T>,
    {
        self._draw([0.0, 0.0], window_size, transform, graphics, glyphs);
    }
    fn _draw<T, G, C>(
        &self,
        window_pos: [f64; 2],
        window_size: [f64; 2],
        transform: Matrix2d,
        graphics: &mut G,
        glyphs: &mut C,
    ) where
        T: ImageSize,
        G: Graphics<Texture = T>,
        C: CharacterCache<Texture = T>,
    {
        let act = self.rect.actualize(Rect {
            pos: window_pos,
            size: window_size,
        });
        rectangle(
            self.background_color,
            [act.pos[0], act.pos[1], act.size[0], act.size[1]],
            transform,
            graphics,
        );
        let ori = match self.orientation {
            Orientation::Automatic => if act.size[0] > act.size[1] {
                Orientation::Horizantal
            } else {
                Orientation::Vertical
            },
            _ => self.orientation,
        };
        use self::WidgetKind::*;
        match self.kind {
            SubWidget { ref subs, .. } => {
                let len = subs.len() as f64;
                let sub_win_size = if let Orientation::Vertical = ori {
                    [
                        act.size[0] - 2.0 * self.spacing,
                        (act.size[1] - self.spacing) / len - self.spacing,
                    ]
                } else {
                    [
                        (act.size[0] - self.spacing) / len - self.spacing,
                        act.size[1] - 2.0 * self.spacing,
                    ]
                };
                for (i, sub) in subs.iter().enumerate() {
                    sub._draw(
                        if let Orientation::Vertical = ori {
                            [
                                act.pos[0] + self.spacing,
                                act.pos[1]
                                    + self.spacing
                                    + i as f64 * (sub_win_size[1] + self.spacing),
                            ]
                        } else {
                            [
                                act.pos[0]
                                    + self.spacing
                                    + i as f64 * (sub_win_size[0] + self.spacing),
                                act.pos[1] + self.spacing,
                            ]
                        },
                        sub_win_size,
                        transform,
                        graphics,
                        glyphs,
                    );
                }
            }
            Label {
                ref string,
                max_text_size,
                color,
            } => {
                let mut size = (act.size[1] * 0.9).min(max_text_size as f64);
                let width = glyphs
                    .width(size as u32, string)
                    .unwrap_or_else(|_| panic!("Unable to cache text: {:?}", string));
                let horiz_offset = (act.size[0] - width).max(0.0) / 2.0;
                if width > act.size[0] {
                    size = size * act.size[0] / width;
                }
                text(
                    color,
                    size.floor() as u32,
                    string,
                    glyphs,
                    transform.trans(
                        act.pos[0] + horiz_offset,
                        act.pos[1] + (act.size[1] / 2.0).max(size),
                    ),
                    graphics,
                ).unwrap_or_else(|_| panic!("Unable to draw text: {:?}", string));
            }
            _ => (),
        }
    }
}
