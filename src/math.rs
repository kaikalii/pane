use std::ops::{Add, Div, Mul, Neg, Sub};

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

impl OneTwo for f64 {
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
}

pub trait Vector2<T>: Clone + Sized
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Div<T, Output = T>,
    T: Neg<Output = T>,
    T: Sin<Output = T>,
    T: Cos<Output = T>,
    T: Pow<T, Output = T>,
    T: OneTwo,
{
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn from_xy(x: T, y: T) -> Self;
    fn neg(self) -> Self {
        Self::from_xy(-self.x(), -self.y())
    }
    fn add(self, other: Self) -> Self {
        Self::from_xy(self.x() + other.x(), self.y() + other.y())
    }
    fn sub(self, other: Self) -> Self {
        Self::from_xy(self.x() - other.x(), self.y() - other.y())
    }
    fn mul(self, by: T) -> Self {
        Self::from_xy(self.x() * by, self.y() * by)
    }
    fn mul2(self, other: Self) -> Self {
        Self::from_xy(self.x() * other.x(), self.y() * other.y())
    }
    fn div(self, by: T) -> Self {
        Self::from_xy(self.x() / by, self.y() / by)
    }
    fn div2(self, other: Self) -> Self {
        Self::from_xy(self.x() / other.x(), self.y() / other.y())
    }
    fn dist(self, to: Self) -> T {
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

impl<T> Vector2<T> for [T; 2]
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Div<T, Output = T>,
    T: Neg<Output = T>,
    T: Sin<Output = T>,
    T: Cos<Output = T>,
    T: Pow<T, Output = T>,
    T: OneTwo,
{
    fn x(&self) -> T {
        self[0]
    }
    fn y(&self) -> T {
        self[1]
    }
    fn from_xy(x: T, y: T) -> Self {
        [x, y]
    }
}

impl<T> Vector2<T> for (T, T)
where
    T: Copy,
    T: Add<T, Output = T>,
    T: Sub<T, Output = T>,
    T: Mul<T, Output = T>,
    T: Div<T, Output = T>,
    T: Neg<Output = T>,
    T: Sin<Output = T>,
    T: Cos<Output = T>,
    T: Pow<T, Output = T>,
    T: OneTwo,
{
    fn x(&self) -> T {
        self.0
    }
    fn y(&self) -> T {
        self.1
    }
    fn from_xy(x: T, y: T) -> Self {
        (x, y)
    }
}
