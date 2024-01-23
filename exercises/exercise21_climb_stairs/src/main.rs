use itertools::{Itertools, MultiProduct};

fn climb_stairs(n: i128) -> i128 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    b
}

pub fn product_repeat<I>(it: I, repeat: usize) -> MultiProduct<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    std::iter::repeat(it).take(repeat).multi_cartesian_product()
}
pub trait ProductRepeat: Iterator + Clone
where
    Self::Item: Clone,
{
    fn product_repeat(self, repeat: usize) -> MultiProduct<Self> {
        std::iter::repeat(self)
            .take(repeat)
            .multi_cartesian_product()
    }
}

impl<T: Iterator + Clone> ProductRepeat for T where T::Item: Clone {}

fn print_all_staris_combinations(n: usize) {
    let mut results: Vec<Vec<usize>> = Vec::new();
    for i in 2..n {
        for r in product_repeat(1..3, i) {
            let s: usize = r.iter().sum();
            if s == n {
                results.push(r)
            }
        }
    }
    let ones: Vec<usize> = vec![1; n];
    results.push(ones);
    results.into_iter().for_each(|x| {
        println!("{:?}", x);
    });
}

fn main() {
    // println!("{:?}", climb_stairs(2)); //2
    // println!("{:?}", climb_stairs(3)); //3
    // println!("{:?}", climb_stairs(4)); //5
    // println!("{:?}", climb_stairs(5)); //8
    // println!("{:?}", climb_stairs(183)); //127127879743834334146972278486287885163

    // print_all_staris_combinations(5);
    // print_all_staris_combinations(8);

    print_all_staris_combinations(40);
}
