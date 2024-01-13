use std::{
    cmp,
    collections::{HashMap, HashSet},
    iter::zip,
};

fn count_uniq_characters(s: String) -> usize {
    let char_set: HashSet<char> = s.chars().collect();
    char_set.len()
}

fn get_missing_character(s: String) -> char {
    let alphabet: Vec<char> = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect())
        .unwrap()
        .chars()
        .collect();
    let char_set: HashSet<char> = s.chars().collect();

    let mut ch_r: char = 'a';
    zip(alphabet, char_set).for_each(|(a, ch)| {
        if a != ch {
            ch_r = a;
        };
    });
    ch_r
}

fn length_of_longest_substring1(s: String) -> i32 {
    if s.len() == 0 {
        return 0 as i32;
    }

    if s.len() == count_uniq_characters(s.clone()) {
        return s.len() as i32;
    }
    if s.len() == count_uniq_characters(s.clone()) + 1 {
        return (s.len() - 1) as i32;
    }
    let mut sub_strings: Vec<HashSet<String>> = vec![];
    let mut temp: HashSet<String> = HashSet::new();
    let mut last = s.chars().nth(0).unwrap();

    s.chars().skip(1).for_each(|ch| {
        if ch != last {
            temp.insert(last.to_string());
            last = ch;
        } else {
            temp = HashSet::new();
            last = ch;
        }
        sub_strings.push(temp.clone());
    });
    let mut max_l = 1;
    sub_strings.into_iter().for_each(|s| {
        if s.len() > max_l {
            max_l = s.len();
        }
    });

    max_l as i32
}

fn all_unq(s: &str, start: usize, end: usize) -> bool {
    let mut char_set: HashSet<char> = HashSet::new();
    let chars: Vec<char> = s.chars().collect();
    for i in start..end {
        if !char_set.insert(chars[i]) {
            return false;
        }
    }
    true
}

fn length_of_longest_substring2(s: String) -> i32 {
    let n = s.len();
    let mut l = 0;
    for i in 0..n {
        for j in i + 1..n + 1 {
            if all_unq(&s, i, j) {
                l = cmp::max(l, j - i);
            }
        }
    }
    l as i32
}

fn length_of_longest_substring3(s: String) -> i32 {
    let mut l = 0;
    let mut char_set: HashMap<char, usize> = HashMap::new();
    let mut start = 0;
    for (end, c) in s.char_indices() {
        if let Some(&n) = char_set.get(&c) {
            start = cmp::max(start, n);
        }
        l = cmp::max(l, end - start + 1);
        char_set.insert(c, end + 1);
    }
    l as i32
}

fn main() {
    println!("{}", length_of_longest_substring3("abcabcbb".to_string())); //3
    println!("{}", length_of_longest_substring3("bbbbb".to_string())); // 1
    println!("{}", length_of_longest_substring3("pwwkew".to_string())); //3
    println!("{}", length_of_longest_substring3("".to_string())); //0
    println!("{}", length_of_longest_substring3("au".to_string())); //2
    println!("{}", length_of_longest_substring3("aab".to_string())); //2
    println!("{}", length_of_longest_substring3("cdd".to_string())); //2
    println!("{}", length_of_longest_substring3("abba".to_string())); //2
}
