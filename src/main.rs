mod ball;
mod math;
mod physics;

fn main() {
    let mut a: Vec<i32> = vec![13,10,11,20,21,30,31];
    for x in  a.split_inclusive_mut(|i| {
        println!("i={:?}", i);
        i%2 == 0
    }) {
        println!("x={:?}", x);
    }
    
}
