use super::math::point2d::Point2d;

#[derive(Debug, Copy, Clone, Default)]
pub struct Ball {
    pub pos: Point2d<f32>,
    pub vel: Point2d<f32>,
    pub mass: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32, mass: Option<f32>) -> Self {
        Self {
            pos: Point2d {x, y},
            vel: Point2d::default(),
            mass: mass.unwrap_or(1.0),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_ball() {
        let b1 = Ball::new(0.0, 0.0, None);
        let b2 = Ball { 
            pos: Point2d { x: 10.0, y: 3.0 },
            vel: Point2d { x: 10.0, y: 3.0 },
            mass: 1.0,
        };
    }
}
