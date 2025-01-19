use super::math::point::Point;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Ball {
    pub pos: Point<f64>,
    pub vel: Point<f64>,
    pub mass: f64,
    pub radius: f64,
    pub elasticity: f64,
}

impl Ball {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Point { x, y },
            vel: Point { x: 0.0, y: 0.0 },
            radius: 1.0,
            elasticity: 1.0,
            mass: 1.0,
        }
    }
    pub fn check_collision(&self, other: &Ball) -> bool {
        (self.pos - other.pos).norm() < (self.radius + other.radius).powi(2)
    }

    pub fn handle_collision(&mut self, other: &mut Ball) {
        if self.check_collision(other) {
            let total_mass = self.mass + other.mass;
            let diff_pos = self.pos - other.pos;
            let diff_vel = self.vel - other.vel;
            let alpha =  diff_pos * 2.0 * (diff_vel * diff_pos) / ((diff_pos * diff_pos) * total_mass);
            self.vel -= alpha * self.mass;
            other.vel += alpha * other.mass;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_ball() {
        let _b1 = Ball::new(10.0, 3.0);
        let _b2 = Ball {
            pos: Point { x: 10.0, y: 3.0 },
            vel: Point { x: 10.0, y: 3.0 },
            ..Default::default()
        };
        assert!(_b1.pos == _b2.pos);
    }
}
