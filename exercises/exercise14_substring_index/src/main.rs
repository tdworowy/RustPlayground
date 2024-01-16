fn str_str_(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(n) => return n as i32,
        None => -1,
    }
}

pub fn str_str(haystack: String, needle: String) -> i32 {
    let n_length = needle.len();
    let h_length = haystack.len();

    if needle.is_empty() {
        return 0;
    }

    if n_length > h_length {
        return -1;
    }

    for index in 0..(h_length - n_length + 1) {
        println!("{:?}, {:?}", index, index + n_length);
        if haystack[index..(index + n_length)] == needle {
            return index as i32;
        }
    }
    -1
}

fn main() {
    println!("{:?}", str_str("aaa".to_string(), "aaa".to_string())); // 0
    println!("{:?}", str_str("sadbutsad".to_string(), "sad".to_string())); // 0
    println!("{:?}", str_str("leetcode".to_string(), "leeto".to_string())); // -1
    println!("{:?}", str_str("aaaabbb".to_string(), "bb".to_string())); // 4
    println!("{:?}", str_str("abcdefg".to_string(), "g".to_string())); // 6
    println!(
        "{:?}",
        str_str("mississippi".to_string(), "issi".to_string())
    ); // 1
    println!(
        "{:?}",
        str_str("mississippi".to_string(), "issip".to_string())
    ); // 4
}
