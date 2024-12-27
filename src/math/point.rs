use std::ops::{Add, AddAssign, Mul, Div};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Point<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> Mul for Point<T> {
    type Output = T;
    fn mul(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> Mul<T> for Point<T> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> Div<T> for Point<T> {
    type Output = Self;
    fn div(self, scalar: T) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
