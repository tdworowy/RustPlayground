use std::cmp::Ordering;

fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut start: i32 = 0;
    let mut end: i32 = (nums.len() -1) as i32;

    while start <= end {
        let mid = start + (end - start) / 2;

        match nums[mid as usize].cmp(&target) {
            Ordering::Equal => {
                return mid;
            }
            Ordering::Greater => {
                end = mid - 1;
            }
            Ordering::Less => {
                start = mid + 1;
            }
        }
    }
    start
}

fn main() {
    println!("{}", search_insert(vec![1, 3, 5, 6], 5));
    println!("{}", search_insert(vec![1, 3, 5, 6], 2));
    println!("{}", search_insert(vec![1, 3, 5, 6], 7));
}
