fn my_sqrt(x: i32) -> i32 {
    let n = 20;
    let mut y: f64 = 1.0;
    let mut y_last: f64 = 1.0;
    for _ in 0..n {
        y = 1.0 / 2.0 * (y + x as f64 / y);
        if y == y_last {
            break;
        }
        y_last = y;
    }
    y.floor() as i32
}

fn main() {
    println!("{:?}", my_sqrt(4)); //2
    println!("{:?}", my_sqrt(8)); //2
    println!("{:?}", my_sqrt(1024)); //32
    println!("{:?}", my_sqrt(2147395599)); //46339
}
