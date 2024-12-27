use super::math::point::Point;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Ball {
    pub pos: Point<f32>,
    pub vel: Point<f32>,
    pub mass: f32,
    pub radius: f32,
    pub elasticity: f32
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Point {x, y},
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_ball() {
        let _b1 = Ball::new(0.0, 0.0);
        let _b2 = Ball { 
            pos: Point { x: 10.0, y: 3.0 },
            vel: Point { x: 10.0, y: 3.0 },
            ..Default::default()
        };
        assert!(_b1.pos == _b2.pos);
    }
}
