fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 1 {
        return vec![vec![1]];
    };
    if num_rows == 2 {
        return vec![vec![1], vec![1, 1]];
    };
    let mut p_tringle = vec![vec![1], vec![1, 1]];
    for i in 1..num_rows - 1 {
        let row = p_tringle[i as usize].clone();
        let mut new_row: Vec<i32> = Vec::new();
        for j in 0..row.len() - 1 {
            new_row.push(row[j] + row[j + 1]);
        }
        new_row.push(1);
        new_row.insert(0, 1);
        p_tringle.push(new_row);
    }
    p_tringle
}
fn main() {
    println!("{:?}", generate(1)); // [[1]]
    println!("{:?}", generate(2)); // [[1], [1,1]]
    println!("{:?}", generate(3)); // [[1], [1,1], [1,2,1]]
    println!("{:?}", generate(4)); // [[1], [1,1], [1,2,1], [1,3,3,1]]
    println!("{:?}", generate(5)); // [[1], [1,1], [1,2,1], [1,3,3,1], [1,4,6,4,1]]
    println!("{:?}", generate(20));
}
