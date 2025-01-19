use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Default, PartialEq, Hash, Eq)]
pub struct Point<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> Point<T> {
    pub fn norm(self) -> T {
        self * self
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> Add
    for Point<T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> Sub
    for Point<T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> Mul
    for Point<T>
{
    type Output = T;
    fn mul(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> Mul<T>
    for Point<T>
{
    type Output = Self;
    fn mul(self, scalar: T) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> Div<T>
    for Point<T>
{
    type Output = Self;
    fn div(self, scalar: T) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> AddAssign
    for Point<T>
{
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy> SubAssign
    for Point<T>
{
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_point() {
        let p1 = Point { x: 1.0, y: 1.0 };
        let p2 = Point { x: 2.0, y: 2.0 };
        let p3 = p1 + p2;
        assert_eq!(p3, Point { x: 3.0, y: 3.0 });
    }

    #[test]
    fn test_mul() {
        let p1 = Point { x: 1.0, y: 1.0 };
        let p2 = Point { x: 2.0, y: 2.0 };
        let p3 = p1 * p2;
        assert_eq!(p3, 4.0);
    }

    #[test]
    fn test_div() {
        let p1 = Point { x: 2.0, y: 6.0 };
        let p2 = p1 / 2.0;
        assert_eq!(p2, Point { x: 1.0, y: 3.0 });
    }

    #[test]
    fn test_sub() {
        let p1 = Point { x: 1.0, y: 1.0 };
        let p2 = Point { x: 2.0, y: 2.0 };
        let p3 = p1 - p2;
        assert_eq!(p3, Point { x: -1.0, y: -1.0 });
    }

    #[test]
    fn test_sub_len2() {
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = p1.norm();
        assert_eq!(p2, 1.0 + 4.0);
    }
}
