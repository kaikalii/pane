use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub trait Pair {
    type Item;
    fn first(&self) -> Self::Item;
    fn second(&self) -> Self::Item;
    fn from_items(a: Self::Item, b: Self::Item) -> Self;
}

impl<T> Pair for (T, T)
where
    T: Clone,
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
    T: Clone,
{
    type Item = T;
    fn first(&self) -> Self::Item {
        self[0].clone()
    }
    fn second(&self) -> Self::Item {
        self[1].clone()
    }
    fn from_items(a: Self::Item, b: Self::Item) -> Self {
        [a, b]
    }
}

impl<T> Pair for (T, T, T, T)
where
    T: Clone,
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
    T: Clone,
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

pub trait ZeroOneTwo {
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
}

impl ZeroOneTwo for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
}

impl ZeroOneTwo for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
}

pub trait Scalar:
    Add<Self, Output = Self>
    + Copy
    + From<f32>
    + From<u32>
    + PartialEq
    + PartialOrd
    + Sum<Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + Sin<Output = Self>
    + Cos<Output = Self>
    + Pow<Self, Output = Self>
    + ZeroOneTwo
{
}

impl<T> Scalar for T where
    T: Copy
        + From<f32>
        + From<u32>
        + PartialEq
        + PartialOrd
        + Add<T, Output = T>
        + Sum<T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Neg<Output = T>
        + Sin<Output = T>
        + Cos<Output = T>
        + Pow<T, Output = T>
        + ZeroOneTwo
{}

pub trait Vector2: Sized {
    type Scalar: Scalar;
    fn x(&self) -> Self::Scalar;
    fn y(&self) -> Self::Scalar;
    fn new(x: Self::Scalar, y: Self::Scalar) -> Self;
    fn map<U, V>(&self) -> V
    where
        U: Scalar + From<Self::Scalar>,
        V: Vector2<Scalar = U>,
    {
        V::new(U::from(self.x()), U::from(self.y()))
    }
    fn neg(self) -> Self {
        Self::new(-self.x(), -self.y())
    }
    fn add<V: Vector2<Scalar = Self::Scalar>>(self, other: V) -> Self {
        Self::new(self.x() + other.x(), self.y() + other.y())
    }
    fn sub<V: Vector2<Scalar = Self::Scalar>>(self, other: V) -> Self {
        Self::new(self.x() - other.x(), self.y() - other.y())
    }
    fn mul(self, by: Self::Scalar) -> Self {
        Self::new(self.x() * by, self.y() * by)
    }
    fn mul2<V: Vector2<Scalar = Self::Scalar>>(self, other: V) -> Self {
        Self::new(self.x() * other.x(), self.y() * other.y())
    }
    fn div(self, by: Self::Scalar) -> Self {
        Self::new(self.x() / by, self.y() / by)
    }
    fn div2<V: Vector2<Scalar = Self::Scalar>>(self, other: V) -> Self {
        Self::new(self.x() / other.x(), self.y() / other.y())
    }
    fn dist<V: Vector2<Scalar = Self::Scalar>>(self, to: V) -> Self::Scalar {
        ((self.x() - to.x()).pow(Self::Scalar::TWO) + (self.y() - to.y()).pow(Self::Scalar::TWO))
            .pow(Self::Scalar::ONE / Self::Scalar::TWO)
    }
    fn mag(self) -> Self::Scalar {
        (self.x().pow(Self::Scalar::TWO) + self.y().pow(Self::Scalar::TWO))
            .pow(Self::Scalar::ONE / Self::Scalar::TWO)
    }
    fn rotate_about<V: Vector2<Scalar = Self::Scalar> + Clone>(
        self,
        pivot: V,
        angle: Self::Scalar,
    ) -> Self {
        let sin = (-angle).sin();
        let cos = (-angle).cos();
        let origin_point = self.sub(pivot.clone());
        let rotated_point = Self::new(
            origin_point.x() * cos - origin_point.y() * sin,
            origin_point.x() * sin + origin_point.y() * cos,
        );
        rotated_point.add(pivot)
    }
}

impl<P> Vector2 for P
where
    P: Pair,
    P::Item: Scalar,
{
    type Scalar = P::Item;
    fn x(&self) -> P::Item {
        self.first()
    }
    fn y(&self) -> P::Item {
        self.second()
    }
    fn new(x: P::Item, y: P::Item) -> Self {
        Self::from_items(x, y)
    }
}

pub trait Rectangle: Clone {
    type Scalar: Scalar;
    type Vector: Vector2<Scalar = Self::Scalar>;
    fn new(top_left: Self::Vector, size: Self::Vector) -> Self;
    fn top_left(&self) -> Self::Vector;
    fn top_right(&self) -> Self::Vector {
        Self::Vector::new(self.top_left().x() + self.size().x(), self.top_left().y())
    }
    fn bottom_left(&self) -> Self::Vector {
        Self::Vector::new(self.top_left().x(), self.top_left().y() + self.size().y())
    }
    fn bottom_right(&self) -> Self::Vector {
        self.top_left().add(self.size())
    }
    fn size(&self) -> Self::Vector;
    fn width(&self) -> Self::Scalar {
        self.size().x()
    }
    fn height(&self) -> Self::Scalar {
        self.size().y()
    }
    fn center(&self) -> Self::Vector {
        self.top_left().add(self.size().div(Self::Scalar::TWO))
    }
    fn with_top_left(self, top_left: Self::Vector) -> Self {
        Self::new(top_left, self.size())
    }
    fn with_size(self, size: Self::Vector) -> Self {
        Self::new(self.top_left(), size)
    }
}

impl<P> Rectangle for P
where
    P: Pair + Clone,
    P::Item: Vector2,
{
    type Scalar = <P::Item as Vector2>::Scalar;
    type Vector = P::Item;
    fn new(top_left: Self::Vector, size: Self::Vector) -> Self {
        Self::from_items(top_left, size)
    }
    fn top_left(&self) -> Self::Vector {
        self.first()
    }
    fn size(&self) -> Self::Vector {
        self.second()
    }
}
