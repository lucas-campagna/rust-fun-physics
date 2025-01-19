use crate::ball::Ball;
use crate::math::point::Point;
use std::cell::RefCell;
use std::collections::HashMap;

const GRAVITY: f64 = 9.81;
type Region = Point<i64>;

pub enum Solver {
    Verlet {
        dt: f64,
        // region: Region 
    },
    
}

impl Solver {
    fn verlet_new(dt: f64) -> Self {
        Solver::Verlet { dt }
    }
}

impl Solver {
    fn step(&self, balls: &mut Vec<Ball>) {
        if balls.len() == 0 {
            return;
        }
        // handle physics
        match self {
            Solver::Verlet { dt } => {
                for ball in balls.iter_mut() {
                    let acc = gravity_force_field() / ball.mass;
                    ball.pos = ball.pos + ball.vel * (*dt) + acc * (dt * dt * 0.5);
                    ball.vel = ball.vel + (acc + acc) * (dt * 0.5);
                }
            }
        };
        // handle collisions
        // 1 - get largest radius
        let largest_radius: f64 = balls.iter().map(|ball| ball.radius).fold(0.0, f64::max);

        // split balls into regions
        let mut regions: HashMap<Region, Vec<Ball>> = HashMap::new();
        while let Some(ball) = balls.pop() {
            let p: Region = Region {
                x: (ball.pos.x / largest_radius).floor() as i64,
                y: (ball.pos.y / largest_radius).floor() as i64,
            };
            regions
                .entry(p)
                .or_insert(Vec::new())
                .push(ball);
        }

        for region in regions.keys() {
            // Iterate over each ball in the current region
            let region = region.clone();
            for ball in regions.get_mut(&region).unwrap().iter_mut() {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        regions
                            .get_mut(&Point {
                                x: region.x + dx,
                                y: region.y + dy,
                            })
                            .unwrap_or(&mut vec![])
                            .iter_mut()
                            .for_each(|other| ball.handle_collision(other));
                    }
                }
            }
        }

        // if let Some((region, balls)) = regions.remove_entry(first_region) {

        // }

        // for (region, balls) in regions.iter_mut() {
        //     // Iterate over each ball in the current region
        //     for ball in balls.iter_mut() {
        //         for dx in -1..=1 {
        //             for dy in -1..=1 {
        //                 regions
        //                     .borrow()
        //                     .get(&Point {
        //                         x: region.x + dx,
        //                         y: region.y + dy,
        //                     })
        //                     .unwrap_or(&vec![]);
        //                     // .iter_mut()
        //                 //     .for_each(|other| ball.handle_collision(other));
        //             }
        //         }
        //     }
        // }
        // getting balls back
        // for (_, region_balls) in regions.borrow_mut().iter_mut() {
        //     balls.append(region_balls);
        // }
    }
}

fn gravity_force_field() -> Point<f64> {
    Point { x: 0.0_f64, y: 1.0_f64 } * GRAVITY
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solver() {
        let mut balls = Vec::new();
        balls.push(Ball::new(0.0, 0.0));
        balls.push(Ball::new(0.0, 1.0));
        let verlet = Solver::verlet_new(0.1);
        for i in 0..10 {
            verlet.step(&mut balls);
            println!("{}: {:?}", i, balls[0].pos);
        }
    }
}
