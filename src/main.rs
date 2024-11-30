use ball::Ball;
mod ball;
mod math;

fn main() {
    let b = Ball::new(1.0, 1.0, None);
    println!("{:?}", b);
}
