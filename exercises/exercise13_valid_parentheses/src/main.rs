fn is_valid(s: String) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }
    let mut open = 0;
    let mut closed = 0;
    let mut next: Vec<char> = Vec::new();
    for ch in s.chars() {
        if vec!['{', '(', '['].contains(&ch) {
            open += 1;
            next.push(match ch {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                _ => panic!(),
            });
        } else {
            closed += 1;
            match next.pop() {
                Some(n) => {
                    if ch != n {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }
    open == closed
}

fn main() {
    println!("{:?}", is_valid("()".to_string())); // true
    println!("{:?}", is_valid("()[]{}".to_string())); // true
    println!("{:?}", is_valid("(]".to_string())); // false
    println!("{:?}", is_valid("([)]".to_string())); // false
    println!("{:?}", is_valid("{[]}".to_string())); // true
    println!("{:?}", is_valid("[".to_string())); // false
    println!("{:?}", is_valid("((".to_string())); // false
    println!("{:?}", is_valid("){".to_string())); // false
}
