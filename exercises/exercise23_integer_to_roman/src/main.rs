use std::collections::HashMap;

fn int_to_roman(num: i32) -> String {
    let map: HashMap<i32, String> = HashMap::from([
        (1, "I".to_string()),
        (2, "II".to_string()),
        (3, "III".to_string()),
        (4, "IV".to_string()),
        (5, "V".to_string()),
        (6, "VI".to_string()),
        (7, "VII".to_string()),
        (8, "VIII".to_string()),
        (9, "IX".to_string()),
        (10, "X".to_string()),
        (20, "XX".to_string()),
        (30, "XXX".to_string()),
        (40, "XL".to_string()),
        (50, "L".to_string()),
        (60, "LX".to_string()),
        (70, "LXX".to_string()),
        (80, "LXXX".to_string()),
        (90, "XC".to_string()),
        (100, "C".to_string()),
        (200, "CC".to_string()),
        (300, "CCC".to_string()),
        (400, "CD".to_string()),
        (500, "D".to_string()),
        (600, "DC".to_string()),
        (700, "DCC".to_string()),
        (800, "DCCC".to_string()),
        (900, "CM".to_string()),
        (1000, "M".to_string()),
    ]);
    let keys: Vec<i32> = vec![
        1000, 900, 800, 700, 600, 500, 400, 300, 200, 100, 90, 80, 70, 60, 50, 40, 30, 20, 10, 9,
        8, 7, 6, 5, 4, 3, 2, 1,
    ];
    if map.contains_key(&num) {
        return map[&num].to_string();
    };
    let mut result: String = "".to_string();
    let mut num = num;
    while true {
        let mut keys_c = keys.clone();
        for i in 0..keys_c.len() {
            if num >= keys_c[i] {
                result = result + &map[&keys_c[i]];
                num -= keys_c[i];
                keys_c.pop();
                break;
            }
        }
        if num == 0 {
            break;
        }
        if map.contains_key(&num) {
            result = result + &map[&num];
            break;
        }
    }

    result
}
fn main() {
    println!("{:?}", int_to_roman(3)); // III
    println!("{:?}", int_to_roman(58)); // LVIII
    println!("{:?}", int_to_roman(1994)); // MCMXCIV
    println!("{:?}", int_to_roman(20)); // XX
    println!("{:?}", int_to_roman(3999)); // MMMCMXCIX
    println!("{:?}", int_to_roman(99999));
}

/*
1994
M
994
MCM
94
MCMXC
4
MCMXCIV
*/
