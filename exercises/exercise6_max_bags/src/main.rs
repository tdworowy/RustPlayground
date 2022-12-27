fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
    let mut full_bags_count = 0;
    let mut not_full_bags: Vec<i32> = Vec::new();
    let mut additional_rocks = additional_rocks;

    for (cap, rock) in capacity.iter().zip(rocks.iter()) {
        if cap == rock {
            full_bags_count += 1;
        } else {
            not_full_bags.push(cap - rock);
        }
    }
    not_full_bags.sort();
    for bag in not_full_bags {
        if bag <= additional_rocks {
            full_bags_count += 1;
            additional_rocks -= bag;
        } else {
            break;
        }
    }
    full_bags_count
}

fn main() {
    println!(
        "{}",
        maximum_bags(
            vec![91, 54, 63, 99, 24, 45, 78],
            vec![35, 32, 45, 98, 6, 1, 25],
            17
        )
    );
}
