extern crate graphics;

use graphics::{math::Matrix2d, rectangle, Graphics};

fn add(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] + b[0], a[1] + b[1]]
}

fn sub(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

fn mul2(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] * b[0], a[1] * b[1]]
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Region {
    rect: Rect,
    subs: Vec<Region>,
    orientation: Orientation,
    spacing: f64,
    color: [f32; 4],
}

impl From<[f64; 2]> for Region {
    fn from(pos: [f64; 2]) -> Self {
        Region {
            rect: Rect {
                pos,
                size: sub([1.0, 1.0], pos),
            },
            subs: Vec::new(),
            orientation: Orientation::Automatic,
            spacing: 10.0,
            color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

impl Region {
    pub fn full() -> Region {
        Region {
            rect: Rect {
                pos: [0.0, 0.0],
                size: [1.0, 1.0],
            },
            subs: Vec::new(),
            orientation: Orientation::Automatic,
            spacing: 10.0,
            color: [0.0, 0.0, 0.0, 0.0],
        }
    }
    pub fn to(mut self, bottom_right: [f64; 2]) -> Region {
        self.rect.size = sub(bottom_right, self.rect.pos);
        self
    }
    pub fn with_spacing(mut self, spacing: f64) -> Region {
        self.spacing = spacing;
        self
    }
    pub fn with_color(mut self, color: [f32; 4]) -> Region {
        self.color = color;
        self
    }
    pub fn with_orientation(mut self, orientation: Orientation) -> Region {
        self.orientation = orientation;
        self
    }
    pub fn sub<I: IntoIterator<Item = Region>>(mut self, regions: I) -> Region {
        self.subs = regions.into_iter().collect();
        self
    }
    pub fn draw<G: Graphics>(&self, window_size: [f64; 2], transform: Matrix2d, graphics: &mut G) {
        self._draw([0.0, 0.0], window_size, transform, graphics);
    }
    fn _draw<G: Graphics>(
        &self,
        window_pos: [f64; 2],
        window_size: [f64; 2],
        transform: Matrix2d,
        graphics: &mut G,
    ) {
        let act = self.rect.actualize(Rect {
            pos: window_pos,
            size: window_size,
        });
        rectangle(
            self.color,
            [act.pos[0], act.pos[1], act.size[0], act.size[1]],
            transform,
            graphics,
        );
        let ori = match self.orientation {
            Orientation::Automatic => if window_size[0] > window_size[1] {
                Orientation::Horizantal
            } else {
                Orientation::Vertical
            },
            _ => self.orientation,
        };
        let len = self.subs.len() as f64;
        let sub_win_size = if let Orientation::Vertical = ori {
            [
                window_size[0] - 2.0 * self.spacing,
                (window_size[1] - self.spacing) / len - self.spacing,
            ]
        } else {
            [
                (window_size[0] - self.spacing) / len - self.spacing,
                window_size[1] - 2.0 * self.spacing,
            ]
        };
        for (i, sub) in self.subs.iter().enumerate() {
            sub._draw(
                if let Orientation::Vertical = ori {
                    [
                        window_pos[0] + self.spacing,
                        self.spacing + i as f64 * (sub_win_size[1] + self.spacing),
                    ]
                } else {
                    [
                        self.spacing + i as f64 * (sub_win_size[0] + self.spacing),
                        window_pos[1] + self.spacing,
                    ]
                },
                sub_win_size,
                transform,
                graphics,
            );
        }
    }
}
