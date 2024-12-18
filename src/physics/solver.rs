use crate::math::Point;
use crate::ball::Ball;

enum Solver {
    Verlet {
        dt: f32,
        background_force_field: dyn Fn(balls &mut Vec<Balls>) -> Point,
    }
}

impl Solver {
    fn new_verlet(dt: f32) -> Self {
        Solver::Verlet { dt }
    }
}

impl Solver {
    fn step(&self, balls: &mut Vec<Ball>) {
        match self {
            Solver::Verlet { dt } => {
                for ball in balls {
                    let acc = self.background_force_field(ball) / ball.mass;
                    ball.pos = ball.pos + ball.vel*dt + acc*(dt*dt*0.5);
                    let new_acc = self.background_force_field(ball) / ball.mass;
                    ball.vel = ball.vel + (acc+new_acc)*(dt*0.5);
                }
            },
        };
        //self.handle_collisions(balls);
    }
    //fn handle_collisions(balls: &mut Vec<Ball>) {
    //    // 1 - get largest radius
    //    let largest_radius: f32 = balls.iter().copied().reduce(|largets, ball| ball.radius, 0.0).unwrap();
    //    // split balls into regions
    //    for ball in balls {
    //
    //    }
    //}
}

fn fast_inverse_sqrt(x: f32) -> f32 {
    const THREE_HALFS: f32 = 1.5;
    let xhalf = 0.5 * x;
    let x2 = x * xhalf;
    let y = x2 * x2 * x2 * x2 * x2;
    let y = y * (THREE_HALFS - x2 * y * y * y * y * y);
    y * xhalf
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solver(){
        let mut balls: Vec<Ball> = Vec::new();
        balls.push(Ball::new(0.0, 0.0));
        balls.push(Ball::new(0.0, 1.0));
        balls.push(Ball::new(1.0, 0.0));
        let verlet = Solver::new_verlet(0.1);
        verlet.step(&mut balls);
    }
}
