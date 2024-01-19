fn length_of_last_word(s: String) -> i32 {
    s.split(" ").filter(|s| !s.is_empty()).last().unwrap().len() as i32
}

fn main() {
    println!("{:?}", length_of_last_word("Hello World".to_string())); // 5
    println!(
        "{:?}",
        length_of_last_word("   fly me   to   the moon  ".to_string())
    ); // 4
    println!(
        "{:?}",
        length_of_last_word("luffy is still joyboy".to_string())
    ); //6
}
