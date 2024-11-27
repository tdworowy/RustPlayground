fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    {
        let mut last: char = colors.chars().nth(0).unwrap();

        let mut res_time: i32 = 0;
        let mut max_time: i32 = 0;

        for (i, color) in colors.chars().enumerate() {
            if color != last {
                res_time -= max_time;
                max_time = 0;
                last = color;
            }

            res_time += needed_time[i];
            max_time = max_time.max(needed_time[i]);
        }

        res_time - max_time
    }
}

fn main() {
    let time1: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", min_cost("abaac".to_string(), time1));

    let time2: Vec<i32> = vec![1, 2, 3, 4, 1];
    println!("{:?}", min_cost("aabaa".to_string(), time2));

    let time3: Vec<i32> = vec![3, 5, 10, 7, 5, 3, 5, 5, 4, 8, 1];
    println!("{:?}", min_cost("aaabbbabbbb".to_string(), time3));
}
