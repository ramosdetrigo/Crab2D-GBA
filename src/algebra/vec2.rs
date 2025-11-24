#![allow(dead_code)]
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A unified numeric trait so we don't write the same gigantic bounds everywhere.
pub trait Number:
    Copy
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
{
}
impl<T> Number for T where
    T: Copy
        + Add<Output = T>
        + AddAssign
        + Sub<Output = T>
        + SubAssign
        + Mul<Output = T>
        + MulAssign
        + Div<Output = T>
        + DivAssign
{}

/// 2D vector with generic numeric components.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2<T: Number> {
    pub x: T,
    pub y: T,
}

impl<T: Number> Vec2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    pub const fn splat(n: T) -> Self {
        Self { x: n, y: n }
    }

    #[inline]
    pub fn dot(&self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline]
    pub fn cross(self, rhs: Self) -> T {
        self.x * rhs.y - rhs.x * rhs.y
    }
}

impl<T: Number> Add for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Number> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Number> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Number> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Number> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Number> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Number> Div<T> for Vec2<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: Number> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
