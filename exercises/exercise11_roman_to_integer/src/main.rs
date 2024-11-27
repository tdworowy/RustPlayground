use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
    let map: HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]);
    let mut sum: i32 = 0;
    let roman_chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < roman_chars.len() {
        let ch1 = roman_chars[i];
        if i < roman_chars.len() - 1 {
            let ch2 = roman_chars[i + 1];
            if map[&ch1] < map[&ch2] {
                sum += map[&ch2] - map[&ch1];
                i += 2;
                continue;
            }
        }

        sum += map[&ch1];
        i += 1;
    }
    sum
}

fn main() {
    println!("{:?}", roman_to_int("III".to_string()));
    println!("{:?}", roman_to_int("LVIII".to_string()));
    println!("{:?}", roman_to_int("MCMXCIV".to_string()));
}
