// works until number is bigger than u128
fn plus_one_small(digits: Vec<i32>) -> Vec<i32> {
    let s = digits
        .into_iter()
        .map(|i| i.to_string())
        .collect::<String>();
    let new_number: u128 = s.parse::<u128>().unwrap() + 1;
    let new_s = format!("{:?}", new_number);

    let result: Vec<i32> = new_s
        .chars()
        .map(|s| (s.to_string()).parse::<i32>().unwrap())
        .collect();
    result
}

// don't work in case [9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9]
fn plus_one_overengineered(digits: Vec<i32>) -> Vec<i32> {
    let mut temp: Vec<i32> = Vec::new();
    for n in digits.clone().into_iter().rev().collect::<Vec<i32>>() {
        if n < 9 {
            temp.push(n);
            break;
        } else {
            temp.push(n)
        }
    }
    let s = temp
        .clone()
        .into_iter()
        .map(|i| i.to_string())
        .rev()
        .collect::<String>();

    let new_number: u128 = s.parse::<u128>().unwrap() + 1;
    let new_s = format!("{:?}", new_number);

    let mut result: Vec<i32> = new_s
        .chars()
        .map(|s| (s.to_string()).parse::<i32>().unwrap())
        .rev()
        .collect::<Vec<i32>>();

    let mut digits_: Vec<i32> = Vec::new();
    for i in 0..digits.len() - temp.len() {
        digits_.push(digits[i]);
    }
    result.reverse();

    for i in &result {
        digits_.push(*i);
    }
    digits_
}

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;

    for idx in (0..digits.len()).rev() {
        if digits[idx] == 9 {
            digits[idx] = 0;
        } else {
            digits[idx] += 1;
            return digits;
        }
    }
    digits.insert(0, 1);

    digits
}

fn main() {
    println!("{:?}", plus_one(vec![1, 2, 3])); // [1,2,4].
    println!("{:?}", plus_one(vec![4, 3, 2, 1])); // [4,3,2,2]
    println!("{:?}", plus_one(vec![9])); //[1,0]
    println!("{:?}", plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0])); //[9, 8, 7, 6, 5, 4, 3, 2, 1, 1]
    println!(
        "{:?}",
        plus_one(vec![
            7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9, 4,
            7, 0, 1, 1, 1, 7, 4, 0, 0, 6
        ])
    ); //7,2,8,5,0,9,1,2,9,5,3,6,6,7,3,2,8,4,3,7,9,5,7,7,4,7,4,9,4,7,0,1,1,1,7,4,0,0,7

    println!("{:?}", plus_one(vec![8, 9, 9, 9])); //[9,0,0,0]
}
