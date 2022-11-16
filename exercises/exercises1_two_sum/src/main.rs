use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut checked: HashMap<i32, i32> = HashMap::new();
    let mut result: Vec<i32> = Vec::new();
    for (i, num) in nums.iter().enumerate() {
        let r = target - num;
        if checked.contains_key(&r) {
            result = vec![i as i32, checked[&r]];
            break;
        } else {
            checked.insert(*num, i as i32);
        }
    }
    result
}

fn main() {
    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));
}
