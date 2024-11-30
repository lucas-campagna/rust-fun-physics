use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Point2d<T: Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T> + Copy> Add for Point2d<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Point2d<T> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

