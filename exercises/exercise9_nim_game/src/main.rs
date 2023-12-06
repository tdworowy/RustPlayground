fn can_win_nim(n: i32) -> bool {
    return n % 4 != 0;
}

fn main() {
    println!("{}", can_win_nim(2));
    println!("{}", can_win_nim(3));
    println!("{}", can_win_nim(4));
    println!("{}", can_win_nim(5));
    println!("{}", can_win_nim(6));
    println!("{}", can_win_nim(9));
}
