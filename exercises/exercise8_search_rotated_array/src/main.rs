fn find_pivot(nums: &Vec<i32>, low: i32, high: i32) -> i32 {
    if high < low {
        return -1;
    } else if high == low {
        return low as i32;
    } else {
        let mid: i32 = (low + high) / 2;
        if mid < high && nums[mid as usize] > nums[(mid + 1) as usize] {
            return mid as i32;
        }
        if mid > low && nums[mid as usize] < nums[(mid - 1) as usize] {
            return (mid - 1) as i32;
        }
        if nums[low as usize] >= nums[mid as usize] {
            return find_pivot(nums, low, mid - 1);
        }
        return find_pivot(nums, mid + 1, high);
    }
}

fn binary_search(nums: &Vec<i32>, low: usize, high: usize, target: i32) -> i32 {
    if high < low {
        return -1;
    }
    let half: usize = (low + high) / 2;
    if target == nums[half] {
        return half as i32;
    }
    if target > nums[half] {
        return binary_search(nums, half + 1, high, target);
    }
    return binary_search(nums, low, half - 1, target);
}

fn search(nums: Vec<i32>, target: i32) -> i32 {
    let len: usize = nums.len();
    if len < 4 {
        if nums.contains(&target) {
            return nums.iter().position(|&r| r == target).unwrap() as i32;
        } else {
            return -1;
        }
    } // ?
    let pivot = find_pivot(&nums, 0, len as i32 - 1);

    if pivot == -1 {
        return binary_search(&nums, 0, len - 1, target);
    }
    if nums[pivot as usize] == target {
        return pivot;
    }
    if nums[0] <= target {
        return binary_search(&nums, 0, (pivot - 1) as usize, target);
    }

    return binary_search(&nums, (pivot + 1) as usize, len - 1, target);
}

fn main() {
    println!("{}", search(vec![4, 5, 6, 7, 0, 1, 2], 0));
    println!("{}", search(vec![4, 5, 6, 7, 0, 1, 2], 3));
    println!("{}", search(vec![1], 0));
    println!("{}", search(vec![1], 2));
    println!("{}", search(vec![1, 3], 2));
    println!("{}", search(vec![1, 3], 1));
}
