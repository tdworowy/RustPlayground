use std::collections::HashMap;

fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut missing: i32 = 0;
    let mut duplicated: i32 = 0;
    let mut numbers_map: HashMap<i32, i32> = HashMap::new();
    for i in 1..nums.len() + 1 {
        if !nums.contains(&(i as i32)) {
            missing = i as i32;
        }

        let numbers = numbers_map.entry(nums[i - 1]).or_insert(0);
        *numbers += 1;
    }
    for (k, v) in numbers_map {
        if v == 2 {
            duplicated = k as i32;
            break;
        }
    }
    vec![duplicated, missing]
}

fn main() {
    println!("{:?}", find_error_nums(vec![1, 2, 2, 4])); // [2,3]
    println!("{:?}", find_error_nums(vec![1, 1])); // [1,2]
}
