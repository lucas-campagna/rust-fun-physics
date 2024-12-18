use ball::Ball;
mod ball;
mod math;
mod physics;

fn main() {
    let b = Ball::new(1.0, 1.0);
    println!("{:?}", b);
}
