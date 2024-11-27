use std::collections::{HashMap, HashSet};

fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut ocurence_mape: HashMap<i32, i32> = HashMap::new();

    arr.into_iter().for_each(|x| {
        let ocurence = ocurence_mape.entry(x).or_insert(0);
        *ocurence += 1;
    });
    let mut unique = HashSet::new();
    for v in ocurence_mape.values() {
        if !unique.insert(v) {
            return false;
        }
    }

    return true;
}

fn main() {
    println!("{:?}", unique_occurrences(vec![1, 2, 2, 1, 1, 3])); // true
    println!("{:?}", unique_occurrences(vec![1, 1, 2, 2])); // false
    println!("{:?}", unique_occurrences(vec![1, 2])); //false
}
