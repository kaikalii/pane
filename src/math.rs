//! Traits for doing vector geometry with built-in types

use std::ops::{Add, Div, Mul, Neg, Sub};

/// A trait for defining a pair of items of the same type.
///
/// This trait is meant to generalize having two similar things.
/// It is implemented for `(T, T)` and `[T; 2]` with `Item = T`.
/// However, because a pair does not necessarily have to be an
/// Actual *pair* It is also implemented for `(T, T, T, T)` and
/// `[T; 4]` with `Item = (T, T)` and `Item = [T; 2]` respectively.
pub trait Pair {
    /// The paired item
    type Item;
    /// Get the first thing
    fn first(&self) -> Self::Item;
    /// Get the second thing
    fn second(&self) -> Self::Item;
    /// Create a pair from two items
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

/// Trait for getting the sine of a number
pub trait Sin {
    /// The output type
    type Output;
    /// Get the sine
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

/// Trait for getting the cosine of a number
pub trait Cos {
    /// The output type
    type Output;
    /// Get the cosine
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

/// Trait for raising numbers to a power
pub trait Pow<P> {
    /// The output type
    type Output;
    /// Raise this number to a power
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

/// Trait for defining small-number constants
pub trait ZeroOneTwo {
    /// Zero `0`
    const ZERO: Self;
    /// One `1`
    const ONE: Self;
    /// Two `2`
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

/// Trait for math with scalar numbers
pub trait Scalar:
    Add<Self, Output = Self>
    + Copy
    + From<f32>
    + From<u32>
    + PartialEq
    + PartialOrd
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Neg<Output = Self>
    + Sin<Output = Self>
    + Cos<Output = Self>
    + Pow<Self, Output = Self>
    + ZeroOneTwo
{
    /// Get the absolute value
    fn abs(self) -> Self {
        if self >= Self::ZERO {
            self
        } else {
            self.neg()
        }
    }
    /// Get the max of this `Scalar` and another
    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }
    /// Get the min of this `Scalar` and another
    fn min(self, other: Self) -> Self {
        if self < other {
            self
        } else {
            other
        }
    }
}

impl<T> Scalar for T where
    T: Copy
        + From<f32>
        + From<u32>
        + PartialEq
        + PartialOrd
        + Add<T, Output = T>
        + Sub<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + Neg<Output = T>
        + Sin<Output = T>
        + Cos<Output = T>
        + Pow<T, Output = T>
        + ZeroOneTwo
{
}

/// Trait for manipulating 2D vectors
pub trait Vector2: Sized {
    /// The scalar type
    type Scalar: Scalar;
    /// Get the x component
    fn x(&self) -> Self::Scalar;
    /// Get the y component
    fn y(&self) -> Self::Scalar;
    /// Create a new vector from an x and y component
    fn new(x: Self::Scalar, y: Self::Scalar) -> Self;
    /// Map this vector to a vector of another type
    fn map<V>(&self) -> V
    where
        V: Vector2,
        V::Scalar: From<Self::Scalar>,
    {
        V::new(V::Scalar::from(self.x()), V::Scalar::from(self.y()))
    }
    /// Negate the vector
    fn neg(self) -> Self {
        Self::new(-self.x(), -self.y())
    }
    /// Add the vector to another
    fn add<V: Vector2<Scalar = Self::Scalar>>(self, other: V) -> Self {
        Self::new(self.x() + other.x(), self.y() + other.y())
    }
    /// Subtract another vector from this one
    fn sub<V: Vector2<Scalar = Self::Scalar>>(self, other: V) -> Self {
        Self::new(self.x() - other.x(), self.y() - other.y())
    }
    /// Multiply this vector by a scalar
    fn mul(self, by: Self::Scalar) -> Self {
        Self::new(self.x() * by, self.y() * by)
    }
    /// Multiply this vector component-wise by another
    fn mul2<V: Vector2<Scalar = Self::Scalar>>(self, other: V) -> Self {
        Self::new(self.x() * other.x(), self.y() * other.y())
    }
    /// Divide this vector by a scalar
    fn div(self, by: Self::Scalar) -> Self {
        Self::new(self.x() / by, self.y() / by)
    }
    /// Divide this vector component-wise by another
    fn div2<V: Vector2<Scalar = Self::Scalar>>(self, other: V) -> Self {
        Self::new(self.x() / other.x(), self.y() / other.y())
    }
    /// Get the distance between this vector and another
    fn dist<V: Vector2<Scalar = Self::Scalar>>(self, to: V) -> Self::Scalar {
        ((self.x() - to.x()).pow(Self::Scalar::TWO) + (self.y() - to.y()).pow(Self::Scalar::TWO))
            .pow(Self::Scalar::ONE / Self::Scalar::TWO)
    }
    /// Get the vector's magnitude
    fn mag(self) -> Self::Scalar {
        (self.x().pow(Self::Scalar::TWO) + self.y().pow(Self::Scalar::TWO))
            .pow(Self::Scalar::ONE / Self::Scalar::TWO)
    }
    /// Rotate the vector some number of radians about a pivot
    fn rotate_about<V: Vector2<Scalar = Self::Scalar> + Clone>(
        self,
        pivot: V,
        radians: Self::Scalar,
    ) -> Self {
        let sin = (-radians).sin();
        let cos = (-radians).cos();
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

/// A trait for manipulating rectangles
pub trait Rectangle: Clone {
    /// The scalar type
    type Scalar: Scalar;
    /// The vector type
    type Vector: Vector2<Scalar = Self::Scalar>;
    /// Create a new rectangle from a top-left corner position and a size
    fn new(top_left: Self::Vector, size: Self::Vector) -> Self;
    /// Get the top-left corner position
    fn top_left(&self) -> Self::Vector;
    /// Get the size
    fn size(&self) -> Self::Vector;
    /// Map this rectangle to a rectangle of another type
    fn map<R>(&self) -> R
    where
        R: Rectangle,
        R::Scalar: From<Self::Scalar>,
    {
        R::new(
            R::Vector::new(R::Scalar::from(self.left()), R::Scalar::from(self.top())),
            R::Vector::new(
                R::Scalar::from(self.width()),
                R::Scalar::from(self.height()),
            ),
        )
    }
    /// Get the top-right corner position
    fn top_right(&self) -> Self::Vector {
        Self::Vector::new(self.top_left().x() + self.size().x(), self.top_left().y())
    }
    /// Get the bottom-left corner position
    fn bottom_left(&self) -> Self::Vector {
        Self::Vector::new(self.top_left().x(), self.top_left().y() + self.size().y())
    }
    /// Get the bottom-right corner position
    fn bottom_right(&self) -> Self::Vector {
        self.top_left().add(self.size())
    }
    /// Get the top y
    fn top(&self) -> Self::Scalar {
        self.top_left().y()
    }
    /// Get the bottom y
    fn bottom(&self) -> Self::Scalar {
        self.top_left().y() + self.size().y()
    }
    /// Get the left x
    fn left(&self) -> Self::Scalar {
        self.top_left().x()
    }
    /// Get the right x
    fn right(&self) -> Self::Scalar {
        self.top_left().x() + self.size().x()
    }
    /// Get the width
    fn width(&self) -> Self::Scalar {
        self.size().x()
    }
    /// Get the height
    fn height(&self) -> Self::Scalar {
        self.size().y()
    }
    /// Get the position of the center
    fn center(&self) -> Self::Vector {
        self.top_left().add(self.size().div(Self::Scalar::TWO))
    }
    /// Transform the rectangle into one with a different top-left corner position
    fn with_top_left(self, top_left: Self::Vector) -> Self {
        Self::new(top_left, self.size())
    }
    /// Transform the rectangle into one with a different size
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
