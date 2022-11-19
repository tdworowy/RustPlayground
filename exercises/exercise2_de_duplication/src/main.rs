fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}

fn main() {
    let nums = &mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!("{}", remove_duplicates(nums));
    println!("{:?}", nums);
}
