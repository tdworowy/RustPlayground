use std::iter::zip;

fn add_binary(a: String, b: String) -> String {
    let mut a = a;
    let mut b = b;

    if a.len() > b.len() {
        while b.len() < a.len() {
            b = format!("{}{}", "0", b);
        }
    };

    if b.len() > a.len() {
        while a.len() < b.len() {
            a = format!("{}{}", "0", a);
        }
    };
    let mut r: usize;
    let mut result: String = "".to_string();
    let mut carry: usize = 0;
    for (d1, d2) in zip(a.chars().rev(), b.chars().rev()) {
        r = carry;
        if d1 == '1' {
            r += 1
        } else {
            r += 0
        };
        if d2 == '1' {
            r += 1
        } else {
            r += 0
        };

        if r % 2 == 1 {
            result = format!("{}{}", "1", result);
        } else {
            result = format!("{}{}", "0", result);
        }

        if r < 2 {
            carry = 0;
        } else {
            carry = 1
        }
    }
    if carry != 0 {
        result = format!("{}{}", "1", result);
    }
    result
}

fn main() {
    println!("{:?}", add_binary("11".to_string(), "1".to_string())); // 100
    println!("{:?}", add_binary("1010".to_string(), "1011".to_string())); // 10101
}

// 1010
// 1011
// 10101

// 11
// 01
// 100
