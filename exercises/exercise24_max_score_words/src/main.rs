use std::collections::HashMap;
use std::iter::zip;
fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    let mut alphabet_map: HashMap<char, i32> = HashMap::new();
    let mut letters = letters;

    let mut words_scored_map: HashMap<String, i32> = HashMap::new();
    let mut words_scored: Vec<String> = Vec::new();

    let alph: Vec<char> = "abcdefghijklmnopqrstuvwxyz".to_string().chars().collect();

    zip(alph, score).for_each(|(l, s)| {
        alphabet_map.insert(l, s);
    });

    for word in words.clone() {
        for l in word.chars() {
            let wm = words_scored_map.entry(word.clone()).or_insert(0);
            *wm += alphabet_map[&l];
        }
    }

    let words_scored_map_read = words_scored_map.clone();

    let mut i: usize = 0;
    while i <= words.len() {
        let mut max = 0;
        let mut _word: String = "".to_string();
        for (w, s) in words_scored_map.clone() {
            if s > max {
                max = s;
                _word = w.clone();
            }
        }
        i += 1;
        words_scored.push(_word.clone());
        words_scored_map.remove(&_word);
    }
    // println!("{:?}", words_scored_map_read);
    let mut results: Vec<String> = Vec::new();
    let mut f: bool = true;
    for word in words_scored {
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
    //println!("{:?}", results);
    let mut score_sum: i32 = 0;
    results.into_iter().for_each(|w| {
        score_sum += words_scored_map_read[&w];
    });
    score_sum
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
    ); //27 <- incorrect, returns 25
    println!(
        "{:?}",
        max_score_words(
            vec!["leetcode".to_string()],
            vec!['l', 'e', 't', 'c', 'o', 'd'],
            vec![0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]
        )
    ); //0
}
