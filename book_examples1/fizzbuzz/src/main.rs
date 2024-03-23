use std::iter::{once, repeat};

fn fizz_buzz(max: usize) -> impl Iterator<Item = String> {
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);
    (1..max).zip(fizzes_buzzes).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    })
}
fn main() {
    let fb: Vec<String> = fizz_buzz(20).collect();
    println!("{:?}", fb);
}
