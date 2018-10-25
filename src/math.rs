use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Pair {
    type Item: Copy;
    fn first(&self) -> Self::Item;
    fn second(&self) -> Self::Item;
    fn from_items(a: Self::Item, b: Self::Item) -> Self;
}

impl<T> Pair for (T, T)
where
    T: Copy,
{
    type Item = T;
    fn first(&self) -> Self::Item {
        self.0.clone()
    }
    fn second(&self) -> Self::Item {
        self.1.clone()
    }
    fn from_items(a: Self::Item, b: Self::Item) -> Self {
        (a, b)
    }
}

impl<T> Pair for [T; 2]
where
    T: Copy,
{
    type Item = T;
    fn first(&self) -> Self::Item {
        self[0].clone()
    }
    fn second(&self) -> Self::Item {
        self[0].clone()
    }
    fn from_items(a: Self::Item, b: Self::Item) -> Self {
        [a, b]
    }
}

impl<T> Pair for (T, T, T, T)
where
    T: Copy,
{
    type Item = (T, T);
    fn first(&self) -> Self::Item {
        (self.0.clone(), self.1.clone())
    }
    fn second(&self) -> Self::Item {
        (self.2.clone(), self.3.clone())
    }
    fn from_items(a: Self::Item, b: Self::Item) -> Self {
        (a.0, a.1, b.0, b.1)
    }
}

impl<T> Pair for [T; 4]
where
    T: Copy,
{
    type Item = [T; 2];
    fn first(&self) -> Self::Item {
        [self[0].clone(), self[1].clone()]
    }
    fn second(&self) -> Self::Item {
        [self[2].clone(), self[3].clone()]
    }
    fn from_items(a: Self::Item, b: Self::Item) -> Self {
        [a[0].clone(), a[1].clone(), b[0].clone(), b[1].clone()]
    }
}

pub trait Sin {
    type Output;
    fn sin(&self) -> Self::Output;
}

impl Sin for f32 {
    type Output = f32;
    fn sin(&self) -> Self::Output {
        f32::sin(*self)
    }
}

impl Sin for f64 {
    type Output = f64;
    fn sin(&self) -> Self::Output {
        f64::sin(*self)
    }
}

pub trait Cos {
    type Output;
    fn cos(&self) -> Self::Output;
}

impl Cos for f32 {
    type Output = f32;
    fn cos(&self) -> Self::Output {
        f32::cos(*self)
    }
}

impl Cos for f64 {
    type Output = f64;
    fn cos(&self) -> Self::Output {
        f64::cos(*self)
    }
}

pub trait Pow<P> {
    type Output;
    fn pow(&self, power: P) -> Self::Output;
}

impl Pow<Self> for f32 {
    type Output = f32;
    fn pow(&self, power: Self) -> Self::Output {
        self.powf(power)
    }
}

impl Pow<Self> for f64 {
    type Output = f64;
    fn pow(&self, power: Self) -> Self::Output {
        self.powf(power)
    }
}

pub trait OneTwo {
    const ONE: Self;
    const TWO: Self;
}

impl OneTwo for f32 {
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
}

impl OneTwo for f64 {
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
}

pub trait Scalar:
    Copy
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + Sin<Output = Self>
    + Cos<Output = Self>
    + Pow<Self, Output = Self>
    + OneTwo
{
}

impl<T> Scalar for T where
    T: Copy
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Neg<Output = T>
        + Sin<Output = T>
        + Cos<Output = T>
        + Pow<T, Output = T>
        + OneTwo
{}

pub trait Vector2<T>: Copy
where
    T: Scalar,
{
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn from_xy(x: T, y: T) -> Self;
    fn map<U, V>(&self) -> V
    where
        U: Scalar + From<T>,
        V: Vector2<U>,
    {
        V::from_xy(U::from(self.x()), U::from(self.y()))
    }
    fn neg(self) -> Self {
        Self::from_xy(-self.x(), -self.y())
    }
    fn add<V: Vector2<T>>(self, other: V) -> Self {
        Self::from_xy(self.x() + other.x(), self.y() + other.y())
    }
    fn sub<V: Vector2<T>>(self, other: V) -> Self {
        Self::from_xy(self.x() - other.x(), self.y() - other.y())
    }
    fn mul(self, by: T) -> Self {
        Self::from_xy(self.x() * by, self.y() * by)
    }
    fn mul2<V: Vector2<T>>(self, other: V) -> Self {
        Self::from_xy(self.x() * other.x(), self.y() * other.y())
    }
    fn div(self, by: T) -> Self {
        Self::from_xy(self.x() / by, self.y() / by)
    }
    fn div2<V: Vector2<T>>(self, other: V) -> Self {
        Self::from_xy(self.x() / other.x(), self.y() / other.y())
    }
    fn dist<V: Vector2<T>>(self, to: V) -> T {
        ((self.x() - to.x()).pow(T::TWO) + (self.y() - to.y()).pow(T::TWO)).pow(T::ONE / T::TWO)
    }
    fn mag(self) -> T {
        (self.x().pow(T::TWO) + self.y().pow(T::TWO)).pow(T::ONE / T::TWO)
    }
    fn rotate_about(self, pivot: Self, angle: T) -> Self {
        let sin = (-angle).sin();
        let cos = (-angle).cos();
        let origin_point = self.sub(pivot.clone());
        let rotated_point = Self::from_xy(
            origin_point.x() * cos - origin_point.y() * sin,
            origin_point.x() * sin + origin_point.y() * cos,
        );
        rotated_point.add(pivot)
    }
}

impl<P, T> Vector2<T> for P
where
    P: Pair<Item = T> + Copy,
    T: Scalar,
{
    fn x(&self) -> T {
        self.first()
    }
    fn y(&self) -> T {
        self.second()
    }
    fn from_xy(x: T, y: T) -> Self {
        Self::from_items(x, y)
    }
}

pub trait Rectangle<T, V>: Pair<Item = V> + Sized
where
    V: Pair<Item = T> + Copy,
    T: Scalar,
{
    fn new(top_left: V, size: V) -> Self {
        Self::from_items(top_left, size)
    }
    fn top_left(&self) -> V {
        self.first()
    }
    fn top_right(&self) -> V {
        V::from_xy(self.top_left().x() + self.size().x(), self.top_left().y())
    }
    fn bottom_left(&self) -> V {
        V::from_xy(self.top_left().x(), self.top_left().y() + self.size().y())
    }
    fn bottom_right(&self) -> V {
        self.top_left().add(self.size())
    }
    fn size(&self) -> V {
        self.second()
    }
    fn width(&self) -> T {
        self.size().x()
    }
    fn height(&self) -> T {
        self.size().y()
    }
    fn center(&self) -> V {
        self.top_left().add(self.size().div(T::TWO))
    }
    fn with_top_left(self, top_left: V) -> Self {
        Self::from_items(top_left, self.size())
    }
    fn with_size(self, size: V) -> Self {
        Self::from_items(self.top_left(), size)
    }
}

impl<T, V, R> Rectangle<T, V> for R
where
    R: Pair<Item = V> + Sized,
    V: Pair<Item = T> + Copy,
    T: Scalar,
{}
