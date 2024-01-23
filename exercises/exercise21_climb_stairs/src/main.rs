fn climb_stairs(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    b
}

fn main() {
    println!("{:?}", climb_stairs(2)); //2
    println!("{:?}", climb_stairs(3)); //3
    println!("{:?}", climb_stairs(4)); //5
    println!("{:?}", climb_stairs(5)); //8
}
