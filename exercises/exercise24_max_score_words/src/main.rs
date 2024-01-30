use std::collections::HashMap;
use std::iter::zip;
use std::sync::mpsc::channel;
use std::thread;

fn generate_combinations_helper(prefix: &Vec<String>, remaining: &[String]) -> Vec<Vec<String>> {
    if remaining.is_empty() {
        return vec![prefix.clone()];
    }

    let mut result = Vec::new();

    for (i, element) in remaining.iter().enumerate() {
        let mut new_prefix = prefix.clone();
        new_prefix.push(element.clone());

        let new_remaining: Vec<String> = remaining[..i]
            .iter()
            .chain(remaining[i + 1..].iter())
            .cloned()
            .collect();

        let combinations = generate_combinations_helper(&new_prefix, &new_remaining);
        result.extend(combinations);
    }

    result
}

fn generate_combinations(elements: &[String]) -> Vec<Vec<String>> {
    generate_combinations_helper(&Vec::new(), elements)
}

fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let mut alphabet_map: HashMap<char, i32> = HashMap::new();
    let mut words_scored_map: HashMap<String, i32> = HashMap::new();
    let alph: Vec<char> = "abcdefghijklmnopqrstuvwxyz".to_string().chars().collect();

    zip(alph, score).for_each(|(l, s)| {
        alphabet_map.insert(l, s);
    });

    let mut words_temp = words.clone();
    words_temp.sort();
    words_temp.dedup();
    for word in words_temp {
        for l in word.chars() {
            let wm = words_scored_map.entry(word.clone()).or_insert(0);
            *wm += alphabet_map[&l];
        }
    }

    let mut score_sums: Vec<i32> = Vec::new();
    let (tx, rx) = channel();
    let words_combination: Vec<Vec<String>> = generate_combinations(&words);
    let words_combination_len = words_combination.len();

    for _words in words_combination {
        let tx = tx.clone();
        let words_scored_map = words_scored_map.clone();
        let mut letters = letters.clone();
        thread::spawn(move || {
            let mut results: Vec<String> = Vec::new();
            let mut f: bool = true;
            let mut score_sum: i32 = 0;

            for word in _words {
                for l in word.chars() {
                    if letters.contains(&l) {
                        let index = letters.iter().position(|x| *x == l).unwrap();
                        letters.remove(index);
                    } else {
                        f = false;
                        break;
                    }
                }
                if f {
                    results.push(word);
                }
            }

            results.into_iter().for_each(|w| {
                score_sum += words_scored_map[&w];
            });
            tx.send(score_sum).unwrap();
        });
    }
    for _ in 0..words_combination_len {
        score_sums.push(rx.recv().unwrap());
    }
    score_sums.into_iter().max().unwrap()
}

fn main() {
    println!(
        "{:?}",
        max_score_words(
            vec![
                "dog".to_string(),
                "cat".to_string(),
                "dad".to_string(),
                "good".to_string()
            ],
            vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
            vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    ); // 23
    println!(
        "{:?}",
        max_score_words(
            vec![
                "xxxz".to_string(),
                "ax".to_string(),
                "bx".to_string(),
                "cx".to_string()
            ],
            vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'],
            vec![4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10]
        )
    ); // 27
    println!(
        "{:?}",
        max_score_words(
            vec!["leetcode".to_string()],
            vec!['l', 'e', 't', 'c', 'o', 'd'],
            vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
        )
    ); // 0
    println!(
        "{:?}",
        max_score_words(
            vec![
                "add".to_string(),
                "dda".to_string(),
                "bb".to_string(),
                "ba".to_string(),
                "add".to_string()
            ],
            vec!['a', 'a', 'a', 'a', 'b', 'b', 'b', 'b', 'c', 'c', 'c', 'c', 'c', 'd', 'd', 'd'],
            vec![3, 9, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    ); // 51

    println!(
        "{:?}",
        max_score_words(
            vec![
                "cadedaecb".to_string(),
                "dccadce".to_string(),
                "eee".to_string(),
                "dda".to_string(),
                "dceeadd".to_string(),
                "abe".to_string(),
                "adea".to_string(),
                "aec".to_string(),
                "aecdbecbbe".to_string(),
            ],
            vec![
                'a', 'a', 'a', 'a', 'a', 'b', 'b', 'b', 'b', 'b', 'b', 'c', 'c', 'c', 'c', 'c',
                'c', 'c', 'c', 'd', 'd', 'd', 'd', 'd', 'd', 'd', 'e', 'e', 'e', 'e', 'e', 'e'
            ],
            vec![7, 1, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    ); // 86
    println!(
        "{:?}",
        max_score_words(
            vec![
                "aeaa".to_string(),
                "d".to_string(),
                "bedc".to_string(),
                "c".to_string(),
                "ccbac".to_string(),
                "eedda".to_string(),
                "aabd".to_string(),
                "abab".to_string(),
            ],
            vec![
                'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'b', 'b', 'b', 'b', 'c', 'c', 'c',
                'c', 'd', 'd', 'd', 'd', 'd', 'e', 'e'
            ],
            vec![10, 8, 8, 1, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        )
    ); // 155
}
