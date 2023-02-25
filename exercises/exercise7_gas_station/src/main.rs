fn can_complete_circuit1(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    let mut current_gas: i32 = 0;
    let mut start_idx: i32 = 0;
    for i in 0..gas.len() {
        let diff = gas[i] - cost[i];
        total += diff;
        current_gas += diff;
        if current_gas < 0 {
            start_idx = i as i32 + 1;
            current_gas = 0;
        }
    }
    if total < 0 {
        -1
    } else {
        start_idx
    }
}

// faster solution
fn can_complete_circuit2(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    let mut current_gas: i32 = 0;
    let mut start_idx: i32 = 0;
    gas.iter().zip(cost).into_iter().enumerate().for_each(|(i, (g, c))| {
        let diff = g - c;
        total += diff;
        current_gas += diff;
        if current_gas < 0 {
            start_idx = i as i32 + 1;
            current_gas = 0;
        }
    });

    if total < 0 {
        -1
    } else {
        start_idx
    }
}

fn main() {
    println!(
        "{}",
        can_complete_circuit1(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
    ); // 3
    println!("{}", can_complete_circuit1(vec![2, 3, 4], vec![3, 4, 3])); // -1

    println!(
        "{}",
        can_complete_circuit2(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2])
    ); // 3
    println!("{}", can_complete_circuit2(vec![2, 3, 4], vec![3, 4, 3])); // -1
}
