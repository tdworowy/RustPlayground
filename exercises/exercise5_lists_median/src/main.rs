fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums = [nums1, nums2].concat();
    nums.sort();
    let center = nums.len() / 2;
    if nums.len() % 2 != 0 {
        return nums[center] as f64;
    } else {
        return (nums[center - 1] + nums[center]) as f64 / 2 as f64;
    }
}

fn main() {
    println!("{}", find_median_sorted_arrays(vec![1, 2], vec![3]));
    println!("{}", find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
    println!("{}", find_median_sorted_arrays(vec![0, 0], vec![0, 0]));
}
